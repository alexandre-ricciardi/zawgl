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

use super::records::*;
use super::super::super::buf_config::*;
use super::super::records::*;

pub struct RelationshipsStore {
    records_manager: RecordsManager,
}

impl RelationshipsStore {
    pub fn new(file: &str) -> Self {
        RelationshipsStore {records_manager: RecordsManager::new(file, RELATIONSHIP_RECORD_SIZE, RELATIONSHIP_NB_RECORDS_PER_PAGE, RELATIONSHIP_NB_PAGES_PER_RECORD)}
    }
    pub fn save(&mut self, id: u64, rel: &RelationshipRecord) -> Option<()> {
        self.records_manager.save(id, &rel.to_bytes()).ok()
    }
    pub fn create(&mut self, rel: &RelationshipRecord) -> Option<u64> {
        self.records_manager.create(&rel.to_bytes()).ok()
    }
    pub fn load(&mut self, rel_id: u64) -> Option<RelationshipRecord> {
        let mut data: [u8; RELATIONSHIP_RECORD_SIZE] = [0; RELATIONSHIP_RECORD_SIZE];
        self.records_manager.load(rel_id, &mut data).ok()?;
        Some(RelationshipRecord::from_bytes(data))
    }
    pub fn sync(&mut self) {
        self.records_manager.sync();
    }
    pub fn clear(&mut self) {
        self.records_manager.clear();
    }
}


#[cfg(test)]
mod test_relationship_store {
    use super::*;
    use super::super::super::super::test_utils::*;
    #[test]
    fn test_create_relationship_0() {
        let file = build_file_path_and_rm_old("test_relationship_store", "test_create_relationship_0.db").unwrap();
        let mut store = RelationshipsStore::new(&file);
        let rr = RelationshipRecord {
            source: 45465,
            target: 9871321,
            relationship_type: 456348,
            next_outbound_edge: 789545,
            next_inbound_edge: 7895465,
            next_prop_id: 8764564,
        };
        let id = store.create(&rr).unwrap();
        let r = store.load(id).unwrap();
        assert_eq!(r.source, 45465);
        assert_eq!(r.target, 9871321);
        assert_eq!(r.relationship_type, 456348);
        assert_eq!(r.next_outbound_edge, 789545);
        assert_eq!(r.next_inbound_edge, 7895465);
        assert_eq!(r.next_prop_id, 8764564);
    }
}