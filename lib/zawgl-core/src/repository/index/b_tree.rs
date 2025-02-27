// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::buf_config::NB_CELL;

use super::store::*;
use super::model::*;

pub type DataPtr = u64;
pub struct BTreeIndex {
    node_store: BTreeNodeStore,
}

fn get_node_ptr(not_found_index: usize, node: &BTreeNode) -> Option<NodeId> {
    let mut node_ptr = node.get_node_ptr();
    if not_found_index != 0 {
        node_ptr = Some(node.get_cell_ref(not_found_index - 1).get_node_ptr()?);
    }
    node_ptr
}

fn binary_search_keys(keys: &[&Key], value: &Key) -> Result<usize, usize> {
    keys.binary_search_by(|&probe|{
        Ord::cmp(&probe, &value)
    })
}

impl BTreeIndex {
    pub fn new(file: &str) -> Self {
        BTreeIndex{node_store: BTreeNodeStore::new(file)}
    }

    fn tree_search(&mut self, value: &Key, node_id: &NodeId) -> Option<Vec<DataPtr>> {
        self.node_store.retrieve_node(node_id)?;
        let node = self.node_store.get_node_ref(node_id)?;
        let keys = node.get_keys();
        let res = binary_search_keys(&keys, value);
        match res {
            Ok(found) => {
                if node.is_leaf() {
                    Some(node.get_cell_ref(found).get_data_ptrs_ref().clone())
                } else {
                    let child = node.get_cell_ref(found).get_node_ptr()?;
                    self.tree_search(value, &child)
                }
            },
            Err(not_found) => {
                if node.is_leaf() {
                    None
                } else {
                    let node_ptr = get_node_ptr(not_found, node)?;
                    self.tree_search(value, &node_ptr)
                }
            }
        }
    }

    fn get_keys(&mut self, node_id: &NodeId, keys: &mut Vec<Key>) -> Option<()> {
        self.node_store.retrieve_node(node_id)?;
        let node = self.node_store.get_node_ref(node_id)?;
        let is_leaf = node.is_leaf();
        let cells = node.get_cells_ref().clone();
        if is_leaf {
            for cell in cells {
                keys.push(cell.get_key().clone());
            }
        } else {
            for cell in cells {
                let child_id = cell.get_node_ptr();
                if let Some(id) = child_id {
                    self.get_keys(&id, keys)?;
                }
            }
        }
        Some(())
    }

    pub fn retrieve_keys(&mut self) -> Option<Vec<Key>> {
        let root = self.node_store.load_or_create_root_node()?;
        let mut keys = vec![];
        self.get_keys(&root, &mut keys)?;
        Some(keys)
    }

    fn split_leaf_node(&mut self, value: &Key, data_ptr: u64, node_id: &NodeId, new_cell_index: usize) -> Option<BTreeNode> {
        let new_node_cells = {
            let node = self.node_store.get_node_mut(node_id)?;
            node.insert_cell(new_cell_index, Cell::new_leaf(value.clone(), data_ptr));
            let split = node.len() / 2;
            let mut new_node_cells = Vec::new();
            while node.len() > split {
                new_node_cells.push(node.pop_cell().unwrap());
            }
            new_node_cells.reverse();
            new_node_cells
        };
        self.node_store.save(node_id)?;
        let new_first_cell_key = new_node_cells[0].get_key().clone();
        let mut new = BTreeNode::new(true, false, new_node_cells);

        let node_ptr = self.node_store.get_node_ptr(node_id);
        if node_ptr.is_some() {
            let ptr = node_ptr?;
            self.node_store.retrieve_node(&ptr)?;
            new.set_node_ptr(node_ptr);
        }
        
        self.node_store.create(&mut new)?;
        
        self.node_store.set_node_ptr(node_id, new.get_id());
        self.node_store.save(node_id)?;
        
        if self.node_store.is_root_node(node_id) {
            self.node_store.set_is_root(node_id, false);
            self.node_store.save(node_id)?;
            let middle_key = Cell::new_ptr(new_first_cell_key, new.get_id());
            let mut new_root = BTreeNode::new(false, true, vec![middle_key]);
            new_root.set_node_ptr(Some(*node_id));
            self.node_store.create(&mut new_root)?;
            None
        } else {
            Some(new)
        }
    }

