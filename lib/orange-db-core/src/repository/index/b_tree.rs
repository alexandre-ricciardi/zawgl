use super::store::*;
use super::model::*;

pub type DataPtr = u64;
pub struct BTreeIndex {
    node_store: BTreeNodeStore,
}



impl BTreeIndex {
    pub fn new(file: &str) -> Self {
        BTreeIndex{node_store: BTreeNodeStore::new(file)}
    }

    fn tree_search(&mut self, value: &str, node: &BTreeNode) -> Option<Vec<DataPtr>> {
        let keys = node.get_keys();
        let res = keys.binary_search(&String::from(value));
        match res {
            Ok(found) => {
                if node.is_leaf() {
                    Some(node.get_cell_ref(found).get_data_ptrs_ref().clone())
                } else {
                    let child = node.get_cell_ref(found).get_node_ptr().and_then(|id|self.node_store.retrieve_node(id))?;
                    self.tree_search(value, &child)
                }
            },
            Err(not_found) => {
                if node.is_leaf() {
                    None
                } else {
                    let child = node.get_cell_ref(not_found).get_node_ptr().and_then(|id|self.node_store.retrieve_node(id))?;
                    self.tree_search(value, &child)
                }
            }
        }
    }


    fn split_leaf_node(&mut self, value: &str, data_ptr: u64, node: &mut BTreeNode, new_cell_index: usize) -> Option<BTreeNode> {
        node.insert_cell(new_cell_index, Cell::new_leaf(value, data_ptr));
        let split = node.get_cells_ref().len() / 2;
        let mut new_node_cells = Vec::new();
        while node.get_cells_ref().len() > split {
            if let Some(cell) = node.pop_cell() {
                new_node_cells.push(cell);
            }
        }
        new_node_cells.reverse();
        let mut new = BTreeNode::new(true, false, new_node_cells);

        if node.get_node_ptr().is_some() {
            let next = node.get_node_ptr().and_then(|id| self.node_store.retrieve_node(id))?;
            new.set_node_ptr(next.get_id());
        }
        
        self.node_store.create(&mut new)?;
        node.set_node_ptr(new.get_id());
        self.node_store.save(node)?;
        Some(new)
    }

    fn split_interior_node(&mut self, new_key: &str, new_node_ptr: Option<NodeId>, parent_node: &mut BTreeNode, new_cell_index: usize) -> Option<BTreeNode> {
        parent_node.insert_cell(new_cell_index, Cell::new_ptr(new_key, new_node_ptr));
        let split = parent_node.get_cells_ref().len() / 2;
        let mut new_node_cells = Vec::new();
        while parent_node.get_cells_ref().len() > split {
            if let Some(cell) = parent_node.pop_cell() {
                new_node_cells.push(cell);
            }
        }
        new_node_cells.reverse();
        let mut new = BTreeNode::new(false, false, new_node_cells);
        self.node_store.create(&mut new);
        Some(new)
    }

    fn insert_or_update_key_ptrs(&mut self, value: &str, data_ptr: u64, node: &mut BTreeNode) -> Option<BTreeNode> {
        let keys = node.get_keys();
        let res = keys.binary_search(&String::from(value));
        match res {
            Ok(found) => {
                if node.is_leaf() {
                    node.get_cell_mut(found).append_data_ptr(data_ptr);
                    self.node_store.save(node)?;
                    None
                } else {
                    let mut child = node.get_cell_ref(found).get_node_ptr().and_then(|id| self.node_store.retrieve_node(id))?;
                    self.insert_or_update_key_ptrs(value, data_ptr, &mut child)
                }
            },
            Err(not_found) => {
                if node.is_leaf() {
                    if node.is_full() {
                        self.split_leaf_node(value, data_ptr, node, not_found)
                    } else {
                        node.insert_cell(not_found, Cell::new_leaf(value, data_ptr));
                        self.node_store.save(node)?;
                        None
                    }
                } else {
                    let mut child = node.get_cell_ref(not_found).get_node_ptr().and_then(|id|self.node_store.retrieve_node(id))?;
                    let split_node = self.insert_or_update_key_ptrs(value, data_ptr, &mut child)?;
                    let first_cell = split_node.get_cell_ref(0);
                    let first_split_cell_key_search = keys.binary_search(first_cell.get_key());
                    match first_split_cell_key_search {
                        Err(not_found) => {
                            if node.is_full() {
                                self.split_interior_node(first_cell.get_key(), split_node.get_id(), node, not_found)
                            } else {
                                node.insert_cell(not_found, Cell::new_ptr(first_cell.get_key(), split_node.get_id()));
                                self.node_store.save(node)?;
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
        let mut root = self.node_store.load_or_create_root_node()?;
        self.insert_or_update_key_ptrs(value, data_ptr, &mut root).map(|_node|())
    }

    pub fn delete(&mut self, value: u64) {

    }

    pub fn sync(&mut self) {
        self.node_store.sync();
    }
}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_insert() {
        let file = "C:\\Temp\\test_b_tree_insert.db";
        std::fs::remove_file(file);
        let mut index = BTreeIndex::new(file);
        let key = "a short key";
        index.insert(key, 42);

        let data_ptrs = index.search(key);

        if let Some(ptrs) = &data_ptrs {
            assert_eq!(ptrs.len(), 1);
            assert_eq!(ptrs[0], 42);
        } else {
            assert!(false);
        }

        index.sync();

        index.insert(key, 56);

        let data_ptrs_1 = index.search(key);

        if let Some(ptrs) = &data_ptrs_1 {
            assert_eq!(ptrs.len(), 2);
            assert_eq!(ptrs[0], 42);
            assert_eq!(ptrs[0], 56);
        } else {
            assert!(false);
        }

    }
}