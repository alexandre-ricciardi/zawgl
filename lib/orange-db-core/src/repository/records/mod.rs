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

struct HeaderPageWrapper<'a> {
    header_page: &'a mut HeaderPage,
    page_map: PageMap,

}

impl <'a> HeaderPageWrapper<'a> {

    fn new(header_page: &'a mut HeaderPage, page_map: PageMap) -> Self {
        HeaderPageWrapper{header_page: header_page, page_map: page_map}
    }
    
    fn get_header_slice_ref(&self, bounds: Bounds) -> &[u8] {
        &self.header_page.data[bounds.begin..bounds.end]
    }
    fn get_header_slice_mut(&mut self, bounds: Bounds) -> &mut [u8] {
        &mut self.header_page.data[bounds.begin..bounds.end]
    }
    fn get_header_first_free_page_ptr(&self) -> PageId {
        let mut bytes = [0u8; FIRST_FREE_PAGE_PTR];
        bytes.copy_from_slice(self.get_header_slice_ref(self.page_map.header_page_free_list_ptr));
        u64::from_be_bytes(bytes)
    }
    fn set_header_first_free_page_ptr(&mut self, id: u64) {
        let bounds = self.page_map.free_list_len;
        self.get_header_slice_mut(bounds).copy_from_slice(&id.to_be_bytes());
    }
}
struct RecordPageWrapper<'a> {
    page: Page<'a>,
    page_map: PageMap,
}

impl <'a> RecordPageWrapper<'a> {
    fn new(page: Page<'a>, page_map: PageMap) -> Self {
        RecordPageWrapper{page: page, page_map: page_map}
    }
    fn get_id(&self) -> PageId {
        self.page.id
    }
    fn get_header_page_wrapper(&mut self) -> HeaderPageWrapper {
        HeaderPageWrapper::new(&mut self.page.header_page, self.page_map)
    }
    fn has_next_page_record(&self) -> bool {
        self.page.data[0] & MULTI_PAGE_RECORD_FLAG == 1
    }
    fn is_free_page(&self) -> bool {
        self.page.data[0] & IS_FREE_PAGE_FLAG == 1
    }
    fn set_free_page(&mut self) {
        self.page.data[0] = self.page.data[0] & !IS_FREE_PAGE_FLAG;
    }
    fn set_page_in_use(&mut self) {
        self.page.data[0] = self.page.data[0] | IS_FREE_PAGE_FLAG;
    }
    fn get_free_next_page_ptr(&self) -> PageId {
        let mut bytes = [0u8; NEXT_PAGE_PTR];
        bytes.copy_from_slice(self.get_slice_ref(self.page_map.next_free_page_ptr));
        u64::from_be_bytes(bytes)
    }
    fn set_free_next_page_ptr(&mut self, id: u64) {
        let bounds = self.page_map.free_list_len;
        self.get_slice_mut(bounds).copy_from_slice(&id.to_be_bytes());
    }
    fn get_slice_ref(&self, bounds: Bounds) -> &[u8] {
        &self.page.data[bounds.begin..bounds.end]
    }
    fn get_slice_mut(&mut self, bounds: Bounds) -> &mut [u8] {
        &mut self.page.data[bounds.begin..bounds.end]
    }
    fn get_free_list_len(&self) -> usize {
        let mut bytes = [0u8; FREE_LIST_LEN_SIZE];
        bytes.copy_from_slice(self.get_slice_ref(self.page_map.free_list_len));
        u32::from_be_bytes(bytes) as usize
    }
    fn set_free_list_len(&mut self, len: usize) {
        let bounds = self.page_map.free_list_len;
        self.get_slice_mut(bounds).copy_from_slice(&(len as u32).to_be_bytes());
    }
    fn get_page_free_list(&self) -> Vec<PageRecordId> {
        let mut res = Vec::new();
        let free_list_slice = self.get_slice_ref(self.page_map.free_list);
        for count in 0..self.get_free_list_len() {
            let offset = count * FREE_LIST_PTR_SIZE;
            let mut bytes = [0u8; FREE_LIST_PTR_SIZE];
            bytes.copy_from_slice(&free_list_slice[offset..offset+FREE_LIST_PTR_SIZE]);
            res.push(u32::from_be_bytes(bytes) as usize);
        }
        res
    }
    fn init_page_free_list(&mut self) {
        let free_list_len = self.get_free_list_len();
        let mut free_list_slice = self.get_slice_mut(self.page_map.free_list);
        for count in 0..free_list_len {
            let offset = count * FREE_LIST_PTR_SIZE;
            let mut bytes = [0u8; FREE_LIST_PTR_SIZE];
            free_list_slice[offset..offset+FREE_LIST_PTR_SIZE].copy_from_slice(&count.to_be_bytes());
        }
    }
    fn pop_free_list_item(&mut self) -> PageRecordId {
        let free_list_len = self.get_free_list_len();
        self.set_free_list_len(free_list_len - 1);
        free_list_len - 1
    }
    fn append_free_list_item(&mut self, page_record_id: PageRecordId) {
        let mut free_records = self.get_page_free_list();
        free_records.push(page_record_id);
        free_records.sort();
        self.set_free_list_len(free_records.len());
        let mut bounds = self.page_map.free_list.shift(FREE_LIST_PTR_SIZE);
        for free_rec_id in free_records {
            self.get_slice_mut(bounds).copy_from_slice(&(free_rec_id as u32).to_be_bytes());
            bounds = bounds.shift(FREE_LIST_PTR_SIZE);
        }
    }
    fn is_page_free_list_empty(&self) -> bool {
        self.get_free_list_len() == 0
    }
}

