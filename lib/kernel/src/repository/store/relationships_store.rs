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
}


#[cfg(test)]
mod test_relationship_store {
    use super::*;
    fn clean(file: &str) {
        std::fs::remove_file(file);
    }
    #[test]
    fn test_create_relationship_0() {
        clean("C:\\Temp\\relationships.db");
        let mut store = RelationshipsStore::new("C:\\Temp\\relationships.db");
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