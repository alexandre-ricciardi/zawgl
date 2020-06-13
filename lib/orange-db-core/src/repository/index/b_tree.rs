use super::super::super::config::*;
use super::node_store::*;

pub type DataPtr = u64;
pub struct BTreeIndex {
    records_manager: BTreeNodeStore,
}



impl BTreeIndex {
    pub fn new(file: &str) -> Self {
        BTreeIndex{records_manager: BTreeNodeStore::new(file)}
    }

    fn tree_search(&mut self, value: &str, node: &BTreeNode, depth: u32) -> Option<DataPtr> {
        let keys = node.get_keys_string();
        let res = keys.binary_search(&String::from(value));
        match res {
            Ok(found) => {
                if node.is_leaf() {
                    Some(node.get_ptr_value(found))
                } else {
                    let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
                    self.records_manager.load(node.get_ptr_value(found), &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                }
            },
            Err(not_found) => {
                if node.is_leaf() {
                    None
                } else {
                    let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
                    self.records_manager.load(node.get_ptr_value(not_found), &mut data);
                    let child = BNodeRecord::from_bytes(data);
                    self.tree_search(value, &child, depth+1)
                }
            }
        }
    }

    pub fn search(&mut self, value: &str) -> Option<DataPtr> {
        if self.records_manager.is_empty() {
            None
        } else {
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            self.records_manager.load(0, &mut  data);
            self.tree_search(value, &BNodeRecord::from_bytes(data), 0)
        }
    }

    pub fn insert(&mut self, value: u64, data_ptr: u64) {

    }
}

#[cfg(test)]
mod test_b_tree {
    use super::*;
    #[test]
    fn test_ser() {
    }
}