const FREE_LIST_PTR_SIZE: usize = 4;
const FREE_LIST_LEN_SIZE: usize = 4;
const NEXT_PAGE_PTR: usize = 8;
const NEXT_FREE_PAGE_PTR: usize = 8;
const HEADER_FLAGS: usize = 1; 
const HEADER_SIZE: usize = HEADER_FLAGS + NEXT_FREE_PAGE_PTR + NEXT_PAGE_PTR;

const MULTI_PAGE_RECORD_FLAG: u8 = 0b1000_0000;
const IS_FREE_PAGE_FLAG: u8 = 0b0100_0000;

const FIRST_FREE_PAGE_PTR: usize = 8;

#[derive(Debug, Copy, Clone)]
struct Bounds {
    begin: usize,
    end: usize,
}

impl Bounds {
    fn new(b: usize, e: usize) -> Self {
        Bounds{begin: b, end: e}
    }
    fn from_offset_and_len(offset: usize, len: usize) -> Self {
        Bounds{begin: offset, end: offset + len}
    }
    fn len(&self) -> usize {
        self.end - self.begin
    }
    fn shift(&self, size: usize) -> Self {
        Bounds{begin: self.end, end: self.end + size}
    }
    fn sub(&self, offset: usize, len: usize) -> Self {
        Bounds{begin: offset + self.begin, end: offset + self.begin + len}
    }
}

#[derive(Debug, Copy, Clone)]
struct PageMap {
    header_flags: Bounds,
    next_free_page_ptr: Bounds,
    free_list_len: Bounds,
    free_list_capacity: usize,
    free_list: Bounds,
    payload: Bounds,
    nb_pages_per_record: usize,
    is_multi_page_record: bool,
    header_page_free_list_ptr: Bounds,
}

fn compute_freelist_len(record_size: usize) -> usize {
    (PAGE_SIZE - HEADER_SIZE) / record_size
}
fn compute_freelist_size(free_list_capacity: usize) -> usize {
    FREE_LIST_PTR_SIZE * free_list_capacity
}
fn compute_nb_pages_per_record(record_size: usize, page_payload_size: usize) -> usize {
    record_size / page_payload_size
}

fn compute_page_map(record_size: usize) -> PageMap {
    let free_list_capacity = compute_freelist_len(record_size);
    let free_list_size = compute_freelist_size(free_list_capacity);
    let header_flags_bounds = Bounds::new(0, HEADER_FLAGS);
    let next_free_page_ptr_bounds = header_flags_bounds.shift(NEXT_FREE_PAGE_PTR);
    let free_list_len = next_free_page_ptr_bounds.shift(FREE_LIST_LEN_SIZE);
    let free_list_bounds = free_list_len.shift(free_list_size);
    let payload_bounds = Bounds::new(free_list_bounds.end, PAGE_SIZE);
    let header_page_free_list_ptr_bounds = Bounds::new(PAGE_COUNTER, PAGE_COUNTER + FIRST_FREE_PAGE_PTR);
    let nb_pages_per_record = compute_nb_pages_per_record(record_size, payload_bounds.len());
    PageMap{
        header_flags: header_flags_bounds,
        next_free_page_ptr: next_free_page_ptr_bounds,
        free_list_len: free_list_len,
        free_list: free_list_bounds,
        free_list_capacity: free_list_capacity,
        nb_pages_per_record: nb_pages_per_record,
        is_multi_page_record: nb_pages_per_record == 0,
        payload: payload_bounds,
        header_page_free_list_ptr: header_page_free_list_ptr_bounds,
    }
}

fn copy_payload_to_buffer(data: &mut [u8], payload: &[u8]) {
    if payload.len() > data.len() {
        data.copy_from_slice(&payload[..data.len()]);
    } else {
        data[..payload.len()].copy_from_slice(&payload);
    }
}

fn copy_buffer_to_payload(payload: &mut [u8], data: &[u8]) {
    if payload.len() < data.len() {
        payload.copy_from_slice(&data[..payload.len()]);
    } else {
        payload[..data.len()].copy_from_slice(&data);
    }
}

impl RecordsManager {
    pub fn new(file: &str, rsize: usize) -> Self {
        RecordsManager{pager: Pager::new(file), record_size: rsize, page_map: compute_page_map(rsize)}
    }

