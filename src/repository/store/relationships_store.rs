use super::records::*;
use super::file_access::*;
use super::store::*;
use std::path::Path;

pub struct RelationshipsStore {
    relationship_records_store: Store,
}

impl RelationshipsStore {
    pub fn new(file: &str) -> Self {
        RelationshipsStore {relationship_records_store: Store::new(file, 65)}
    }
    pub fn save(&mut self, rel: &RelationshipRecord) -> u64 {
        self.relationship_records_store.save(&rr_to_bytes(rel))
    }
    pub fn load(&mut self, rel_id: u64) -> RelationshipRecord {
        let mut data: [u8; 65] = [0; 65];
        self.relationship_records_store.load(rel_id, &mut data);
        rr_from_bytes(data)
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
        store.save(&rr);
        let r = store.load(0);
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