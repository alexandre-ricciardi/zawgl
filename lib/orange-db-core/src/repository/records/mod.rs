use super::pager::*;

pub type RecordId = u64;
pub type PageRecordId = usize;

pub struct RecordsManager {
    pager: Pager,
    record_size: usize,
}

const FREE_LIST_PTR_SIZE: usize = 4;
const HEADER_SIZE: usize = 8;

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

    fn compute_location(&self, record_id: u64) -> (PageId, PageRecordId) {
        let nb_records_per_page = compute_payload_size(self.record_size) / self.record_size;
        (1 + (record_id / nb_records_per_page as u64), (record_id % nb_records_per_page as u64) as usize)
    }

    pub fn load(&mut self, id: RecordId, mut data: &mut [u8]) {
        let location = self.compute_location(id);
        let page = self.pager.load_page(&location.0);
        let location_in_page = location.1 * self.record_size;
        data.copy_from_slice(&page.data[location_in_page..location_in_page + self.record_size]);
    }

    pub fn append(&mut self, data: &[u8]) -> RecordId {
        0
    }
}