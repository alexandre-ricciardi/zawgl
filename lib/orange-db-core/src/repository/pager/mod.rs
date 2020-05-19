use super::io::file_access::*;
use std::collections::HashMap;

pub type PageId = u64;
pub type RecordId = u64;
pub type PageRecordId = usize;

#[derive(Debug, Clone)]
pub enum PagerError {
    FileOverflow,
}

pub type PagerResult = std::result::Result<PageId, PagerError>;

pub struct Pager {
    records_file: FileAccess,
    record_size: usize,
    page_cache: HashMap<PageId, Box<[u8]>>
}

impl Pager {
    const PAGE_HEADER_SIZE: usize = 0;
    const PAGE_SIZE: usize = 4096;
    pub fn new(file: &str, page_size: usize, record_size: usize) -> Self {
        Pager { records_file: FileAccess::new(file), record_size: record_size, page_cache: HashMap::new()}
    }
    fn compute_page_id(&self, record_id: u64) -> (PageId, PageRecordId) {
        let page_payload_size = Pager::PAGE_SIZE - Pager::PAGE_HEADER_SIZE;
        let nb_records_per_page = page_payload_size / self.record_size;
        (record_id / nb_records_per_page as u64, (record_id % nb_records_per_page as u64) as usize)
    }
    pub fn save(&mut self, id: RecordId, data: &[u8]) -> PagerResult {
        let pos = id * self.record_size as u64;
        if pos > self.records_file.get_file_len() {
            Err(PagerError::FileOverflow)
        } else {
            self.records_file.write_at(pos, &data);
            Ok(id)
        }
    }
    pub fn load(&mut self, id: RecordId, mut data: &mut [u8]) -> PagerResult {
        let location = self.compute_page_id(id);
        let page_begin_pos = location.0 * Pager::PAGE_SIZE as u64;
        if page_begin_pos > self.records_file.get_file_len() - Pager::PAGE_SIZE as u64 {
            Err(PagerError::FileOverflow)
        } else {
            let location_in_page = location.1 as usize * self.record_size;
            if let Some(page) = self.page_cache.get(&location.0) {
                data.copy_from_slice(&page[location_in_page..location_in_page + self.record_size as usize]);
            } else {
                let mut page_data = Box::new([0u8; Pager::PAGE_SIZE]);
                self.records_file.read_at(page_begin_pos, &mut *page_data);
                self.page_cache.insert(location.0, page_data);
                data.copy_from_slice(&page_data[location_in_page..location_in_page + self.record_size as usize]);
            }
            Ok(location.0)
        }
    }
    pub fn append(&mut self, data: &[u8]) -> PagerResult {
        let next_pid = self.records_file.get_file_len() / self.record_size;
        self.save(next_pid, data)
    }
    pub fn is_empty(&self) -> bool {
        self.records_file.get_file_len() == 0
    }
}