    fn split_interior_node(&mut self, new_key: &Key, chid_id: Option<NodeId>, new_split_ptr: Option<NodeId>, node_id: &NodeId, new_cell_index: usize) -> Option<BTreeNode> {
        let new_node_cells = {
            let node = self.node_store.get_node_mut(node_id)?;
            node.insert_cell(new_cell_index, Cell::new_ptr(new_key.clone(), new_split_ptr));
            let split = node.len() / 2;
            let mut new_node_cells = Vec::new();
            while node.len() > split {
                new_node_cells.push(node.pop_cell().unwrap());
            }
            new_node_cells.reverse();
            new_node_cells
        };
        self.node_store.save(node_id)?;
        let new_first_cell_key = new_node_cells[0].get_key().clone();
        let mut new = BTreeNode::new(false, false, new_node_cells);
        new.set_node_ptr(chid_id);
        self.node_store.create(&mut new);

        if self.node_store.is_root_node(node_id) {
            self.node_store.set_is_root(node_id, false);
            self.node_store.save(node_id)?;
            let middle_key = Cell::new_ptr(new_first_cell_key, new.get_id());
            let mut new_root = BTreeNode::new(false, true, vec![middle_key]);
            new_root.set_node_ptr(Some(*node_id));
            self.node_store.create(&mut new_root)?;
            None
        } else {
            Some(new)
        }
    }

    fn insert_or_update_key_ptrs(&mut self, value: &Key, data_ptr: u64, node_id: &NodeId) -> Option<BTreeNode> {
        self.node_store.retrieve_node(node_id)?;
        let (search, keys) = {
            let node = self.node_store.get_node_ref(node_id)?;
            let keys = node.get_keys();
            let res = binary_search_keys(&keys, value);
            (res, keys.iter().map(|key| (*key).clone()).collect::<Vec<Key>>())
        };
        match search {
            Ok(found) => {
                let node = self.node_store.get_node_mut(node_id)?;
                if node.is_leaf() {
                    node.get_cell_mut(found).append_data_ptr(data_ptr);
                    self.node_store.save(node_id)?;
                    None
                } else {
                    let child_id = node.get_cell_ref(found).get_node_ptr()?;
                    self.insert_or_update_key_ptrs(value, data_ptr, &child_id)
                }
            },
            Err(not_found) => {
                if self.node_store.is_leaf_node(node_id)? {
                    let node = self.node_store.get_node_mut(node_id)?;
                    if node.is_full() {
                        self.split_leaf_node(value, data_ptr, node_id, not_found)
                    } else {
                        node.insert_cell(not_found, Cell::new_leaf(value.clone(), data_ptr));
                        self.node_store.save(node_id)?;
                        None
                    }
                } else {
                    let child_id = {
                        let node = self.node_store.get_node_ref(node_id)?;
                        get_node_ptr(not_found, node)
                    };
                    if let Some(child_id) = child_id {
                        let split = self.insert_or_update_key_ptrs(value, data_ptr, &child_id)?;
                        let split_key = split.get_cell_ref(0).get_key();
                        let search = binary_search_keys(&keys.iter().map(|key| key).collect::<Vec<&Key>>(), &split_key);

                        let node = self.node_store.get_node_ref(node_id)?;
                        if node.is_full() {
                            match search {
                                Ok(found_split) => {
                                    let node = self.node_store.get_node_mut(node_id)?;
                                    node.get_cell_mut(found_split).set_node_ptr(split.get_id());
                                    self.node_store.save(node_id)?;
                                    None
                                },
                                Err(not_found_split) => {
                                    self.split_interior_node(&split_key, Some(child_id), split.get_id(), node_id, not_found_split)
                                },
                            }
                            
                        } else {
                            match search {
                                Ok(found_split) => {
                                    let node = self.node_store.get_node_mut(node_id)?;
                                    node.get_cell_mut(found_split).set_node_ptr(split.get_id());
                                    self.node_store.save(node_id)?;
                                },
                                Err(not_found_split) => {
                                    let node = self.node_store.get_node_mut(node_id)?;
                                    node.insert_cell(not_found_split, Cell::new_ptr(split_key.clone(), split.get_id()));
                                    self.node_store.save(node_id)?;
                                },
                            }
                            None
                        }
                    } else {
                        None
                    }
                }
            }
        }
    }

