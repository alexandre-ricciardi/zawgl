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

use super::super::buf_config::*;
use super::io::file_access::*;
use std::collections::HashMap;

pub type PageId = u64;

#[derive(Debug, Clone)]
pub enum PagerError {
    FileOverflow,
}

pub type PagerResult = std::result::Result<PageId, PagerError>;

pub struct HeaderPage {
    pub data: [u8; PAGE_SIZE],
}

impl  HeaderPage {
    fn new(data: [u8; PAGE_SIZE]) -> Self {
        HeaderPage{data: data}
    }

    pub fn get_page_count(&self) -> u64 {
        let mut bytes = [0u8; PAGE_COUNTER_SIZE];
        bytes.copy_from_slice(&self.data[..PAGE_COUNTER_SIZE]);
        u64::from_be_bytes(bytes)
    }

    fn set_page_count(&mut self, count: u64) {
        self.data[..PAGE_COUNTER_SIZE].copy_from_slice(&count.to_be_bytes());
    }
}

pub struct Page<'a> {
    pub id: PageId,
    pub header_page: &'a mut HeaderPage,
    pub data: &'a mut [u8; PAGE_SIZE],
}

impl <'a> Page<'a> {
    fn new(id: PageId, header_page: &'a mut HeaderPage, data: &'a mut [u8; PAGE_SIZE]) -> Self {
        Page{id: id, header_page: header_page, data: data}
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

    pub fn get_header_page_ref(&self) -> &HeaderPage {
        &self.header_page
    }
    
    fn read_page_data(&mut self, pid: PageId) -> [u8; PAGE_SIZE] {
        let mut page_data = [0u8; PAGE_SIZE];
        let page_begin_pos = pid * PAGE_SIZE as u64;
        self.records_file.read_at(page_begin_pos, &mut page_data);
        page_data
    }

    pub fn load_page(&mut self, pid: PageId) -> Option<Page> {
        if self.header_page.get_page_count() >= pid {
            if !self.page_cache.contains_key(&pid) {
                let page_data = self.read_page_data(pid);
                self.page_cache.insert(pid, page_data);
            }
            Some(Page::new(pid, &mut self.header_page, self.page_cache.get_mut(&pid).unwrap()))
        } else {
            None
        }
        
    }

    pub fn append(&mut self) -> Page {
        let next_pid = self.header_page.get_page_count() + 1;
        self.header_page.set_page_count(next_pid);
        let page_data = [0u8; PAGE_SIZE];
        self.page_cache.insert(next_pid, page_data);
        Page::new(next_pid, &mut self.header_page, self.page_cache.get_mut(&next_pid).unwrap())
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