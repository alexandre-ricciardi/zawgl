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


const PAGE_HEADER_SIZE: usize = 8;
const PAGE_SIZE: usize = 4096;
const PAGE_FREE_LIST_PTR_SIZE: usize = 4;

fn compute_freelist_len(record_size: usize) -> usize {
    (PAGE_SIZE - PAGE_HEADER_SIZE) / record_size
}
fn compute_freelist_size(record_size: usize) -> usize {
    PAGE_FREE_LIST_PTR_SIZE * compute_freelist_len(record_size)
}
fn compute_header_size(record_size: usize) -> usize {
    compute_freelist_size(record_size) + PAGE_HEADER_SIZE
}
fn compute_payload_size(record_size: usize) -> usize {
    PAGE_SIZE - compute_header_size(record_size)
}

pub struct Page<'a> {
    data: &'a mut [u8; PAGE_SIZE],
    record_size: usize,
}

impl <'a> Page<'a> {
    fn new(data: &'a mut [u8; PAGE_SIZE], record_size: usize) -> Self {
        Page{data: data, record_size: record_size}
    }

    fn get_next_freeblock_page(&self) -> PageId {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[..8]);
        u64::from_be_bytes(bytes)
    }

    fn set_next_freeblock_page(&mut self, pid: PageId) {
        self.data.copy_from_slice(&pid.to_be_bytes());
    }

    fn get_list_free_page_record_ids(&self) -> Vec<PageRecordId> {
        let mut res = Vec::new();
        let freelist_len = compute_freelist_len(self.record_size);
        let mut count = 0;
        while count < freelist_len {
            let beg_ptr = count * PAGE_FREE_LIST_PTR_SIZE;
            let end_ptr = beg_ptr + PAGE_FREE_LIST_PTR_SIZE;
            let page_record_id_bytes = [0u8; PAGE_FREE_LIST_PTR_SIZE];
            page_record_id_bytes.copy_from_slice(&self.data[beg_ptr..end_ptr]);
            let page_record_id = u32::from_be_bytes(page_record_id_bytes) as usize;
            if page_record_id == 0 {
                break;
            }
            res.push(page_record_id);
            count += 1;
        }
        res
    }

    fn set_list_free_page_record_ids(&self, freelist: Vec<PageRecordId>) {
        
    }
}

pub struct Pager {
    records_file: FileAccess,
    record_size: usize,
    page_cache: HashMap<PageId, [u8; PAGE_SIZE]>
}

impl Pager {
    pub fn new(file: &str, record_size: usize) -> Self {
        Pager { records_file: FileAccess::new(file), record_size: record_size, page_cache: HashMap::new()}
    }
    
    fn compute_page_id(&self, record_id: u64) -> (PageId, PageRecordId) {
        let nb_records_per_page = compute_payload_size(self.record_size) / self.record_size;
        (record_id / nb_records_per_page as u64, (record_id % nb_records_per_page as u64) as usize)
    }
    pub fn save(&mut self, id: RecordId, data: &[u8]) -> PagerResult {
        let location = self.compute_page_id(id);
        let page_begin_pos = location.0 * PAGE_SIZE as u64;
        if page_begin_pos > self.records_file.get_file_len() - PAGE_SIZE as u64 {
            Err(PagerError::FileOverflow)
        } else {
            let location_in_page = location.1 * self.record_size + PAGE_HEADER_SIZE;
            if let Some(page) = self.page_cache.get(&location.0) {
                page[location_in_page..location_in_page + self.record_size].copy_from_slice(data);
            } else {
                let mut page_data = [0u8; PAGE_SIZE];
                self.records_file.read_at(page_begin_pos, &mut page_data);
                page_data[location_in_page..location_in_page + self.record_size].copy_from_slice(data);
                self.page_cache.insert(location.0, page_data);
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
                data.copy_from_slice(&page[location_in_page..location_in_page + self.record_size]);
            } else {
                let mut page_data = [0u8; PAGE_SIZE];
                self.records_file.read_at(page_begin_pos, &mut page_data);
                data.copy_from_slice(&page_data[location_in_page..location_in_page + self.record_size]);
                self.page_cache.insert(location.0, page_data);
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