    pub fn search(&mut self, value: &Key) -> Option<Vec<DataPtr>> {
        let root = self.node_store.load_or_create_root_node()?;
        self.tree_search(value, &root)
    }

    pub fn insert(&mut self, value: &Key, data_ptr: u64) -> Option<()> {
        let root = self.node_store.load_or_create_root_node()?;
        self.insert_or_update_key_ptrs(value, data_ptr, &root).map(|_node|())
    }

    fn drop_key(&mut self, value: &Key, node_id: &NodeId) -> Option<bool> {
        self.node_store.retrieve_node(node_id)?;
        let search_res = {
            let node = self.node_store.get_node_ref(node_id)?;
            let keys = node.get_keys();
            let res = binary_search_keys(&keys, value);
            res
        };
        match search_res {
            Ok(found) => {
                let (ochild_id, is_leaf) = {
                    let node = self.node_store.get_node_ref(node_id)?;
                    (node.get_cell_ref(found).get_node_ptr(), node.is_leaf())
                };

                if !is_leaf {
                    let child_id = ochild_id.unwrap();
                    let droped = self.drop_key(value, &child_id)?;
                    let node = self.node_store.get_node_mut(node_id)?;
                    node.remove_cell(found);
                    self.node_store.save(node_id)?;


                    if droped {
                        let (child_node_len, child_node_half_full) = {
                            let child_node = self.node_store.get_node_ref(&child_id)?;
                            (child_node.len(), child_node.is_not_half_full())
                        };
                        if child_node_half_full {
                            let sibling_node_id = {
                                let current_node = self.node_store.get_node_ref(node_id)?;
                                if found > 0 {
                                    current_node.get_cell_ref(found - 1).get_node_ptr()
                                } else {
                                    current_node.get_node_ptr()
                                }
                            };
                            if let Some(sibling_node_id) = sibling_node_id {
                                let merge = {
                                    let sibling = self.node_store.get_node_ref(&sibling_node_id)?;
                                    sibling.len() + child_node_len <= NB_CELL
                                };
    
                                if merge {
                                    let (child_node_cells, next_ptr) = {
                                        self.node_store.retrieve_node(&child_id)?;
                                        let child_node = self.node_store.get_node_mut(&child_id)?;
                                        let cells = child_node.get_cells_ref().clone();
                                        child_node.remove_cells();
                                        let child_node_ptr = child_node.get_node_ptr();
                                        self.node_store.save(&child_id)?;
                                        (cells, child_node_ptr)
                                    };
                                    self.node_store.retrieve_node(&sibling_node_id)?;
                                    let sibling_node = self.node_store.get_node_mut(&sibling_node_id)?;
                                    for cell in child_node_cells {
                                        sibling_node.append_cell(cell);
                                    }
                                    if sibling_node.is_leaf() {
                                        sibling_node.set_node_ptr(next_ptr);
                                    }
                                    self.node_store.save(&sibling_node_id)?;
                                }
                            }
                        }
                    }
                    Some(true)
                } else {
                    let node = self.node_store.get_node_mut(node_id)?;
                    node.remove_cell(found);
                    self.node_store.save(node_id)?;
                    Some(true)
                }
            },
            Err(not_found) => {
                let (ochild_id, is_leaf) = {
                    let node = self.node_store.get_node_ref(node_id)?;
                    (get_node_ptr(not_found, node), node.is_leaf())
                };
                if !is_leaf {
                    let child_id = &ochild_id.unwrap();
                    let droped = self.drop_key(value, &child_id)?;
                    let current_node = self.node_store.get_node_ref(node_id)?;
                    if droped {
                        let (child_node_len, child_node_half_full) = {
                            let child_node = self.node_store.get_node_ref(&child_id)?;
                            (child_node.len(), child_node.is_not_half_full())
                        };
                        if child_node_half_full {
                            let sibling_node_id = if not_found > 1 {
                                current_node.get_cell_ref(not_found - 2).get_node_ptr()
                            } else if not_found == 1 {
                                current_node.get_node_ptr()
                            } else {
                                None
                            };
                            if let Some(sibling_node_id) = sibling_node_id {
                                let merge = {
                                    let sibling = self.node_store.get_node_ref(&sibling_node_id)?;
                                    sibling.len() + child_node_len <= NB_CELL
                                };
                                if merge {
                                    let (child_node_cells, next_ptr) = {
                                        self.node_store.retrieve_node(&child_id)?;
                                        let child_node = self.node_store.get_node_mut(&child_id)?;
                                        let cells = child_node.get_cells_ref().clone();
                                        child_node.remove_cells();
                                        let child_node_ptr = child_node.get_node_ptr();
                                        self.node_store.save(&child_id)?;
                                        (cells, child_node_ptr)
                                    };
                                    self.node_store.retrieve_node(&sibling_node_id)?;
                                    let sibling_node = self.node_store.get_node_mut(&sibling_node_id)?;
                                    for cell in child_node_cells {
                                        sibling_node.append_cell(cell);
                                    }
                                    if sibling_node.is_leaf() {
                                        sibling_node.set_node_ptr(next_ptr);
                                    }
                                    self.node_store.save(&sibling_node_id)?;
                                    let node = self.node_store.get_node_mut(node_id)?;
                                    node.remove_cell(not_found);
                                    self.node_store.save(node_id)?;
                                }
                            } else {
                                let merge = {
                                    let child_node = self.node_store.get_node_ref(child_id)?;
                                    let sibling_node = self.node_store.get_node_ref(&child_node.get_node_ptr().unwrap())?;
                                    sibling_node.len() + child_node_len <= NB_CELL
                                };
                                if merge {
                                    let (child_node_cells, next_ptr) = {
                                        self.node_store.retrieve_node(&child_id)?;
                                        let child_node = self.node_store.get_node_mut(&child_id)?;
                                        let cells = child_node.get_cells_ref().clone();
                                        child_node.remove_cells();
                                        let child_node_ptr = child_node.get_node_ptr().unwrap();
                                        self.node_store.save(&child_id)?;
                                        (cells, child_node_ptr)
                                    };
                                    self.node_store.retrieve_node(&next_ptr)?;
                                    let sibling_node = self.node_store.get_node_mut(&next_ptr)?;
                                    for cell in child_node_cells {
                                        sibling_node.append_cell(cell);
                                    }
                                    self.node_store.save(&next_ptr)?;
                                    self.node_store.set_node_ptr(node_id, Some(next_ptr))?;
                                }
                            }
                        }
                    }
                    Some(droped)
                } else {
                    Some(false)
                }
            }
        }
    }

