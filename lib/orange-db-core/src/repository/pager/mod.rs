use super::io::file_access::*;

pub type PageId = u64;

#[derive(Debug, Clone)]
pub enum PagerError {
    FileOverflow,
}

pub type PagerResult = std::result::Result<PageId, PagerError>;

pub struct Pager {
    records_file: FileAccess,
    record_size: u64,
}

impl Pager {
    pub fn new(file: &str, record_size: u64) -> Self {
        Pager { records_file: FileAccess::new(file), record_size: record_size}
    }
    pub fn save(&mut self, id: PageId, data: &[u8]) -> PagerResult {
        let pos = id * self.record_size;
        if pos > self.records_file.get_file_len() {
            Err(PagerError::FileOverflow)
        } else {
            self.records_file.write_at(pos, &data);
            Ok(pos / self.record_size)
        }
    }
    pub fn load(&mut self, id: PageId, mut data: &mut [u8]) -> PagerResult {
        let pos = id * self.record_size;
        if pos > self.records_file.get_file_len() - self.record_size {
            Err(PagerError::FileOverflow)
        } else {
            self.records_file.read_at(pos, data);
            Ok(id)
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