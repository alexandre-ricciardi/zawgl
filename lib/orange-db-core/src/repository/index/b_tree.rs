use super::super::super::config::*;
use super::node_store::*;

pub type DataPtr = u64;
pub struct BTreeIndex {
    node_store: BTreeNodeStore,
}



impl BTreeIndex {
    pub fn new(file: &str) -> Self {
        BTreeIndex{node_store: BTreeNodeStore::new(file)}
    }

    fn tree_search(&mut self, value: &str, node: &BTreeNode, depth: u32) -> Option<Vec<DataPtr>> {
        let keys = node.get_keys();
        let res = keys.binary_search(&String::from(value));
        match res {
            Ok(found) => {
                if node.is_leaf {
                    Some(node.cells[found].data_ptrs.clone())
                } else {
                    let child = self.node_store.retrieve_node(node.cells[found].node_ptr)?;
                    self.tree_search(value, &child, depth+1)
                }
            },
            Err(not_found) => {
                if node.is_leaf {
                    None
                } else {
                    let child = self.node_store.retrieve_node(node.cells[not_found].node_ptr)?;
                    self.tree_search(value, &child, depth+1)
                }
            }
        }
    }

    pub fn search(&mut self, value: &str) -> Option<Vec<DataPtr>> {
        let root = self.node_store.retrieve_node(0);
        root.and_then(|node| self.tree_search(value, &node, 0))
    }

    fn split_leaf_node(&mut self, node: &mut BTreeNode) {
        
    }

    fn insert_or_update_key(&mut self, value: &str, data_ptr: u64, node: &mut BTreeNode) -> Option<NodeId> {
        let keys = node.get_keys();
        let res = keys.binary_search(&String::from(value));
        match res {
            Ok(found) => {
                if node.is_leaf {
                    node.cells[found].node_ptr = data_ptr;
                    Some(node.id)
                } else {
                    let mut child = self.node_store.retrieve_node(node.cells[found].node_ptr)?;
                    self.insert_or_update_key(value, data_ptr, &mut child)
                }
            },
            Err(not_found) => {
                if node.is_leaf {
                    if node.is_full() {

                    } else {
                        node.cells.insert(not_found, Cell::new(value, data_ptr));
                    }
                    Some(node.id)
                } else {
                    let mut child = self.node_store.retrieve_node(node.cells[not_found].node_ptr)?;
                    self.insert_or_update_key(value, data_ptr, &mut child)
                }
            }
        }
    }

    pub fn insert(&mut self, value: u64, data_ptr: u64) {
        
    }

    pub fn delete(&mut self, value: u64) {

    }

}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_ser() {
    }
}