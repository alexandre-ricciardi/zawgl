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

pub struct DynamicStore {
    records_manager: RecordsManager,
}

impl DynamicStore {
    pub fn new(file: &str) -> Self {
        DynamicStore {records_manager: RecordsManager::new(file, DYN_RECORD_SIZE, DYN_NB_RECORDS_PER_PAGE, DYN_NB_PAGES_PER_RECORD)}
    }
    pub fn create(&mut self, dr: &DynamicStoreRecord) -> Option<u64> {
        self.records_manager.create(&dr_to_bytes(dr)).ok()
    }
    pub fn save_data(&mut self, data: &[u8]) -> Option<u64> {
        let mut count = data.len() / 120;
        let rest = data.len() % 120;
        let mut next = 0u64;
        let mut has_next = false;
        let mut end = rest + count * 120;
        loop {
            let mut dr = DynamicStoreRecord {
                in_use: true,
                has_next,
                data: [0u8; 120],
                next
            };
            let len = end - count * 120;
            dr.data[0..len].copy_from_slice(&data[count * 120..end]);
            next = self.create(&dr)?;
            end = count * 120;
            has_next = true;
            if count == 0 {
                break;
            } else {
                count -= 1;
            }
        }
        Some(next)
        
    }

    pub fn load_data(&mut self, id: u64) -> Option<Box<[u8]>> {
        let mut data = Vec::new();
        let mut next = id;
        let mut has_next = true;
        while has_next {
            let dr = self.load(next)?;
            data.extend_from_slice(&dr.data);
            has_next = dr.has_next;
            next = dr.next;
        }
        Some(data.into_boxed_slice())
    }

    pub fn load_string(&mut self, id: u64) -> Option<String> {
        let load = self.load_data(id)?;
        let mut it = load.iter();
        let str_end = it.position(|&c| c == b'\0').unwrap_or(load.len());
        let mut result = Vec::new();
        result.extend_from_slice(&load[0..str_end]);
        Some(String::from_utf8(result).ok()?)
    }

    pub fn load(&mut self, dr_id: u64) -> Option<DynamicStoreRecord> {
        let mut data: [u8; 129] = [0; 129];
        self.records_manager.load(dr_id, &mut data).ok()?;
        Some(dr_from_bytes(data))
    }

    pub fn retrieve_all_string(&mut self) -> Option<Vec<(String, u64)>> {
        let ids = self.records_manager.retrieve_all_records_ids().ok()?;
        let mut res = Vec::new();
        for id in ids {
            res.push((self.load_string(id)?, id))
        }
        Some(res)
    }

    pub fn sync(&mut self) {
        self.records_manager.sync();
    }

    pub fn clear(&mut self) {
        self.records_manager.clear();
    }
}

#[cfg(test)]
mod test_dyn_store {
    use super::*;
    use super::super::super::super::test_utils::*;
    #[test]
    fn test_dyn_short() {
        let file = build_file_path_and_rm_old("test_dyn_store", "test_dyn_short.db").unwrap();
        let mut ds = DynamicStore::new(&file);
        let short = b"qsdfqsdfqsdf";
        let id = ds.save_data(short).unwrap();
        let data = ds.load_data(id).unwrap();
        assert_eq!(&data[0..12], short);
    }

    #[test]
    fn test_dyn_long() {
        let file = build_file_path_and_rm_old("test_dyn_store", "test_dyn_long.db").unwrap();
        let mut ds = DynamicStore::new(&file);
        for i in 0..10 {
            let long = ["qsdfqsdfqsdlkqshdfhljbqlcznzelfnqelincqzlnfqzlnec
            qfqsdfqsdfqsdlkqshdfhljbqlcznzelfnqelincqzlnfqzlnecqfqsdfqsdfqsdlkqsh
            dfhljbqlcznzelfnqelincqzlnfqzlnecqfqsdfqsdfqsdlkqshdfhljbqlcznzelfnqel", &i.to_string()].concat();
            let input = long.clone();
            let id = ds.save_data(&long.into_bytes()).unwrap();
            let load = ds.load_string(id).unwrap();
            assert_eq!(input, load);
        }
    }  
}