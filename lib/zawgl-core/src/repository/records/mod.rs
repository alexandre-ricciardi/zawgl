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

use super::pager::*;
use super::pager::page_cache::{PageId, PageData};

pub type RecordId = u64;
pub type PageRecordId = usize;

#[derive(Debug, Clone)]
pub enum RecordsManagerError {
    NotFound,
}

pub type RecordsManagerResult<T> = std::result::Result<T, RecordsManagerError>;

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
    nb_records_per_page: usize,
    page_map: PageMap,
}

struct RecordPageWrapper<'a> {
    page: &'a mut PageData,
    page_map: PageMap,
    page_id: PageId,
}

impl<'a> RecordPageWrapper<'a> {
    fn new(page_id: PageId, page: &'a mut PageData, page_map: PageMap) -> Self {
        RecordPageWrapper{page, page_map, page_id}
    }
    fn get_id(&self) -> PageId {
        self.page_id
    }
    fn has_next_page_record(&self) -> bool {
        (self.page[0] & MULTI_PAGE_RECORD_FLAG) > 0
    }
    fn set_page_in_use(&mut self) {
        self.page[0] |= IS_PAGE_IN_USE_FLAG;
    }
    fn is_page_in_use(&self) -> bool {
        (self.page[0] & IS_PAGE_IN_USE_FLAG) > 0
    }
    fn get_free_next_page_ptr(&self) -> PageId {
        let mut bytes = [0u8; NEXT_PAGE_PTR];
        self.read_from_bounds(self.page_map.next_free_page_ptr, &mut bytes);
        u64::from_be_bytes(bytes)
    }
    fn set_free_next_page_ptr(&mut self, id: u64) {
        let bounds = self.page_map.next_free_page_ptr;
        self.write_to_bounds(bounds, &id.to_be_bytes());
    }
    fn read_from_bounds(&self, bounds: Bounds, bytes: &mut [u8]) {
        let slice = &self.page[bounds.begin..bounds.end];
        bytes.copy_from_slice(slice);
    }
    fn write_to_bounds(&mut self, bounds: Bounds, bytes: &[u8]) {
        self.page[bounds.begin..bounds.end].copy_from_slice(bytes);
    }
    fn get_free_list_len(&self) -> usize {
        let mut bytes = [0u8; FREE_LIST_ITEM_COUNTER_SIZE];
        self.read_from_bounds(self.page_map.free_list_len, &mut bytes);
        u32::from_be_bytes(bytes) as usize
    }
    fn set_free_list_len(&mut self, len: usize) {
        let bounds = self.page_map.free_list_len;
        self.write_to_bounds(bounds, &(len as u32).to_be_bytes());
    }
    fn get_page_free_list(&self) -> Vec<PageRecordId> {
        let mut res = Vec::new();
        let free_list_slice = &self.page[self.page_map.free_list.begin..self.page_map.free_list.end];
        let free_list_len = self.get_free_list_len();
        for count in 0..free_list_len {
            let offset = count * FREE_LIST_PTR_SIZE;
            let mut bytes = [0u8; FREE_LIST_PTR_SIZE];
            bytes.copy_from_slice(&free_list_slice[offset..offset+FREE_LIST_PTR_SIZE]);
            res.push(u32::from_be_bytes(bytes) as usize);
        }
        res
    }
    fn init_page_free_list(&mut self) {
        let free_list_len = self.page_map.nb_records_per_page;
        self.set_free_list_len(free_list_len);
        let free_list_bounds = self.page_map.free_list;
        for count in 0..free_list_len {
            let offset = count * FREE_LIST_PTR_SIZE;
            let free_list_ptr = count as u32;
            self.write_to_bounds(free_list_bounds.sub(offset, FREE_LIST_PTR_SIZE), &free_list_ptr.to_be_bytes());
        }
    }
    fn pop_free_list_item(&mut self) -> Option<PageRecordId> {
        let free_list_len = self.get_free_list_len();
        if free_list_len > 0 {
            let res = self.get_page_free_list().pop();
            self.set_free_list_len(free_list_len - 1);
            res
        } else {
            None
        }
        
    }
    fn append_free_list_item(&mut self, page_record_id: PageRecordId) {
        let mut free_records = self.get_page_free_list();
        free_records.push(page_record_id);
        free_records.sort();
        self.set_free_list_len(free_records.len());
        let mut bounds = self.page_map.free_list.shift(FREE_LIST_PTR_SIZE);
        for free_rec_id in free_records {
            self.write_to_bounds(bounds, &(free_rec_id as u32).to_be_bytes());
            bounds = bounds.shift(FREE_LIST_PTR_SIZE);
        }
    }
    fn is_page_free_list_empty(&self) -> bool {
        self.get_free_list_len() == 0
    }
}

