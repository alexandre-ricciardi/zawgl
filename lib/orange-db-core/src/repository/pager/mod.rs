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


const PAGE_HEADER_SIZE: usize = 1;
const PAGE_SIZE: usize = 4096;
const PAGE_PAYLOAD_SIZE: usize = PAGE_SIZE - PAGE_HEADER_SIZE;

pub struct Page {
    header: u8,
    payload: [u8; PAGE_PAYLOAD_SIZE],
}

impl Page {
    fn new() -> Self {
        Page{header: 0, payload: [0u8; PAGE_PAYLOAD_SIZE]}
    }

    fn get_next_free_record_pos(&self) -> u8 {
        self.header
    }

    fn set_next_free_record_pos(&mut self, pos: u8) {
        self.header = pos;
    }

    fn from_be_bytes(data: [u8; PAGE_SIZE]) -> Self {
        let mut payload = [0u8; PAGE_PAYLOAD_SIZE];
        payload.copy_from_slice(&data[PAGE_HEADER_SIZE..]);
        Page{header: data[0], payload: payload}
    }
}

pub struct Pager {
    records_file: FileAccess,
    record_size: usize,
    page_cache: HashMap<PageId, Box<Page>>
}

impl Pager {
    pub fn new(file: &str, record_size: usize) -> Self {
        Pager { records_file: FileAccess::new(file), record_size: record_size, page_cache: HashMap::new()}
    }
    fn compute_page_id(&self, record_id: u64) -> (PageId, PageRecordId) {
        let nb_records_per_page = PAGE_PAYLOAD_SIZE / self.record_size;
        (record_id / nb_records_per_page as u64, (record_id % nb_records_per_page as u64) as usize)
    }
    pub fn save(&mut self, id: RecordId, data: &[u8]) -> PagerResult {
        let location = self.compute_page_id(id);
        let page_begin_pos = location.0 * PAGE_SIZE as u64;
        if page_begin_pos > self.records_file.get_file_len() - PAGE_SIZE as u64 {
            Err(PagerError::FileOverflow)
        } else {
            let location_in_page = location.1 * self.record_size;
            if let Some(page) = self.page_cache.get(&location.0) {
                page.payload[location_in_page..location_in_page + self.record_size].copy_from_slice(data);
            } else {
                let mut page_data = [0u8; PAGE_SIZE];
                self.records_file.read_at(page_begin_pos, &mut page_data);
                let mut page = Box::new(Page::from_be_bytes(page_data));
                page.payload[location_in_page..location_in_page + self.record_size].copy_from_slice(data);
                self.page_cache.insert(location.0, page);
            }
            Ok(location.0)
        }
    }
    pub fn load(&mut self, id: RecordId, mut data: &mut [u8]) -> PagerResult {
        let location = self.compute_page_id(id);
        let page_begin_pos = location.0 * PAGE_SIZE as u64;
        if page_begin_pos > self.records_file.get_file_len() - PAGE_SIZE as u64 {
            Err(PagerError::FileOverflow)
        } else {
            let location_in_page = location.1 * self.record_size;
            if let Some(page) = self.page_cache.get(&location.0) {
                data.copy_from_slice(&page.payload[location_in_page..location_in_page + self.record_size]);
            } else {
                let mut page_data = [0u8; PAGE_SIZE];
                self.records_file.read_at(page_begin_pos, &mut page_data);
                let mut page = Box::new(Page::from_be_bytes(page_data));
                data.copy_from_slice(&page.payload[location_in_page..location_in_page + self.record_size]);
                self.page_cache.insert(location.0, page);
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