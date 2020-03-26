use super::records::*;
use super::store::*;

pub struct DynamicStore {
    dyn_records_store: Store,
}

impl DynamicStore {
    pub fn new(file: &str) -> Self {
        DynamicStore {dyn_records_store: Store::new(file, 129)}
    }
    pub fn save(&mut self, dr: DynamicStoreRecord) {
        self.dyn_records_store.save(&dr_to_bytes(dr));
    }
    pub fn load(&mut self, dr_id: u64) -> DynamicStoreRecord {
        let mut data: [u8; 129] = [0; 129];
        self.dyn_records_store.load(dr_id, &mut data);
        dr_from_bytes(data)
    }
}