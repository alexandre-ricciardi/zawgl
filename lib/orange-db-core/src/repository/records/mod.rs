use super::pager::*;

pub type RecordId = u64;
pub type PageRecordId = usize;

struct RecordLocation {
    page_id: PageId,
    record_id_in_page: PageRecordId,
    page_record_address: usize,
    payload_record_address: usize,
    nb_pages_per_record: usize,
    is_multi_pages_record: bool,
}

pub struct RecordsManager {
    pager: Pager,
    record_size: usize,
    page_map: PageMap,
}

struct RecordPageWrapper<'a> {
    page: Page<'a>,
    page_map: PageMap,
}

impl <'a> RecordPageWrapper<'a> {
    fn new(page: Page<'a>, page_map: PageMap) -> Self {
        RecordPageWrapper{page: page, page_map: page_map}
    }
    fn has_multi_page_record(&self) -> bool {
        self.page.data[0] & MULTI_PAGE_RECORD_FLAG == 1
    }
    fn get_free_list_ptr(&self) -> PageId {
        let mut bytes = [0u8; NEXT_FREE_SLOT_PAGE_PTR_SIZE];
        bytes.copy_from_slice(self.get_slice_ref(&self.page_map.next_free_slot_page_ptr));
        u64::from_be_bytes(bytes)
    }
    fn get_multi_page_ptr(&self) -> PageId {
        let mut bytes = [0u8; MULTI_PAGE_RECORD_PTR_SIZE];
        bytes.copy_from_slice(self.get_slice_ref(&self.page_map.multi_page_recode_ptr));
        u64::from_be_bytes(bytes)
    }
    fn get_slice_ref(&self, bounds: &Bounds) -> &[u8] {
        &self.page.data[bounds.begin..bounds.end]
    }
    fn get_free_list_len(&self) -> usize {
        let mut bytes = [0u8; FREE_LIST_LEN_SIZE];
        bytes.copy_from_slice(self.get_slice_ref(&self.page_map.free_list_len));
        u16::from_be_bytes(bytes) as usize
    }
    fn get_page_free_list(&self) -> Vec<PageRecordId> {
        let mut res = Vec::new();
        let free_list_slice = self.get_slice_ref(&self.page_map.free_list);
        for count in 0..self.get_free_list_len() {
            let offset = count * FREE_LIST_PTR_SIZE;
            let mut bytes = [0u8; FREE_LIST_PTR_SIZE];
            bytes.copy_from_slice(&free_list_slice[offset..offset+FREE_LIST_PTR_SIZE]);
            res.push(u32::from_be_bytes(bytes) as usize);
        }
        res
    }
}

const FREE_LIST_PTR_SIZE: usize = 4;
const FREE_LIST_LEN_SIZE: usize = 2;
const MULTI_PAGE_RECORD_PTR_SIZE: usize = 8;
const NEXT_FREE_SLOT_PAGE_PTR_SIZE: usize = 8;
const HEADER_FLAGS: usize = 1; 
const HEADER_SIZE: usize = HEADER_FLAGS + NEXT_FREE_SLOT_PAGE_PTR_SIZE + MULTI_PAGE_RECORD_PTR_SIZE;

const MULTI_PAGE_RECORD_FLAG: u8 = 0b1000_0000;

#[derive(Debug, Copy, Clone)]
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
    fn shift(&self, size: usize) -> Self {
        Bounds{begin: self.end, end: self.end + size}
    }
}

#[derive(Debug, Copy, Clone)]
struct PageMap {
    header_flags: Bounds,
    next_free_slot_page_ptr: Bounds,
    multi_page_recode_ptr: Bounds,
    free_list_len: Bounds,
    free_list: Bounds,
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
    let header_flags_bounds = Bounds::new(0, HEADER_FLAGS);
    let next_free_slot_page_ptr_bounds = header_flags_bounds.shift(NEXT_FREE_SLOT_PAGE_PTR_SIZE);
    let multi_page_recode_ptr_bounds = next_free_slot_page_ptr_bounds.shift(MULTI_PAGE_RECORD_PTR_SIZE);
    let free_list_len = multi_page_recode_ptr_bounds.shift(FREE_LIST_LEN_SIZE);
    let free_list_bounds = free_list_len.shift(free_list_size);
    let payload_bounds = Bounds::new(free_list_bounds.end, PAGE_SIZE);
    PageMap{
        header_flags: header_flags_bounds,
        next_free_slot_page_ptr: next_free_slot_page_ptr_bounds,
        multi_page_recode_ptr: multi_page_recode_ptr_bounds,
        free_list_len: free_list_len,
        free_list: free_list_bounds,
        payload: payload_bounds,
    }
}

fn append_payload(data: &mut [u8], payload: &[u8]) {
    if payload.len() > data.len() {
        data.copy_from_slice(&payload[..data.len()]);
    } else {
        data[..payload.len()].copy_from_slice(&payload)
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
                page_record_address: self.page_map.payload.begin,
                payload_record_address: 0,
                nb_pages_per_record: nb_pages_per_record,
                is_multi_pages_record: true}
        } else {
            let page_record_id = (record_id % nb_records_per_page as u64) as usize;
            RecordLocation{
                page_id: 1 + (record_id / nb_records_per_page as u64),
                record_id_in_page: page_record_id,
                page_record_address: self.page_map.payload.begin + self.record_size * page_record_id,
                payload_record_address: self.record_size * page_record_id,
                nb_pages_per_record: 0,
                is_multi_pages_record: false
            }
        }
    }

    pub fn load(&mut self, id: RecordId, data: &mut [u8]) {
        let location = self.compute_location(id);
        if location.is_multi_pages_record {
            let mut page_count = 0;
            let mut has_next_page = true;
            while has_next_page {
                let rpage = RecordPageWrapper::new(self.pager.load_page(&location.page_id), self.page_map);
                let payload = rpage.get_slice_ref(&self.page_map.payload);
                append_payload(&mut data[page_count*self.page_map.payload.size()..], payload);
                page_count += 1;
                has_next_page = rpage.has_multi_page_record();
            }
        } else {
            let rpage = RecordPageWrapper::new(self.pager.load_page(&location.page_id), self.page_map);
            let payload = rpage.get_slice_ref(&self.page_map.payload);
            data.copy_from_slice(&payload[location.payload_record_address..location.payload_record_address+self.record_size]);
        }
    }

    pub fn append(&mut self, data: &[u8]) -> RecordId {
        0
    }

    pub fn delete(&mut self, id: RecordId) {
        let loc = self.compute_location(id);
        let first_page = self.pager.load_page(&loc.page_id);
        if loc.is_multi_pages_record {
            //self.delete_multi_page_record(&first_page);
        } else {
            
        }
    }
}