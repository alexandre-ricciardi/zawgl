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
    page_map: PageMap,
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

struct Bounds {
    begin: usize,
    end: usize,
}

impl Bounds {
    fn new(b: usize, e: usize) -> Self {
        Bounds{begin: b, end: e}
    }
    fn size(&self) -> usize {
        self.end - self.begin
    }
}

struct PageMap {
    header_flags: Bounds,
    next_free_slot_page_ptr: Bounds,
    multi_page_recode_ptr: Bounds,
    payload: Bounds,
}

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

fn compute_page_map(record_size: usize) -> PageMap {
    let free_list_size = compute_freelist_size(record_size);
    PageMap{
        header_flags: Bounds{begin: 0usize, end: HEADER_FLAGS},
        next_free_slot_page_ptr: Bounds{begin: HEADER_FLAGS, end: free_list_size + HEADER_FLAGS},
        multi_page_recode_ptr: Bounds{begin: free_list_size + HEADER_FLAGS, end: free_list_size + HEADER_FLAGS + MULTI_PAGE_RECORD_PTR_SIZE},
        payload: Bounds{begin: HEADER_SIZE, end: PAGE_SIZE}
    }
}

impl RecordsManager {
    pub fn new(file: &str, rsize: usize) -> Self {
        RecordsManager{pager: Pager::new(file), record_size: rsize, page_map: compute_page_map(rsize)}
    }

    fn is_multi_pages_record(&self) -> bool {
        let page_payload_size = self.page_map.payload.size();
        page_payload_size / self.record_size == 0
    }

    fn compute_location(&self, record_id: u64) -> RecordLocation {
        let page_payload_size = self.page_map.payload.size();
        let nb_records_per_page = page_payload_size / self.record_size;
        if self.is_multi_pages_record() {
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

    fn append_page_data(&self, mut data: &mut [u8], page: &Page) {
        let mut len = self.page_map.payload.size();
        if len > data.len() {
            len = data.len();
        }
        data[..len].copy_from_slice(&page.data[self.page_map.payload.begin..]);
    }

    fn load_multi_page(&mut self, mut data: &mut [u8], current_page: &Page, page_count: usize) {
        self.append_page_data(&mut data[page_count*self.page_map.payload.size()..], &current_page);
        if current_page.has_multi_page_record() {
            let next_page_ptr = current_page.get_multi_page_ptr();
            let mut next_page = self.pager.load_page(&next_page_ptr);
            self.load_multi_page(data, &next_page, page_count + 1);
        }
    }

    pub fn load(&mut self, id: RecordId, mut data: &mut [u8]) {
        let location = self.compute_location(id);
        let first_page = self.pager.load_page(&location.page_id);
        if location.is_multi_pages_record {
            self.load_multi_page(data, &first_page, 0);
        } else {
            let location_in_page = location.record_id_in_page * self.record_size;
            data.copy_from_slice(&first_page.data[location_in_page..location_in_page + self.record_size]);
        }
    }

    pub fn append(&mut self, data: &[u8]) -> RecordId {
        0
    }
}