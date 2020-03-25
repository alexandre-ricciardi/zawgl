use super::records::*;
use super::file_access::*;

pub struct NodesStore {
    node_records_file: FileAccess,
    not_in_use_nodes_pos: Vec<u64>,
}

impl NodesStore {
    pub fn new(file: &str) -> Self {
        let mut store = NodesStore { node_records_file: FileAccess::new(file), not_in_use_nodes_pos: Vec::new()};
        store.scan();
        store
    }
    pub fn save(&mut self, node: NodeRecord) {
        let data = nr_to_bytes(node);
        self.node_records_file.write_at(self.next_free_record_pos(), &data);
    }
    fn next_free_record_pos(&self) -> u64 {
        if self.not_in_use_nodes_pos.len() == 0 {
            self.node_records_file.get_file_len()
        } else {
            self.not_in_use_nodes_pos[0]
        }
    }
    fn scan(&mut self) {
        let len = self.node_records_file.get_file_len();
        let mut data = [0u8; 1];
        let mut index = 0u64;
        while index < len {
            self.node_records_file.read_at(index, &mut data);
            let in_use = data[0] & 0b0000_0001 > 0;
            if !in_use {
                self.not_in_use_nodes_pos.push(index);
            }
            index += 17;
        }
    }
    pub fn load(&mut self, node_id: u64) -> NodeRecord {
        let mut data = [0u8; 17];
        self.node_records_file.read_at(node_id * 17, &mut data);
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