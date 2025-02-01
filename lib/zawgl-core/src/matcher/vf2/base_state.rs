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

use std::collections::{HashMap};

pub struct BaseState<NID0, NID1> where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    pub term_in_count: usize,
    pub term_out_count: usize,
    pub term_both_count: usize,
    pub core_count: usize,
    pub core_map: HashMap<NID0, NID1>,
    pub multi_core_map: HashMap<NID0, Vec<NID1>>,
    pub in_map: HashMap<NID0, usize>,
    pub out_map: HashMap<NID0, usize>,
}

impl <NID0, NID1> BaseState<NID0, NID1> where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    
    pub fn new() -> Self {
        BaseState {
            term_in_count: 0,
            term_out_count: 0,
            term_both_count: 0,
            core_count: 0,
            core_map: HashMap::new(),
            multi_core_map: HashMap::new(),
            in_map: HashMap::new(),
            out_map: HashMap::new(),
        }
    }

    pub fn term_in(&self) -> bool {
        self.core_count < self.term_in_count
    }

    pub fn term_in_vertex(&self, v0: &NID0) -> bool {
        let has_in_count = self.in_map.contains_key(v0);
        has_in_count && !self.core_map.contains_key(v0)
    }

    pub fn multi_term_in_vertex(&self, v0: &NID0) -> bool {
        let has_in_count = self.in_map.contains_key(v0);
        has_in_count && !self.multi_core_map.contains_key(v0)
    }
    
    pub fn term_out(&self) -> bool {
        self.core_count < self.term_out_count
    }

    pub fn term_out_vertex(&self, v0: &NID0) -> bool {
        let has_out_count = self.out_map.contains_key(v0);
        has_out_count && !self.core_map.contains_key(v0)
    }

    pub fn multi_term_out_vertex(&self, v0: &NID0) -> bool {
        let has_out_count = self.out_map.contains_key(v0);
        has_out_count && !self.multi_core_map.contains_key(v0)
    }
    
    pub fn term_both(&self) -> bool {
        self.core_count < self.term_both_count
    }
    
    pub fn term_both_vertex(&self, v0: &NID0) -> bool {
        let has_in_count = self.in_map.contains_key(v0); 
        let has_out_count = self.out_map.contains_key(v0);
        has_in_count && has_out_count && !self.core_map.contains_key(v0)
    }

    pub fn multi_term_both_vertex(&self, v0: &NID0) -> bool {
        let has_in_count = self.in_map.contains_key(v0); 
        let has_out_count = self.out_map.contains_key(v0);
        has_in_count && has_out_count && !self.multi_core_map.contains_key(v0)
    }

    pub fn in_core(&self, v0: &NID0) -> bool
    {
        self.core_map.contains_key(v0)
    }

    pub fn in_multi_core(&self, v0: &NID0) -> bool {
        self.multi_core_map.contains_key(v0)
    }

    pub fn count(&self) -> usize {
        self.core_count
    }

    pub fn multi_core_last(&self, v0: &NID0) -> Option<NID1> {
        self.multi_core_map.get(v0).unwrap().iter().last().cloned()
    }

    pub fn get_map(&self) ->  &HashMap<NID0, NID1> {
        &self.core_map
    }

    pub fn get_multi_core_map(&self) ->  &HashMap<NID0, Vec<NID1>> {
        &self.multi_core_map
    }

    pub fn in_depth(&self, v0: &NID0) -> usize {
        if let Some(count) = self.in_map.get(v0) {
            *count
        } else {
            0
        }
    }

    pub fn out_depth(&self, v0: &NID0) -> usize {
        if let Some(count) = self.out_map.get(v0) {
            *count
        } else {
            0
        }
    }

    pub fn term_set(&self) -> (usize, usize, usize) {
        (self.term_in_count, self.term_out_count, self.term_both_count)
    }
}
