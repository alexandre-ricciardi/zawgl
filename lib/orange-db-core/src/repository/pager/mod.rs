use super::io::file_access::*;
use std::collections::HashMap;

pub type PageId = u64;

#[derive(Debug, Clone)]
pub enum PagerError {
    FileOverflow,
}

pub type PagerResult = std::result::Result<PageId, PagerError>;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_COUNTER: usize = 8;

pub struct HeaderPage {
    pub data: [u8; PAGE_SIZE],
}

impl  HeaderPage {
    fn new(data: [u8; PAGE_SIZE]) -> Self {
        HeaderPage{data: data}
    }

    fn get_page_count(&self) -> u64 {
        let mut bytes = [0u8; PAGE_COUNTER];
        bytes.copy_from_slice(&self.data[..PAGE_COUNTER]);
        u64::from_be_bytes(bytes)
    }

    fn set_page_count(&mut self, count: u64) {
        self.data[..8].copy_from_slice(&count.to_be_bytes());
    }
}

pub struct Page<'a> {
    pub header_page: &'a mut HeaderPage,
    pub data: &'a mut [u8; PAGE_SIZE],
}

impl <'a> Page<'a> {
    fn new(header_page: &'a mut HeaderPage, data: &'a mut [u8; PAGE_SIZE]) -> Self {
        Page{header_page: header_page, data: data}
    }
}

pub struct Pager {
    records_file: FileAccess,
    page_cache: HashMap<PageId, [u8; PAGE_SIZE]>,
    nb_pages: u64,
    header_page: HeaderPage,
}


fn load_or_create_header_page(io: &mut FileAccess) -> HeaderPage {
    let mut header_page_data = [0u8; PAGE_SIZE];
    if io.get_file_len() == 0 {
        io.write_at(0, &header_page_data);
    } else {
        io.read_at(0, &mut header_page_data);
    }
    HeaderPage{data: header_page_data}
}

impl Pager {
    pub fn new(file: &str) -> Self {
        let mut file_io = FileAccess::new(file);
        let header_page = load_or_create_header_page(&mut file_io);
        Pager { records_file: file_io, page_cache: HashMap::new(), nb_pages: 0u64, header_page: header_page}
    }

    pub fn get_header_page_mut(&mut self) -> &mut HeaderPage {
        &mut self.header_page
    }

    fn read_page_data(&mut self, pid: &PageId) -> [u8; PAGE_SIZE] {
        let mut page_data = [0u8; PAGE_SIZE];
        let page_begin_pos = *pid * PAGE_SIZE as u64;
        self.records_file.read_at(page_begin_pos, &mut page_data);
        page_data
    }

    pub fn load_page(&mut self, pid: &PageId) -> Page {
        if !self.page_cache.contains_key(pid) {
            let page_data = self.read_page_data(pid);
            self.page_cache.insert(*pid, page_data);
        }
        Page::new(&mut self.header_page,self.page_cache.get_mut(pid).unwrap())
    }

    pub fn append(&mut self) -> Page {
        let next_pid = self.header_page.get_page_count() + 1;
        self.header_page.set_page_count(next_pid);
        let page_data = [0u8; PAGE_SIZE];
        self.page_cache.insert(next_pid, page_data);
        Page::new(&mut self.header_page, self.page_cache.get_mut(&next_pid).unwrap())
    }
    
    pub fn sync(&mut self) {
        self.records_file.write_at(0, &self.header_page.data);
        let mut pids = self.page_cache.keys().cloned().collect::<Vec<PageId>>();
        pids.sort();
        for pid in pids {
            let pos = pid * PAGE_SIZE as u64;
            self.records_file.write_at(pos, self.page_cache.get(&pid).unwrap());
        }
    }
}