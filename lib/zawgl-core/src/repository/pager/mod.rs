// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod page_cache;

use self::page_cache::{PageId, PageCache, PageData};

use super::super::buf_config::*;
use super::io::file_access::*;

#[derive(Debug, Clone)]
pub enum PagerError {
    FileOverflow,
}
pub const IS_PAGE_IN_USE_FLAG: u8 = 0b0100_0000;
pub type PagerResult = std::result::Result<PageId, PagerError>;
pub type CountValue = u64;
pub struct HeaderPage {
    pub page: PageData,
    header_page_map: HeaderPageMap,
}


fn make_empty_heap_page() -> PageData {
    [0u8; PAGE_SIZE]
}

impl HeaderPage {
    fn new(data: PageData) -> Self {
        HeaderPage{page: data, header_page_map: HeaderPageMap::new()}
    }

    pub fn get_page_count(&self) -> u64 {
        let mut bytes = [0u8; PAGE_COUNTER_SIZE];
        bytes.copy_from_slice(&self.page[..PAGE_COUNTER_SIZE]);
        u64::from_be_bytes(bytes)
    }

    pub fn set_page_count(&mut self, count: u64) {
        self.page[..PAGE_COUNTER_SIZE].copy_from_slice(&count.to_be_bytes());
    }
    
    pub fn get_header_first_free_page_ptr(&self) -> PageId {
        let mut bytes = [0u8; FIRST_FREE_PAGE_PTR];
        self.read_header_from_bounds(self.header_page_map.header_page_free_list_ptr, &mut bytes);
        u64::from_be_bytes(bytes)
    }
    pub fn set_header_first_free_page_ptr(&mut self, id: u64) {
        let bounds = self.header_page_map.header_page_free_list_ptr;
        self.write_header_to_bounds(bounds, &id.to_be_bytes());
    }
    
    pub fn get_header_records_counter(&self) -> CountValue {
        let mut bytes = [0u8; RECORDS_COUNTER_SIZE];
        self.read_header_from_bounds(self.header_page_map.header_page_records_counter, &mut bytes);
        u64::from_be_bytes(bytes)
    }
    pub fn set_header_records_counter(&mut self, id: CountValue) {
        let bounds = self.header_page_map.header_page_records_counter;
        self.write_header_to_bounds(bounds, &id.to_be_bytes());
    }
    
    pub fn get_header_records_version_counter(&self) -> CountValue {
        let mut bytes = [0u8; RECORDS_COUNTER_SIZE];
        self.read_header_from_bounds(self.header_page_map.header_page_records_version_counter, &mut bytes);
        u64::from_be_bytes(bytes)
    }
    
    pub fn set_header_records_version_counter(&mut self, id: CountValue) {
        let bounds = self.header_page_map.header_page_records_version_counter;
        self.write_header_to_bounds(bounds, &id.to_be_bytes());
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_page_count() == 0
    }
    
    pub fn read_header_from_bounds(&self, bounds: Bounds, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.page[bounds.begin..bounds.end]);
    }
    
    pub fn write_header_to_bounds(&mut self, bounds: Bounds, bytes: &[u8]) {
        self.page[bounds.begin..bounds.end].copy_from_slice(bytes);
    }
    
    pub fn read_header_payload_from_bounds(&self, bounds: Bounds, bytes: &mut [u8]) {
        let payload_bounds = self.header_page_map.header_page_payload.sub(bounds.begin, bounds.len());
        let slice = &self.page[payload_bounds.begin..payload_bounds.end];
        bytes.copy_from_slice(slice);
    }
    
    pub fn write_header_payload_to_bounds(&mut self, bounds: Bounds, bytes: &[u8]) {
        let payload_bounds = self.header_page_map.header_page_payload.sub(bounds.begin, bounds.len());
        self.page[payload_bounds.begin..payload_bounds.end].copy_from_slice(bytes);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub begin: usize,
    pub end: usize,
}

impl Bounds {
    pub fn new(b: usize, e: usize) -> Self {
        Bounds{begin: b, end: e}
    }
    pub fn from_offset_and_len(offset: usize, len: usize) -> Self {
        Bounds{begin: offset, end: offset + len}
    }
    pub fn len(&self) -> usize {
        self.end - self.begin
    }
    pub fn shift(&self, size: usize) -> Self {
        Bounds{begin: self.end, end: self.end + size}
    }
    pub fn sub(&self, offset: usize, len: usize) -> Self {
        Bounds{begin: offset + self.begin, end: offset + self.begin + len}
    }
}

struct HeaderPageMap {
    header_page_free_list_ptr: Bounds,
    header_page_records_counter: Bounds,
    header_page_records_version_counter: Bounds,
    header_page_payload: Bounds,
}

