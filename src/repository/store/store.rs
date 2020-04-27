
use super::super::file::file_access::*;
use std::collections::BTreeSet;

pub struct Store {
    records_file: FileAccess,
    not_in_use_records_pos: BTreeSet<u64>,
    record_size: u64,
    current_file_len: u64,
}

impl Store {
    pub fn new(file: &str, record_size: u64) -> Self {
        let mut store = Store { records_file: FileAccess::new(file), not_in_use_records_pos: BTreeSet::new(), record_size: record_size, current_file_len: 0};
        store.scan();
        store.current_file_len = store.records_file.get_file_len();
        store
    }
    pub fn save(&mut self, id: u64, data: &[u8]) -> u64 {
        let next_pos = id * self.record_size;
        self.records_file.write_at(next_pos, &data);
        next_pos / self.record_size
    }

    fn next_free_record_pos(&mut self) -> u64 {
        if let Some(&pos) = self.not_in_use_records_pos.iter().next() {
            self.not_in_use_records_pos.remove(&pos);
            pos
        } else {
            let prev_file_len = self.current_file_len;
            self.current_file_len += self.record_size;
            prev_file_len
        }
    }

    pub fn next_free_record_id(&mut self) -> u64 {
        self.next_free_record_pos() / self.record_size
    }

    pub fn compute_next_free_records_ids(&self, n: usize) -> Vec<u64> {
        let mut res = Vec::new();
        let mut next = 0;
        let size_not_used = self.not_in_use_records_pos.len();
        let mut not_used_index = 0usize;
        while next < n {
            if not_used_index < size_not_used {
                //res.push(self.not_in_use_records_pos.pop_first() / self.record_size);
                not_used_index += 1;
            } else {
                res.push(self.records_file.get_file_len() / self.record_size + next as u64);
            }
            next += 1;
        }
        res
    }
    fn scan(&mut self) {
        let len = self.records_file.get_file_len();
        let mut data = [0u8; 1];
        let mut pos = 0u64;
        while pos < len {
            self.records_file.read_at(pos, &mut data);
            let in_use = data[0] & 0b0000_0001 > 0;
            if !in_use {
                self.not_in_use_records_pos.insert(pos);
            }
            pos += self.record_size;
        }
    }
    pub fn load(&mut self, id: u64, mut data:&mut [u8]) {
        self.records_file.read_at(id * self.record_size, &mut data);
    }
}