// MIT License
// Copyright (c) 2025 Alexandre RICCIARDI
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

use super::properties_repository::*;
use super::super::model::*;
use super::super::repository::index::b_tree::*;

pub struct KeyValuesRepository {
    data_repository: PropertiesRespository,
    data_index: BTreeIndex,
}

impl KeyValuesRepository {
    pub fn new(init_ctx: init::DatabaseInitContext) -> Self {
        KeyValuesRepository { 
            data_repository: PropertiesRespository::new(&init_ctx.get_meta_data_values_store_path().unwrap(), &init_ctx.get_dynamic_store_path().unwrap()),
            data_index: BTreeIndex::new(&init_ctx.get_meta_data_index_store_path().unwrap())
        }
    }

    pub fn put(&mut self, key: &str, properties: &mut [Property]) -> Option<()> {
        let props_id = self.data_repository.create_list(properties)?;
        self.data_index.insert(key, props_id)
    }

    pub fn get(&mut self, key: &str) -> Option<Vec<Vec<Property>>> {
        let props_ids = self.data_index.search(key)?;
        let mut res = vec![];
        for pid in props_ids {
            let props = self.data_repository.retrieve_list(pid)?;
            res.push(props);
        }
        Some(res)
    }
    pub fn sync(&mut self) {
        self.data_index.sync();
        self.data_repository.sync();

    }
    pub fn clear(&mut self) {
        self.data_index.clear();
        self.data_repository.clear();
    }
}