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
        self.records_manager.save(id, &rr_to_bytes(rel)).ok()
    }
    pub fn create(&mut self, rel: &RelationshipRecord) -> Option<u64> {
        self.records_manager.create(&rr_to_bytes(&rel)).ok()
    }
    pub fn load(&mut self, rel_id: u64) -> Option<RelationshipRecord> {
        let mut data: [u8; 65] = [0; 65];
        self.records_manager.load(rel_id, &mut data).ok()?;
        Some(rr_from_bytes(data))
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
            in_use: true,
            first_node: 45465,
            second_node: 9871321,
            relationship_type: 456348,
            first_prev_rel_id: 789545,
            first_next_rel_id: 7895465,
            second_prev_rel_id: 789455,
            second_next_rel_id: 554978,
            next_prop_id: 8764564,
        };
        let id = store.create(&rr).unwrap();
        let r = store.load(id).unwrap();
        assert_eq!(r.in_use, true);
        assert_eq!(r.first_node, 45465);
        assert_eq!(r.second_node, 9871321);
        assert_eq!(r.relationship_type, 456348);
        assert_eq!(r.first_prev_rel_id, 789545);
        assert_eq!(r.first_next_rel_id, 7895465);
        assert_eq!(r.second_prev_rel_id, 789455);
        assert_eq!(r.second_next_rel_id, 554978);
        assert_eq!(r.next_prop_id, 8764564);
    }
}