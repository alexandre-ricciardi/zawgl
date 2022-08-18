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
        self.records_manager.save(id, &node.to_bytes()).ok()
    }
    pub fn create(&mut self,node: &NodeRecord) -> Option<u64> {
        self.records_manager.create(&node.to_bytes()).ok()
    }
    pub fn load(&mut self, node_id: u64) -> Option<NodeRecord> {
        let mut data: [u8; NODE_RECORD_SIZE] = [0; NODE_RECORD_SIZE];
        self.records_manager.load(node_id, &mut data).ok()?;
        Some(NodeRecord::from_bytes(data))
    }
    pub fn sync(&mut self) {
        self.records_manager.sync();
    }

    pub fn retrieve_all_nodes_ids(&mut self) -> Option<Vec<u64>> {
        self.records_manager.retrieve_all_records_ids().ok()
    }
}

#[cfg(test)]
mod test_nodes_store {
    use super::*;
    use super::super::super::super::test_utils::*;
    
    #[test]
    fn test_create_node_0() {
        let file = build_file_path_and_rm_old("test_nodes_store", "test_create_node_0.db").unwrap();
        let mut store = NodesStore::new(&file);
        let nr = NodeRecord {
            first_inbound_edge: 11287665,
            first_outbound_edge: 87687554,
            next_prop_id: 89089807,
            node_type: 3,
        };
        let id = store.create(&nr).unwrap();
        let r = store.load(id).unwrap();
        assert_eq!(r.first_inbound_edge, 11287665);
        assert_eq!(r.first_outbound_edge, 87687554);
        assert_eq!(r.next_prop_id, 89089807);

        let rids = store.retrieve_all_nodes_ids().unwrap();
        assert_eq!(1, rids.len());
        assert_eq!(id, rids[0]);
    }
}