    pub fn delete(&mut self, value: &Key) -> Option<bool> {
        let root = self.node_store.load_or_create_root_node()?;
        self.drop_key(value, &root)
    }

    pub fn sync(&mut self) {
        self.node_store.sync();
    }
    
    pub fn soft_sync(&mut self) {
        self.node_store.soft_sync();
    }
    pub fn clear(&mut self) {
        self.node_store.clear();
    }
    pub fn reset(&mut self) {
        self.node_store.erase();
        self.clear();
    }
}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    use super::super::super::super::test_utils::*;
    #[test]
    fn test_insert() {
        let file = build_file_path_and_rm_old("b_tree", "test_insert.db").unwrap();
        let mut index = BTreeIndex::new(&file);
        let key = Key::from_str("a short key");
        index.insert(&key, 42);
        let long_key = Key::from_str("a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key ");
        index.insert(&long_key, 87968567);


        let data_ptrs = index.search(&key);

        if let Some(ptrs) = &data_ptrs {
            assert_eq!(ptrs.len(), 1);
            assert_eq!(ptrs[0], 42);
        } else {
            panic!("should not happen");
        }

        index.sync();

        index.insert(&key, 56);

        let data_ptrs_1 = index.search(&key);

        if let Some(ptrs) = &data_ptrs_1 {
            assert_eq!(ptrs.len(), 2);
            assert!(ptrs.contains(&42));
            assert!(ptrs.contains(&56));
        } else {
            panic!("should not happen");
        }

        let data_ptrs_2 = index.search(&long_key).unwrap();
        assert_eq!(data_ptrs_2.len(), 1);
        assert!(data_ptrs_2.contains(&87968567));
    }

    #[test]
    fn test_root_split() {
        let file = build_file_path_and_rm_old("b_tree", "test_root_split.db").unwrap();
        let mut index = BTreeIndex::new(&file);
        let len = 2500;
        for i in 0..len {
            index.insert(&Key::from_str(&format!("key # {}", i)), i as u64);
            //println!("insert {} values", i);
        }

        //index.soft_sync();

        for i in 0..len {
            let optrs = index.search(&Key::from_str(&format!("key # {}", i)));
            if let Some(ptrs) = optrs {
                assert_eq!(ptrs.len(), 1);
                assert!(ptrs.contains(&(i as u64)));
            } else {
                panic!("empty search result for key # {}", i);
            }
        }

        for i in 0..len {
            let droped = index.delete(&Key::from_str(&format!("key # {}", i))).unwrap();
            if i % 1 == 0 {
                println!("droped {} values", i);
            }
        }

        let keys_set = index.retrieve_keys();
        assert_eq!(keys_set.map(|keys| keys.len()), Some(0));

    }

    
    #[test]
    fn test_root_split_same_key() {
        let file = build_file_path_and_rm_old("b_tree", "test_root_split_same_key.db").unwrap();
        let mut index = BTreeIndex::new(&file);

        for i in 0..3000 {
            index.insert(&Key::from_str("same key"), i);
            if i % 1000 == 0 {
                println!("inserted {} values", i);
            }
        }
        //index.soft_sync();


        let optrs = index.search(&Key::from_str("same key"));
        if let Some(ptrs) = optrs {
            assert_eq!(ptrs.len(), 3000);
        } else {
            panic!("empty search result for same key");
        }
    }

    #[test]
    fn test_same_key_many_search() {

        let file = build_file_path_and_rm_old("b_tree", "test_same_key_many_search.db").unwrap();
        let mut index = BTreeIndex::new(&file);
        let mut count = 0;
        for i in 0..10000 {
            index.insert(&Key::from_str("same key"), count);
            count += 1;
            index.insert(&Key::from_str("same key"), count);
            count += 1;
            index.insert(&Key::from_str("same key"), count);
            count += 1;
            if i % 1000 == 0 {
                println!("inserted {} values", i);
                index.sync();
                index.sync();
                index.sync();
                index = BTreeIndex::new(&file);
            }
            let optrs = index.search(&Key::from_str("same key"));
            if let Some(ptrs) = optrs {
                assert_eq!(ptrs.len(), 3*(i+1) as usize);
                let mut check = 0;
                for p in ptrs {
                    assert_eq!(p, check);
                    check += 1;
                }
            } else {
                panic!("empty search result for same key");
            }
        }
    }
}