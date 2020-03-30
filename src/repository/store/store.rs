use super::records::*;
use super::file_access::*;
use std::path::Path;

pub struct Store {
    records_file: FileAccess,
    not_in_use_records_pos: Vec<u64>,
    record_size: u64,
}

impl Store {
    pub fn new(file: &str, record_size: u64) -> Self {
        let mut store = Store { records_file: FileAccess::new(file), not_in_use_records_pos: Vec::new(), record_size: record_size};
        store.scan();
        store
    }
    pub fn save(&mut self, data: &[u8]) -> u64{
        let next_pos = self.next_free_record_pos();
        self.records_file.write_at(next_pos, &data);
        next_pos / self.record_size
    }
    fn next_free_record_pos(&mut self) -> u64 {
        let last = self.not_in_use_records_pos.pop();
        match last {
            Some(pos) => pos,
            None => self.records_file.get_file_len()
        }
    }
    fn scan(&mut self) {
        let len = self.records_file.get_file_len();
        let mut data = [0u8; 1];
        let mut index = 0u64;
        while index < len {
            self.records_file.read_at(index, &mut data);
            let in_use = data[0] & 0b0000_0001 > 0;
            if !in_use {
                self.not_in_use_records_pos.push(index);
            }
            index += self.record_size;
        }
    }
    pub fn load(&mut self, id: u64, mut data:&mut [u8]) {
        self.records_file.read_at(id * self.record_size, &mut data);
    }
}