    fn compute_location(&self, record_id: u64) -> RecordLocation {
        let page_payload_size = self.page_map.payload.len();
        let nb_records_per_page = page_payload_size / self.record_size;
        if self.page_map.is_multi_page_record {
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
            let mut page_count: usize = 0;
            let mut has_next_page = true;
            while has_next_page {
                let payload_bounds = self.page_map.payload;
                let rpage = self.load_page_wrapper(location.page_id + page_count as u64);
                let payload = rpage.get_slice_ref(payload_bounds);
                copy_payload_to_buffer(&mut data[page_count*payload_bounds.len()..], payload);
                page_count += 1;
                has_next_page = rpage.has_next_page_record();
            }
        } else {
            let payload_bounds = self.page_map.payload;
            let record_size = self.record_size;
            let rpage = self.load_page_wrapper(location.page_id);
            let payload = rpage.get_slice_ref(payload_bounds);
            data.copy_from_slice(&payload[location.payload_record_address..location.payload_record_address+record_size]);
        }
    }

    pub fn append(&mut self, data: &[u8]) -> RecordId {
        let free_list_capacity = self.page_map.free_list_capacity;
        let record_size = self.record_size;
        let nb_pages_per_record = self.page_map.nb_pages_per_record;
        let is_multi_page_record = self.page_map.is_multi_page_record;
        let payload_bounds = self.page_map.payload;
        let first_free_page_ptr = {
            self.get_header_page_wrapper().get_header_first_free_page_ptr()
        };
        let mut first = true;
        if first_free_page_ptr == 0 {
            if is_multi_page_record {
                for page_count in 0..nb_pages_per_record {
                    let new_page = self.pager.append();
                    let mut wrapper = RecordPageWrapper::new(new_page, self.page_map);
                    if first {
                        let first_page_id = wrapper.get_id();
                        wrapper.get_header_page_wrapper().set_header_first_free_page_ptr(first_page_id);
                        first = false;
                    }
                    copy_buffer_to_payload(wrapper.get_slice_mut(payload_bounds), &data[page_count*payload_bounds.len()..]);
                }
            } else {
                let new_page = self.pager.append();
                let mut wrapper = RecordPageWrapper::new(new_page, self.page_map);
                wrapper.init_page_free_list();
                let page_id = wrapper.get_id();
                wrapper.get_header_page_wrapper().set_header_first_free_page_ptr(page_id);
                let page_record_id = wrapper.pop_free_list_item();
                wrapper.get_slice_mut(payload_bounds.sub(page_record_id * self.record_size, self.record_size)).copy_from_slice(&data);
            }
        } else {
            if is_multi_page_record {
                let mut wrapper = self.load_page_wrapper(first_free_page_ptr);
                let next_free_page_ptr = wrapper.get_free_next_page_ptr();
                wrapper.get_header_page_wrapper().set_header_first_free_page_ptr(next_free_page_ptr);
                for page_count in 0..nb_pages_per_record {
                    copy_buffer_to_payload(wrapper.get_slice_mut(payload_bounds), &data[page_count*payload_bounds.len()..]);
                }
            } else {
                let mut wrapper = self.load_page_wrapper(first_free_page_ptr);
                let page_record_id = wrapper.pop_free_list_item();
                wrapper.get_slice_mut(payload_bounds.sub(page_record_id * record_size, record_size)).copy_from_slice(&data);
                if wrapper.is_page_free_list_empty() {
                    let next_free_page_ptr = wrapper.get_free_next_page_ptr();
                    wrapper.get_header_page_wrapper().set_header_first_free_page_ptr(next_free_page_ptr);
                }
            }
        }
        0
    }

    fn make_page_wrapper<'a>(&self, page: Page<'a>) -> RecordPageWrapper<'a> {
        RecordPageWrapper::new(page, self.page_map)
    }

    fn load_page_wrapper(&mut self, id: PageId) -> RecordPageWrapper {
        let page = self.pager.load_page(id);
        RecordPageWrapper::new(page, self.page_map)
    }

    fn get_header_page_wrapper(&mut self) -> HeaderPageWrapper {
        HeaderPageWrapper::new(self.pager.get_header_page_mut(), self.page_map)
    }

    pub fn delete(&mut self, id: RecordId) {
        let free_list_capacity = self.page_map.free_list_capacity;
        let loc = self.compute_location(id);
        let mut rpage = self.load_page_wrapper(loc.page_id);
        let mut append_page_to_free_list = true;
        if loc.is_multi_pages_record {
            rpage.set_free_page();
        } else {
            if rpage.get_free_list_len() == free_list_capacity {
                rpage.set_free_page();
            } else {
                //do not append page to free list more than once
                append_page_to_free_list = false;
            }
            rpage.append_free_list_item(loc.record_id_in_page);
        }

        if append_page_to_free_list {
            let first_free_page_ptr = {
                let header_page_wrapper = rpage.get_header_page_wrapper();
                header_page_wrapper.get_header_first_free_page_ptr()
            };
            rpage.set_free_next_page_ptr(first_free_page_ptr);
            rpage.get_header_page_wrapper().set_header_first_free_page_ptr(loc.page_id);
        }
    }
}