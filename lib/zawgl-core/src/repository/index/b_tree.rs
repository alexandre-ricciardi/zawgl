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
    let mut node_ptr = node.get_node_ptr()?;
    if not_found_index != 0 {
        node_ptr = node.get_cell_ref(not_found_index - 1).get_node_ptr()?;
    }
    Some(node_ptr)
}

fn binary_search_keys(keys: &[&str], value: &str) -> Result<usize, usize> {
    keys.binary_search_by(|&probe|{
        Ord::cmp(&probe.len(), &value.len()).then(probe.cmp(value))
    })
}

impl BTreeIndex {
    pub fn new(file: &str) -> Self {
        BTreeIndex{node_store: BTreeNodeStore::new(file)}
    }

    fn tree_search(&mut self, value: &str, node_id: &NodeId) -> Option<Vec<DataPtr>> {
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


    fn split_leaf_node(&mut self, value: &str, data_ptr: u64, node_id: &NodeId, new_cell_index: usize) -> Option<BTreeNode> {
        {   
            let node = self.node_store.get_node_mut(node_id)?;
            node.insert_cell(new_cell_index, Cell::new_leaf(value, data_ptr));
        }
        let new_node_cells = {
            let node = self.node_store.get_node_mut(node_id)?;
            let split = node.get_cells_ref().len() / 2;
            let mut new_node_cells = Vec::new();
            while node.get_cells_ref().len() > split {
                if let Some(cell) = node.pop_cell() {
                    new_node_cells.push(cell);
                }
            }
            new_node_cells.reverse();
            new_node_cells
        };
        let new_first_cell = &new_node_cells[0];
        let mut middle_key = Cell::new_ptr(new_first_cell.get_key(), None);
        let mut new = BTreeNode::new(true, false, new_node_cells);

        let node_ptr = self.node_store.get_node_ptr(node_id);
        if node_ptr.is_some() {
            let ptr = node_ptr?;
            self.node_store.retrieve_node(&ptr)?;
            new.set_node_ptr(node_ptr);
        }
        
        self.node_store.create(&mut new)?;
        
        self.node_store.set_node_ptr(node_id, new.get_id());
        
        if self.node_store.is_root_node(node_id) {
            self.node_store.set_is_root(node_id, false);
            self.node_store.save(node_id)?;
            middle_key.set_node_ptr(new.get_id());
            let mut new_root = BTreeNode::new(false, true, vec![middle_key]);
            new_root.set_node_ptr(Some(*node_id));
            self.node_store.create(&mut new_root)?;
            None
        } else {
            self.node_store.save(node_id)?;
            Some(new)
        }
    }

    fn split_interior_node(&mut self, new_key: &str, new_node_ptr: Option<NodeId>, node_id: &NodeId, new_cell_index: usize) -> Option<BTreeNode> {
        
        
        let new_node_cells = {
            let node = self.node_store.get_node_mut(node_id)?;
            node.insert_cell(new_cell_index, Cell::new_ptr(new_key, new_node_ptr));
            let split = node.get_cells_ref().len() / 2;
            let mut new_node_cells = Vec::new();
            while node.get_cells_ref().len() > split {
                if let Some(cell) = node.pop_cell() {
                    new_node_cells.push(cell);
                }
            }
            new_node_cells.reverse();
            new_node_cells
        };
        let new_first_cell = &new_node_cells[0];
        let mut middle_key = Cell::new_ptr(new_first_cell.get_key(), None);
        let mut new = BTreeNode::new(false, false, new_node_cells);
        self.node_store.create(&mut new);

        if self.node_store.is_root_node(node_id) {
            self.node_store.set_is_root(node_id, false);
            self.node_store.save(node_id)?;
            middle_key.set_node_ptr(Some(*node_id));
            let mut new_root = BTreeNode::new(false, true, vec![middle_key]);
            new_root.set_node_ptr(Some(*node_id));
            self.node_store.create(&mut new_root)?;
            None
        } else {
            self.node_store.save(node_id)?;
            Some(new)
        }
    }

    fn insert_or_update_key_ptrs(&mut self, value: &str, data_ptr: u64, node_id: &NodeId) -> Option<BTreeNode> {
        self.node_store.retrieve_node(node_id)?;
        let search_res = {
            let node = self.node_store.get_node_ref(node_id)?;
            let keys = node.get_keys();
            let res = binary_search_keys(&keys, value);
            (res, keys.iter().map(|key| key.to_string()).collect::<Vec<String>>())
        };
        match search_res.0 {
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
                if self.node_store.is_leaf_node(node_id) {
                    let node = self.node_store.get_node_mut(node_id)?;
                    if node.is_full() {
                        self.split_leaf_node(value, data_ptr, node_id, not_found)
                    } else {
                        node.insert_cell(not_found, Cell::new_leaf(value, data_ptr));
                        self.node_store.save(node_id)?;
                        None
                    }
                } else {
                    let node_ptr = {
                        let node = self.node_store.get_node_ref(node_id)?;
                        get_node_ptr(not_found, node)?
                    };
                    self.node_store.retrieve_node(&node_ptr)?;
                    let split_node = self.insert_or_update_key_ptrs(value, data_ptr, &node_ptr)?;
                    let first_cell = split_node.get_cell_ref(0);
                    let first_cell_key = first_cell.get_key();
                    let keys = search_res.1.iter().map(|key| key.as_str()).collect::<Vec<&str>>();
                    let first_split_cell_key_search = binary_search_keys(&keys, first_cell_key);
                    match first_split_cell_key_search {
                        Err(not_found) => {
                            if self.node_store.is_full_node(node_id) {
                                self.split_interior_node(first_cell.get_key(), split_node.get_id(), node_id, not_found)
                            } else {
                                let node = self.node_store.get_node_mut(node_id)?;
                                node.insert_cell(not_found, Cell::new_ptr(first_cell.get_key(), split_node.get_id()));
                                self.node_store.save(&node_ptr)?;
                                None
                            }
                        },
                        _ => {None}
                    }
                    
                }
            }
        }
    }

