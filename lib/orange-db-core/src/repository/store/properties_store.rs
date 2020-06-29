use super::super::super::config::*;
use super::super::records::*;
use super::records::*;

pub struct PropertiesStore {
    records_manager: RecordsManager,
}

impl PropertiesStore {
    pub fn new(file: &str) -> Self {
        PropertiesStore {records_manager: RecordsManager::new(file, PROPERTY_RECORD_SIZE, PROPERTY_NB_RECORDS_PER_PAGE, PROPERTY_NB_PAGES_PER_RECORD)}
    }
    pub fn create(&mut self, pr: &PropertyRecord) -> Option<u64> {
        self.records_manager.create(&pr_to_bytes(pr)).ok()
    }
    pub fn load(&mut self, pr_id: u64) -> Option<PropertyRecord> {
        let mut data: [u8; 42] = [0; 42];
        self.records_manager.load(pr_id, &mut data).ok()?;
        Some(pr_from_bytes(data))
    }
}