const MULTI_PAGE_RECORD_FLAG: u8 = 0b1000_0000;



#[derive(Debug, Copy, Clone)]
struct PageMap {
    header_flags: Bounds,
    next_free_page_ptr: Bounds,
    free_list_len: Bounds,
    free_list_capacity: usize,
    free_list: Bounds,
    payload: Bounds,
    nb_pages_per_record: usize,
    nb_records_per_page: usize,
    is_multi_page_record: bool,
}

const fn max_nb_records(record_size: usize) -> usize {
    (PAGE_SIZE - HEADER_SIZE) / record_size
}

const fn compute_unused_page_size(record_size: usize) -> usize {
    (PAGE_SIZE - HEADER_SIZE) % record_size
}

const fn compute_freelist_size(free_list_capacity: usize) -> usize {
    FREE_LIST_PTR_SIZE * free_list_capacity
}
const fn compute_nb_pages_per_record(record_size: usize, page_payload_size: usize) -> usize {
    record_size / page_payload_size
}

fn compute_page_map(nb_records_per_page: usize, nb_pages_per_record: usize) -> PageMap {
    let free_list_size = compute_freelist_size(nb_records_per_page);
    //TODO handle all cases
    let header_flags_bounds = Bounds::new(0, HEADER_FLAGS);
    let next_free_page_ptr_bounds = header_flags_bounds.shift(NEXT_FREE_PAGE_PTR);
    let free_list_len = next_free_page_ptr_bounds.shift(FREE_LIST_ITEM_COUNTER_SIZE);
    let free_list_bounds = free_list_len.shift(free_list_size);
    let payload_bounds = Bounds::new(free_list_bounds.end, PAGE_SIZE);
    PageMap{
        header_flags: header_flags_bounds,
        next_free_page_ptr: next_free_page_ptr_bounds,
        free_list_len,
        free_list: free_list_bounds,
        free_list_capacity: nb_records_per_page,
        nb_pages_per_record,
        nb_records_per_page,
        is_multi_page_record: nb_pages_per_record != 0,
        payload: payload_bounds,
    }
}

fn copy_payload_to_buffer(data: &mut [u8], payload: &[u8]) {
    if payload.len() > data.len() {
        data.copy_from_slice(&payload[..data.len()]);
    } else {
        data[..payload.len()].copy_from_slice(payload);
    }
}

fn copy_buffer_to_payload(wrapper: &mut RecordPageWrapper, payload: Bounds, data: &[u8]) {
    if payload.len() < data.len() {
        wrapper.write_to_bounds(payload, &data[..payload.len()])
    } else {
        wrapper.write_to_bounds(Bounds::new(0, data.len()), data)
    }
}

impl RecordsManager {
    pub fn new(file: &str, record_size: usize, nb_records_per_page: usize, nb_pages_per_record: usize) -> Self {
        RecordsManager{pager: Pager::new(file), record_size, nb_records_per_page, page_map: compute_page_map(nb_records_per_page, nb_pages_per_record)}
    }

