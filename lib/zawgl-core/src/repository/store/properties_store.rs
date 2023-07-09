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

use super::super::super::buf_config::*;
use super::super::records::*;
use super::records::*;

pub struct PropertiesStore {
    records_manager: RecordsManager,
}

impl PropertiesStore {
    pub fn new(file: &str) -> Self {
        PropertiesStore {records_manager: RecordsManager::new(file, PROPERTY_RECORD_SIZE, PROPERTY_NB_RECORDS_PER_PAGE, PROPERTY_NB_PAGES_PER_RECORD)}
    }
    pub fn create(&mut self, pr: &PropertyRecord) -> Option<u64> {
        self.records_manager.create(&pr_to_bytes(pr)).ok()
    }
    pub fn load(&mut self, pr_id: u64) -> Option<PropertyRecord> {
        let mut data: [u8; 42] = [0; 42];
        self.records_manager.load(pr_id, &mut data).ok()?;
        Some(pr_from_bytes(data))
    }
    pub fn sync(&mut self) {
        self.records_manager.sync();
        self.clear();
    }
    pub fn clear(&mut self) {
        self.records_manager.clear();
    }
}