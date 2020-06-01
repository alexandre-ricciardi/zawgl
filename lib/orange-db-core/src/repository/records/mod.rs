use super::pager::*;

pub type RecordId = u64;
pub type PageRecordId = usize;

struct RecordLocation {
    page_id: PageId,
    record_id_in_page: PageRecordId,
    nb_pages_per_record: usize,
    is_multi_pages_record: bool,
}

pub struct RecordsManager {
    pager: Pager,
    record_size: usize,
}

trait PageRecord {
    fn has_multi_page_record(&self) -> bool;
    fn get_free_list_ptr(&self) -> PageId;
    fn get_page_free_list(&self) -> Vec<PageRecordId>;
    fn get_multi_page_ptr(&self) -> PageId;
}

impl <'a> PageRecord for Page<'a> {
    fn has_multi_page_record(&self) -> bool {
        self.data[0] & MULTI_PAGE_RECORD_FLAG == 1
    }
    fn get_free_list_ptr(&self) -> PageId {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[1..9]);
        u64::from_be_bytes(bytes)
    }
    fn get_page_free_list(&self) -> Vec<PageRecordId> {
        Vec::new()
    }
    fn get_multi_page_ptr(&self) -> PageId {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[9..9+MULTI_PAGE_RECORD_PTR_SIZE]);
        u64::from_be_bytes(bytes)
    }
}

const FREE_LIST_PTR_SIZE: usize = 4;
const MULTI_PAGE_RECORD_PTR_SIZE: usize = 8;
const NEXT_FREE_SLOT_PAGE_PTR_SIZE: usize = 8;
const HEADER_FLAGS: usize = 1; 
const HEADER_SIZE: usize = HEADER_FLAGS + NEXT_FREE_SLOT_PAGE_PTR_SIZE + MULTI_PAGE_RECORD_PTR_SIZE;

const MULTI_PAGE_RECORD_FLAG: u8 = 0b1000_0000;

fn compute_freelist_len(record_size: usize) -> usize {
    (PAGE_SIZE - HEADER_SIZE) / record_size
}
fn compute_freelist_size(record_size: usize) -> usize {
    FREE_LIST_PTR_SIZE * compute_freelist_len(record_size)
}
fn compute_header_size(record_size: usize) -> usize {
    compute_freelist_size(record_size) + HEADER_SIZE
}
fn compute_payload_size(record_size: usize) -> usize {
    PAGE_SIZE - compute_header_size(record_size)
}

impl RecordsManager {
    pub fn new(file: &str, rsize: usize) -> Self {
        RecordsManager{pager: Pager::new(file), record_size: rsize}
    }

    fn compute_location(&self, record_id: u64) -> RecordLocation {
        let page_payload_size = compute_payload_size(self.record_size);
        let nb_records_per_page = page_payload_size / self.record_size;
        if nb_records_per_page == 0 {
            let nb_pages_per_record = self.record_size / page_payload_size;
            RecordLocation{
                page_id: 1 + (nb_pages_per_record as u64 * record_id),
                record_id_in_page: 0,
                nb_pages_per_record: nb_pages_per_record,
                is_multi_pages_record: true}
        } else {
            RecordLocation{
                page_id: 1 + (record_id / nb_records_per_page as u64),
                record_id_in_page: (record_id % nb_records_per_page as u64) as usize,
                nb_pages_per_record: 0,
                is_multi_pages_record: false
            }
        }
    }

    pub fn load(&mut self, id: RecordId, mut data: &mut [u8]) {
        let location = self.compute_location(id);
        if location.is_multi_pages_record {
            let mut pages = Vec::new();
            let first_page = self.pager.load_page(&location.page_id);
            pages.push(first_page);

        }
        let page = self.pager.load_page(&location.0);
        let location_in_page = location.1 * self.record_size;
        data.copy_from_slice(&page.data[location_in_page..location_in_page + self.record_size]);
    }

    pub fn append(&mut self, data: &[u8]) -> RecordId {
        0
    }
}