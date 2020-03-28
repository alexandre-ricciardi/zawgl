use super::records::*;
use super::file_access::*;
use super::store::*;

pub struct NodesStore {
    node_records_store: Store,
}

impl NodesStore {
    pub fn new(file: &str) -> Self {
        NodesStore {node_records_store: Store::new(file, 17)}
    }
    pub fn save(&mut self, node: NodeRecord) -> u64 {
        self.node_records_store.save(&nr_to_bytes(node))
    }
    pub fn load(&mut self, node_id: u64) -> NodeRecord {
        let mut data: [u8; 17] = [0; 17];
        self.node_records_store.load(node_id, &mut data);
        nr_from_bytes(data)
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
        store.save(nr);
        let r = store.load(0);
        assert_eq!(r.in_use, true);
        assert_eq!(r.next_rel_id, 11287665);
        assert_eq!(r.next_prop_id, 89089807);
    }
}