    pub fn search(&mut self, value: &str) -> Option<Vec<DataPtr>> {
        let root = self.node_store.load_or_create_root_node()?;
        self.tree_search(value, &root)
    }

    pub fn insert(&mut self, value: &str, data_ptr: u64) -> Option<()> {
        let root = self.node_store.load_or_create_root_node()?;
        self.insert_or_update_key_ptrs(value, data_ptr, &root).map(|_node|())
    }

    fn drop_key(&mut self, value: &str, node_id: &NodeId) -> Option<usize> {
        self.node_store.retrieve_node(node_id)?;
        let (search_res, keys) = {
            let node = self.node_store.get_node_ref(node_id)?;
            let keys = node.get_keys();
            let res = binary_search_keys(&keys, value);
            (res, keys.iter().map(|key| key.to_string()).collect::<Vec<String>>())
        };
        match search_res {
            Ok(found) => {
                let child_id = {
                    let node = self.node_store.get_node_ref(node_id)?;
                    node.get_cell_ref(found).get_node_ptr()?
                };
                let target_node_cell = self.drop_key(value, &child_id)?;
                let node = self.node_store.get_node_mut(node_id)?;
                node.remove_cell(found);
                self.node_store.save(node_id)?;
                let current_node = self.node_store.get_node_ref(node_id)?;
                if !current_node.is_leaf() {
                    let (child_node_len, child_node_half_full) = {
                        let child_node = self.node_store.get_node_ref(&child_id)?;
                        (child_node.len(), child_node.is_half_full())
                    };
                    if !child_node_half_full {
                        if found > 0 {
                            let sibling_node_id = current_node.get_cell_ref(found - 1).get_node_ptr()?;
                            let child_node_cells = {
                                let child_node = self.node_store.get_node_mut(&child_id)?;
                                let cells = child_node.get_cells_ref().clone();
                                while let Some(_cell) = child_node.pop_cell() {}
                                cells
                            };
                            let sibling_node = self.node_store.get_node_mut(&sibling_node_id)?;
                            if sibling_node.len() + child_node_len <= NB_CELL {
                                for cell in child_node_cells {
                                    sibling_node.append_cell(cell.clone());
                                }
                            }
                        }
                    }
                }

                
                Some(found)
            },
            Err(_not_found) => {
                None
            }
        }
    }

    pub fn delete(&mut self, value: &str) -> Option<()> {
        let root = self.node_store.load_or_create_root_node()?;
        self.drop_key(value, &root).map(|_node|())
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
        let key = "a short key";
        index.insert(key, 42);
        let long_key = "a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key a long key ";
        index.insert(long_key, 87968567);


        let data_ptrs = index.search(key);

        if let Some(ptrs) = &data_ptrs {
            assert_eq!(ptrs.len(), 1);
            assert_eq!(ptrs[0], 42);
        } else {
            panic!("should not happen");
        }

        index.sync();

        index.insert(key, 56);

        let data_ptrs_1 = index.search(key);

        if let Some(ptrs) = &data_ptrs_1 {
            assert_eq!(ptrs.len(), 2);
            assert!(ptrs.contains(&42));
            assert!(ptrs.contains(&56));
        } else {
            panic!("should not happen");
        }

        let data_ptrs_2 = index.search(long_key).unwrap();
        assert_eq!(data_ptrs_2.len(), 1);
        assert!(data_ptrs_2.contains(&87968567));
    }

    #[test]
    fn test_root_split() {
        let file = build_file_path_and_rm_old("b_tree", "test_root_split.db").unwrap();
        let mut index = BTreeIndex::new(&file);

        for i in 0..1000 {
            index.insert(&format!("key # {}", i), i);
        }

        //index.soft_sync();

        for i in 0..1000 {
            let optrs = index.search(&format!("key # {}", i));
            if let Some(ptrs) = optrs {
                assert_eq!(ptrs.len(), 1);
                assert!(ptrs.contains(&i));
            } else {
                panic!("empty search result for key # {}", i);
            }
            
        }

    }

    
    #[test]
    fn test_root_split_same_key() {
        let file = build_file_path_and_rm_old("b_tree", "test_root_split_same_key.db").unwrap();
        let mut index = BTreeIndex::new(&file);

        for i in 0..3000 {
            index.insert("same key", i);
            if i % 1000 == 0 {
                println!("inserted {} values", i);
            }
        }
        //index.soft_sync();


        let optrs = index.search("same key");
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
            index.insert("same key", count);
            count += 1;
            index.insert("same key", count);
            count += 1;
            index.insert("same key", count);
            count += 1;
            if i % 1000 == 0 {
                println!("inserted {} values", i);
                index.sync();
                index.sync();
                index.sync();
                index = BTreeIndex::new(&file);
            }
            let optrs = index.search("same key");
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