    fn compute_location(&self, record_id: u64) -> RecordLocation {
        let record_ptr = record_id - 1;
        let page_payload_size = self.page_map.payload.len();
        let nb_records_per_page = page_payload_size / self.record_size;
        if self.page_map.is_multi_page_record {
            let nb_pages_per_record = self.record_size / page_payload_size;
            RecordLocation{
                page_id: 1 + (nb_pages_per_record as u64 * record_ptr),
                record_id_in_page: 0,
                page_record_address: self.page_map.payload.begin,
                payload_record_address: 0,
                nb_pages_per_record,
                is_multi_pages_record: true}
        } else {
            let page_record_id = (record_ptr % nb_records_per_page as u64) as usize;
            RecordLocation{
                page_id: 1 + (record_ptr / nb_records_per_page as u64),
                record_id_in_page: page_record_id,
                page_record_address: self.page_map.payload.begin + self.record_size * page_record_id,
                payload_record_address: self.record_size * page_record_id,
                nb_pages_per_record: 0,
                is_multi_pages_record: false
            }
        }
    }

    fn configure_free_pages(&mut self, mut pages: Vec<PageId>) -> RecordsManagerResult<PageId> {
        pages.reverse();
        let mut curr_pid = 0u64;
        let mut res = Err(RecordsManagerError::NotFound);
        for page_id in pages {
            let page = self.pager.load_page(page_id);
            let mut wrapper = page.map(|p| RecordPageWrapper::new(page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
            wrapper.set_free_next_page_ptr(curr_pid);
            wrapper.init_page_free_list();
            wrapper.set_page_in_use();
            curr_pid = page_id;
            res = Ok(page_id)
        }
        res
    }
    
    pub fn load(&mut self, id: RecordId, data: &mut [u8]) -> RecordsManagerResult<()> {
        let location = self.compute_location(id);
        if location.is_multi_pages_record {
            let mut page_count: usize = 0;
            let mut has_next_page = true;
            while has_next_page {
                let payload_bounds = self.page_map.payload;
                {
                    let page  = self.pager.load_page(location.page_id).ok_or(RecordsManagerError::NotFound)?;
                    let current_payload_slice = &page[payload_bounds.begin..payload_bounds.end];
                    copy_payload_to_buffer(&mut data[page_count*payload_bounds.len()..], current_payload_slice);
                }
                let page = self.pager.load_page(location.page_id);
                let wrapper = page.map(|p| RecordPageWrapper::new(location.page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
                page_count += 1;
                has_next_page = wrapper.has_next_page_record();
            }
            Ok(())
        } else {
            let payload_bounds = self.page_map.payload;
            let record_size = self.record_size;
            let page = self.pager.load_page(location.page_id).ok_or(RecordsManagerError::NotFound)?;
            let current_payload_slice = &page[payload_bounds.begin..payload_bounds.end];
            data.copy_from_slice(&current_payload_slice[location.payload_record_address..location.payload_record_address+record_size]);
            Ok(())
        }
    }

    pub fn save(&mut self, id: RecordId, data: &[u8]) -> RecordsManagerResult<()> {
        let location = self.compute_location(id);
        let payload_bounds = self.page_map.payload;
        let nb_pages_per_record = self.page_map.nb_pages_per_record;
        let record_size = self.record_size;
        if location.is_multi_pages_record {
            {
                let page = self.pager.load_page(location.page_id);
                let next_free_page_ptr = {
                    let wrapper = page.map(|p| RecordPageWrapper::new(location.page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
                    wrapper.get_free_next_page_ptr()
                };
                self.pager.get_header_page_mut().set_header_first_free_page_ptr(next_free_page_ptr);
            }
            let page = self.pager.load_page(location.page_id);
            let mut wrapper = page.map(|p| RecordPageWrapper::new(location.page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
            wrapper.set_page_in_use();
            for page_count in 0..nb_pages_per_record {
                copy_buffer_to_payload(&mut wrapper, payload_bounds, &data[page_count*payload_bounds.len()..]);
            }
        } else {
            let mut page = self.pager.load_page(location.page_id);
            let mut wrapper = page.as_mut().map(|p| RecordPageWrapper::new(location.page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
            wrapper.set_page_in_use();
            wrapper.write_to_bounds(payload_bounds.sub(location.record_id_in_page * record_size, record_size), data);
            if wrapper.is_page_free_list_empty() {
                let mut next_free_page_ptr = self.pager.get_header_page_mut().get_header_first_free_page_ptr();
                let mut has_free_slot = false;
                while !has_free_slot && next_free_page_ptr != 0 {
                    let curr_free_page_candidate = self.pager.load_page(next_free_page_ptr);
                    let wrapper_free_page_candidate = curr_free_page_candidate.map(|p| RecordPageWrapper::new(next_free_page_ptr, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
                    has_free_slot = !wrapper_free_page_candidate.is_page_free_list_empty();
                    if has_free_slot {
                        self.pager.get_header_page_mut().set_header_first_free_page_ptr(next_free_page_ptr);
                    } else {
                        next_free_page_ptr = wrapper_free_page_candidate.get_free_next_page_ptr();
                    }
                }
                
            }
        }
        self.increment_records_version_counter();
        Ok(())
    }

    fn increment_records_version_counter(&mut self) {
        let next = self.pager.get_header_page_ref().get_header_records_version_counter() + 1;
        self.pager.get_header_page_mut().set_header_records_counter(next);
    }

    fn increment_records_counter(&mut self) {
        let next = self.pager.get_header_page_ref().get_header_records_counter() + 1;
        self.pager.get_header_page_mut().set_header_records_counter(next);
    }

    fn decrement_records_counter(&mut self) {
        let next = self.pager.get_header_page_ref().get_header_records_counter() - 1;
        self.pager.get_header_page_mut().set_header_records_counter(next);
    }

    pub fn create(&mut self, data: &[u8]) -> RecordsManagerResult<RecordId> {
        let record_size = self.record_size;
        let nb_pages_per_record = self.page_map.nb_pages_per_record;
        let nb_records_per_page = self.page_map.nb_records_per_page;
        let is_multi_page_record = self.page_map.is_multi_page_record;
        let payload_bounds = self.page_map.payload;
        let first_free_page_ptr = self.pager.get_header_page_ref().get_header_first_free_page_ptr();
        let mut record_id = 0;
        if first_free_page_ptr == 0 {
            if is_multi_page_record {
                let mut first = true;
                for page_count in 0..nb_pages_per_record {
                    let pages = self.pager.append(10);
                    let page_id = self.configure_free_pages(pages)?;
                    let page = self.pager.load_page(page_id);
                    let mut wrapper = page.map(|p| RecordPageWrapper::new(page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
                    copy_buffer_to_payload(&mut wrapper, payload_bounds, &data[page_count*payload_bounds.len()..]);
                    if first {
                        let first_page_id = wrapper.get_id();
                        record_id = (first_page_id - 1) / nb_pages_per_record as u64;
                        self.pager.get_header_page_mut().set_header_first_free_page_ptr(first_page_id);
                        first = false;
                    }
                }
            } else {
                let pages = self.pager.append(10);
                let page_id = self.configure_free_pages(pages)?;
                let page = self.pager.load_page(page_id);
                let mut wrapper = page.map(|p| RecordPageWrapper::new(page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
                let page_id = wrapper.get_id();
                
                let opage_record_id = wrapper.pop_free_list_item();
                if let Some(page_record_id) = opage_record_id {
                    record_id = (wrapper.get_id() - 1) * nb_records_per_page as u64 + page_record_id as u64;
                    wrapper.write_to_bounds(payload_bounds.sub(page_record_id * self.record_size, self.record_size), data)
                }
                if wrapper.get_free_list_len() > 0 {
                    self.pager.get_header_page_mut().set_header_first_free_page_ptr(page_id);
                } else {
                    let next_free_page_id = wrapper.get_free_next_page_ptr();
                    self.pager.get_header_page_mut().set_header_first_free_page_ptr(next_free_page_id);
                }
            }
        } else if is_multi_page_record {
                
            let next_free_page_ptr = {
                let page = self.pager.load_page(first_free_page_ptr);
                let wrapper = page.map(|p| RecordPageWrapper::new(first_free_page_ptr, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
                wrapper.get_free_next_page_ptr()                    
            };
            self.pager.get_header_page_mut().set_header_first_free_page_ptr(next_free_page_ptr);
            let page = self.pager.load_page(first_free_page_ptr);
            let mut wrapper = page.map(|p| RecordPageWrapper::new(first_free_page_ptr, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
            let mut first = true;
            for page_count in 0..nb_pages_per_record {
                if first {
                    record_id = (wrapper.get_id() - 1) / nb_pages_per_record as u64;
                    first = false;
                }
                copy_buffer_to_payload(&mut wrapper, payload_bounds, &data[page_count*payload_bounds.len()..]);
            }
        } else {
            let mut page = self.pager.load_page(first_free_page_ptr);
            let mut wrapper = page.as_mut().map(|p| RecordPageWrapper::new(first_free_page_ptr, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
            let page_record_id = wrapper.pop_free_list_item().ok_or(RecordsManagerError::NotFound)?;
            record_id = (wrapper.get_id() - 1) * nb_records_per_page as u64 + page_record_id as u64;
            wrapper.write_to_bounds(payload_bounds.sub(page_record_id * record_size, record_size), data);
            if wrapper.is_page_free_list_empty() {
                let next_free_page_ptr = wrapper.get_free_next_page_ptr();
                self.pager.get_header_page_mut().set_header_first_free_page_ptr(next_free_page_ptr);
            }
        }
    
        self.increment_records_version_counter();
        self.increment_records_counter();
        Ok(record_id + 1)
    }

    pub fn delete(&mut self, id: RecordId) -> RecordsManagerResult<()> {
        let free_list_capacity = self.page_map.free_list_capacity;
        let loc = self.compute_location(id);
        let append_page_to_free_list = {
            let mut page = self.pager.load_page(loc.page_id);
            let mut wrapper = page.as_mut().map(|p| RecordPageWrapper::new(loc.page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
            wrapper.set_page_in_use();
            let mut append_page_to_free_list = true;
            if !loc.is_multi_pages_record {
                if wrapper.get_free_list_len() != free_list_capacity {
                    append_page_to_free_list = false;
                }
                wrapper.append_free_list_item(loc.record_id_in_page);
            }
            append_page_to_free_list
        };
        if append_page_to_free_list {
            let first_free_page_ptr = self.pager.get_header_page_ref().get_header_first_free_page_ptr();
            let mut page = self.pager.load_page(loc.page_id);
            let mut wrapper = page.as_mut().map(|p| RecordPageWrapper::new(loc.page_id, p, self.page_map)).ok_or(RecordsManagerError::NotFound)?;
            wrapper.set_page_in_use();
            wrapper.set_free_next_page_ptr(first_free_page_ptr);
            self.pager.get_header_page_mut().set_header_first_free_page_ptr(loc.page_id);
        }
        self.decrement_records_counter();
        self.increment_records_version_counter();
        Ok(())
    }

    pub fn get_payload_len(&self) -> usize {
        self.page_map.payload.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pager.get_header_page_ref().is_empty()
    }

    pub fn sync(&mut self) {
        self.pager.sync();
        self.pager.clear();
    }

    pub fn soft_sync(&mut self) {
        self.pager.sync();
    }

    pub fn clear(&mut self) {
        self.pager.clear();
    }
    pub fn retrieve_all_records_ids(&mut self) -> RecordsManagerResult<Vec<u64>> {
        let page_count = self.pager.get_header_page_ref().get_page_count();
        let nb_records_per_page = self.page_map.nb_records_per_page;
        let mut res = Vec::new();
        for pid in 0..page_count {
            let page_id = pid + 1;
            let mut page = self.pager.fetch_non_empty_page(page_id);
            let mp = page.as_mut();
            let page_wrapper = mp.map(|p| RecordPageWrapper::new(page_id, p , self.page_map));
            if let Some(wrapper) = page_wrapper {
                let free_list = wrapper.get_page_free_list();
                let mut free_list_iter = free_list.iter();
                for page_record_id in 0..nb_records_per_page {
                    if let Some(free_list_item_id) = free_list_iter.next() {
                        if page_record_id != *free_list_item_id {
                            res.push((wrapper.get_id() - 1) * nb_records_per_page as u64 + page_record_id as u64 + 1);
                        }
                    } else {
                        res.push((wrapper.get_id() - 1) * nb_records_per_page as u64 + page_record_id as u64 + 1);
                    }
                }
            }
        }
        Ok(res)
    }
    pub fn get_pager_ref(&self) -> &Pager {
        &self.pager
    }
    pub fn get_pager_mut(&mut self) -> &mut Pager {
        &mut self.pager
    }
    pub fn erase(&mut self) {
        self.pager.erase();
    }
}


#[cfg(test)]
mod test_record_manager {
    use std::collections::HashSet;

    use super::*;
    use super::super::super::test_utils::*;
    #[test]
    fn test_create() {
        let file = build_file_path_and_rm_old("test_record_manager", "test_create.db").unwrap();
        let mut rm = RecordsManager::new(&file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD);

        let mut ids = vec![];
        for i in 0..10000 {
            let data = [i as u8; BTREE_NODE_RECORD_SIZE];
            let id = rm.create(&data).expect("record id");
            ids.push(id);
        }

        rm.soft_sync();

        for i in 0..10000 {
            let content = [i as u8; BTREE_NODE_RECORD_SIZE];
            let id = ids[i];
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            rm.load(id, &mut data).expect("load data");
            assert_eq!(content, data);
        }
        
        let mut rm_load = RecordsManager::new(&file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD);

        for i in 0..10000 {
            let data = [(i+1) as u8; BTREE_NODE_RECORD_SIZE];
            let id = ids[i];
            rm_load.save(id, &data).expect(&format!("load data {}", id));
        }

        rm_load.soft_sync();
        
        for i in 0..10000 {
            let data_1 = [(i+1) as u8; BTREE_NODE_RECORD_SIZE];
            let data_10 = [(i+10) as u8; BTREE_NODE_RECORD_SIZE];
            let id = ids[i];
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            if i % 2 == 0 {
                rm_load.save(id, &data_10).expect("save data");
            } else {
                rm_load.load(id, &mut data).expect("load data");
                assert_eq!(data_1, data);
            }
        }

        rm_load.soft_sync();
        let mut rm_load_2 = RecordsManager::new(&file, BTREE_NODE_RECORD_SIZE, BTREE_NB_RECORDS_PER_PAGE, BTREE_NB_PAGES_PER_RECORD);

        assert_eq!(rm_load_2.retrieve_all_records_ids().expect("ids"), ids);

        for i in 0..10000 {
            let data_1 = [(i+1) as u8; BTREE_NODE_RECORD_SIZE];
            let data_10 = [(i+10) as u8; BTREE_NODE_RECORD_SIZE];
            let id = ids[i];
            let mut data = [0u8; BTREE_NODE_RECORD_SIZE];
            rm_load.load(id, &mut data).expect("load data");
            if i % 2 == 0 {
                assert_eq!(data_10, data);
            } else {
                assert_eq!(data_1, data);
            }
        }

    }

    #[test]
    fn test_create_issue() {
        let file = build_file_path_and_rm_old("test_record_manager", "test_create.db").unwrap();
        let mut rm = RecordsManager::new(&file, NODE_RECORD_SIZE, NODE_NB_RECORDS_PER_PAGE, NODE_NB_PAGES_PER_RECORD);

        let mut ids = HashSet::new();
        for i in 0..10000 {
            let data = [i as u8; NODE_RECORD_SIZE];
            let id = rm.create(&data).expect("record id");
            ids.insert(id);
        }

        rm.sync();

        for i in 0..10000 {
            let data = [i as u8; NODE_RECORD_SIZE];
            let id = rm.create(&data).expect("record id");
            ids.insert(id);
        }

        rm.sync();

        assert_eq!(ids.len(), 20000);
    }
}