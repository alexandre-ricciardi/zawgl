use super::super::super::buf_config::*;
use super::super::records::*;
use super::records::*;

pub struct NodesStore {
    records_manager: RecordsManager,
}

impl NodesStore {
    pub fn new(file: &str) -> Self {
        NodesStore {records_manager: RecordsManager::new(file, NODE_RECORD_SIZE, NODE_NB_RECORDS_PER_PAGE, NODE_NB_PAGES_PER_RECORD)}
    }
    pub fn save(&mut self, id: u64, node: &NodeRecord) -> Option<()> {
        self.records_manager.save(id, &nr_to_bytes(&node)).ok()
    }
    pub fn create(&mut self,node: &NodeRecord) -> Option<u64> {
        self.records_manager.create(&nr_to_bytes(&node)).ok()
    }
    pub fn load(&mut self, node_id: u64) -> Option<NodeRecord> {
        let mut data: [u8; 17] = [0; 17];
        self.records_manager.load(node_id, &mut data).ok()?;
        Some(nr_from_bytes(data))
    }
}

#[cfg(test)]
mod test_nodes_store {
    use super::*;
    fn clean() {
        std::fs::remove_file("C:\\Temp\\nodes.db");
    }
    #[test]
    fn test_create_node_0() {
        clean();
        let mut store = NodesStore::new("C:\\Temp\\nodes.db");
        let nr = NodeRecord {
            in_use: true,
            next_rel_id: 11287665,
            next_prop_id: 89089807,
        };
        let id = store.create(&nr).unwrap();
        let r = store.load(id).unwrap();
        assert_eq!(r.in_use, true);
        assert_eq!(r.next_rel_id, 11287665);
        assert_eq!(r.next_prop_id, 89089807);
    }
}