impl HeaderPageMap {
    pub fn new() -> Self {
        let header_page_free_list_ptr_bounds = Bounds::new(PAGE_COUNTER_SIZE, PAGE_COUNTER_SIZE + FIRST_FREE_PAGE_PTR);
        let header_page_records_counter_bounds = header_page_free_list_ptr_bounds.shift(RECORDS_COUNTER_SIZE);
        let header_page_records_version_counter_bounds = header_page_records_counter_bounds.shift(RECORDS_COUNTER_SIZE);
        let header_page_payload_bounds = Bounds::new(header_page_records_counter_bounds.end, PAGE_SIZE);
        HeaderPageMap {
            header_page_free_list_ptr: header_page_free_list_ptr_bounds,
            header_page_records_counter: header_page_records_counter_bounds,
            header_page_records_version_counter: header_page_records_version_counter_bounds,
            header_page_payload: header_page_payload_bounds,
        }
    }
    
}

pub struct Pager {
    records_file: FileAccess,
    page_cache: PageCache,
    header_page: HeaderPage,
}


fn load_or_create_header_page(io: &mut FileAccess) -> HeaderPage {
    let mut header_page_data = make_empty_heap_page();
    if io.get_file_len() == 0 {
        io.write_at(0, &header_page_data);
    } else {
        let bmut_data = &mut header_page_data;
        io.read_at(0, bmut_data);
    }
    HeaderPage{page: header_page_data, header_page_map: HeaderPageMap::new()}
}

impl Pager {
    pub fn new(file: &str) -> Self {
        let mut file_io = FileAccess::new(file);
        let header_page = load_or_create_header_page(&mut file_io);
        Pager { records_file: file_io, page_cache: PageCache::new(1000), header_page }
    }

    pub fn get_header_page_ref(&self) -> &HeaderPage {
        &self.header_page
    }
    
    pub fn get_header_page_mut(&mut self) -> &mut HeaderPage {
        &mut self.header_page
    }
    
    fn read_page_data(&mut self, pid: PageId) -> [u8; PAGE_SIZE] {
        let mut page_data = [0u8; PAGE_SIZE];
        let page_begin_pos = pid * PAGE_SIZE as u64;
        self.records_file.read_at(page_begin_pos, &mut page_data);
        page_data
    }

    fn read_page_header(&mut self, pid: PageId) -> [u8; 1] {
        let mut page_data = [0u8];
        let page_begin_pos = pid * PAGE_SIZE as u64;
        self.records_file.read_at(page_begin_pos, &mut page_data);
        page_data
    }

    pub fn load_page(&mut self, pid: PageId) -> Option<&mut PageData> {
        let nb_pages = self.header_page.get_page_count();
        if nb_pages >= pid {
            if self.page_cache.contains_page_id(&pid) {
                self.page_cache.get_mut(&pid)
            } else {
                let data = self.read_page_data(pid);
                self.page_cache.put(data, pid)
            }
        } else {
            None
        }
    }

    pub fn fetch_non_empty_page(&mut self, pid: PageId) -> Option<PageData> {
        if self.page_cache.contains_page_id(&pid) {
            Some(self.page_cache.get_ref(&pid).unwrap().clone())
        } else {
            let nb_pages = self.header_page.get_page_count();
            if nb_pages >= pid {
                let header = self.read_page_header(pid);
                if (header[0] & IS_PAGE_IN_USE_FLAG) > 0 {
                    Some(self.read_page_data(pid))
                } else { 
                    None
                }
            } else {
                None
            }
        }
    }

    pub fn drop_page(&mut self, pid: &PageId) {
        self.page_cache.drop_page(pid);
    }

    pub fn append(&mut self, nb_pages_to_create: usize) -> Vec<PageId> {
        let mut pages = Vec::new();
        for _ in 0..nb_pages_to_create {
            pages.push(self._append_one_page());
        }
        pages
    }

    fn _append_one_page(&mut self) -> PageId {
        let next_pid = self.header_page.get_page_count() + 1;
        self.page_cache.put([0u8; PAGE_SIZE], next_pid);
        self.header_page.set_page_count(next_pid);
        next_pid
    }
    
    pub fn sync(&mut self) {
        self.records_file.write_at(0, &self.header_page.page);
        let pages = self.page_cache.get_mut_pages_ref();
        for p in pages {
            let pos = p.0 * PAGE_SIZE as u64;
            self.records_file.write_at(pos, p.1);
        }
    }
    fn copy_from_bounds(&self, bounds: Bounds, bytes: &mut [u8]) {
        let slice = &self.header_page.page[bounds.begin..bounds.end];
        bytes.copy_from_slice(slice);
    }
    pub fn clear(&mut self) {
        self.page_cache.clear();
    }
    pub fn erase(&mut self) {
        self.records_file.erase();
    }
}