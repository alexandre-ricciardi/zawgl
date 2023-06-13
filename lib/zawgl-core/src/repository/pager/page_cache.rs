// MIT License
// Copyright (c) 2023 Alexandre RICCIARDI
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

use super::super::super::buf_config::*;
use std::{collections::{HashMap}, cmp::Ordering};

pub type PageId = u64;
pub type HitsCount = u64;
pub type PageData = [u8; PAGE_SIZE];

#[derive(Debug, Clone, Copy)]
struct PageHits {
    hits: HitsCount,
    page_id: PageId,
    pos: usize,
    was_mut: bool,
}

impl PageHits {
    fn new(page_id: PageId, pos: usize) -> Self {
        PageHits { page_id, hits: 0, pos, was_mut: false }
    }
}

impl Ord for PageHits {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.was_mut && !other.was_mut {
            Ordering::Greater
        } else {
            self.hits.cmp(&other.hits)
        }
    }
}

impl PartialEq for PageHits {
    fn eq(&self, other: &PageHits) -> bool {
        self.hits == other.hits
    }
}

impl Eq for PageHits {
}

impl PartialOrd for PageHits {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.was_mut && !other.was_mut {
            Some(Ordering::Greater)
        } else {
            Some(self.hits.cmp(&other.hits))
        }
    }
}

pub struct PageCache {
    pages: Vec<PageData>,
    pages_map: HashMap<PageId, PageHits>,
    capacity: usize,
    dropped_pages: Vec<usize>,
}

impl PageCache {
    pub fn new(capacity: usize) -> Self {
        PageCache { capacity, pages: Vec::with_capacity(capacity), pages_map: HashMap::new(), dropped_pages: Vec::new() }
    }

    pub fn put(&mut self, data: PageData, page_id: PageId) -> Option<&mut PageData> {
        if self.dropped_pages.len() > 0 {
            self.dropped_pages.pop().and_then(|dp| {
                let page_hit = PageHits::new(page_id, dp);
                self.insert_page(data, page_hit)})
        } else if self.pages_map.contains_key(&page_id) {
            let ph = self.pages_map[&page_id];
            self.insert_page(data, PageHits::new(page_id, ph.pos))
        } else if self.pages.len() >= self.capacity && false {
            let mut hits: Vec<PageHits> = self.pages_map.values().map(|ph| ph.clone()).collect();
            hits.sort();
            let mut hit = hits[0];
            if hit.was_mut {
                self.append_page(data, page_id)
            } else {
                hit.page_id = page_id;
                self.insert_page(data, hit)
            }
        } else {
            self.append_page(data, page_id)
        }
    }

    fn insert_page(&mut self, data: PageData, page_hit: PageHits) -> Option<&mut PageData> {
        self.pages_map.insert(page_hit.page_id, page_hit);
        self.pages.insert(page_hit.pos, data);
        self.pages.get_mut(page_hit.pos)
    }

    fn append_page(&mut self, data: PageData, page_id: PageId) -> Option<&mut PageData> {
        let pos = self.pages.len();
        self.pages.push(data);
        self.pages_map.insert(page_id, PageHits::new(page_id, pos));
        self.pages.get_mut(pos)
    }

    pub fn get_ref(&mut self, page_id: &PageId) -> Option<&PageData> {
        self.inc_page_hits(page_id);
        self.pages_map.get(page_id).and_then(|pos| self.pages.get(pos.pos))
    }

    fn inc_page_hits(&mut self, page_id: &PageId) {
        if let Some(ph) = self.pages_map.get_mut(page_id) {
            ph.hits += 1;
        }
    }
    
    fn inc_mut_page_hits(&mut self, page_id: &PageId) {
        if let Some(ph) = self.pages_map.get_mut(page_id) {
            ph.hits += 1;
            ph.was_mut = true;
        }
    }
    
    pub fn contains_page_id(&self, pid: &PageId) -> bool {
        self.pages_map.contains_key(pid)
    }

    pub fn get_mut(&mut self, page_id: &PageId) -> Option<&mut PageData> {
        self.inc_mut_page_hits(page_id);
        self.pages_map.get(page_id).and_then(|pos| self.pages.get_mut(pos.pos))
    }

    pub fn get_page_ids(&self) -> Vec<PageId> {
        self.pages_map.keys().cloned().collect::<Vec<PageId>>()
    }

    pub fn drop_page(&mut self, pid: &PageId) {
        let ph = self.pages_map.get(pid);
        if let Some(page_hit) = ph {
            self.dropped_pages.push(page_hit.pos);
            self.pages_map.remove(pid);
        }
    }

}