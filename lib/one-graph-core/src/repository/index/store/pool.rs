use crate::repository::store::records::NodeRecord;

use super::records::*;
use super::super::super::records::*;
use super::super::super::super::buf_config::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct NodeRecordPool {
    pub records: HashMap<u64, BNodeRecord>,
    pub records_manager: Rc<RefCell<RecordsManager>>,
}

impl NodeRecordPool {

    pub fn new(record_manager: Rc<RefCell<RecordsManager>>) -> Self {
        NodeRecordPool{ records: HashMap::new(), records_manager: record_manager }
    }

    pub fn load_node_record_clone(&mut self, id: u64) -> Option<BNodeRecord> {
        if !self.records.contains_key(&id) {
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            self.records_manager.borrow_mut().load(id, &mut data).ok()?;
            self.records.insert(id, BNodeRecord::from_bytes(data));
        }
        Some(self.records.get(&id)?.clone())
    }

    pub fn load_node_record_ref(&mut self, id: u64) -> Option<&BNodeRecord> {
        if !self.records.contains_key(&id) {
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            self.records_manager.borrow_mut().load(id, &mut data).ok()?;
            self.records.insert(id, BNodeRecord::from_bytes(data));
        }
        Some(self.records.get(&id)?)
    }

    pub fn load_node_record_mut(&mut self, id: u64) -> Option<&mut BNodeRecord> {
        if !self.records.contains_key(&id) {
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            self.records_manager.borrow_mut().load(id, &mut data).ok()?;
            self.records.insert(id, BNodeRecord::from_bytes(data));
            
        }
        Some(self.records.get_mut(&id)?)
    }

    pub fn create_node_record(&mut self, node_record: BNodeRecord) -> Option<u64> {
        let id = self.records_manager.borrow_mut().create(&node_record.to_bytes()).ok()?;
        self.records.insert(id, node_record);
        Some(id)
    }

    pub fn save_all_node_records(&mut self) -> Option<()> {
        for r in &self.records {
            self.records_manager.borrow_mut().save(*r.0, &r.1.to_bytes()).ok()?
        }
        Some(())
    }
}