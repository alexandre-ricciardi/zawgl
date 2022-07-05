use std::collections::HashMap;
use super::super::super::graph::traits::*;

pub struct BaseState<NID0: MemGraphId, NID1: MemGraphId> where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    pub term_in_count: usize,
    pub term_out_count: usize,
    pub term_both_count: usize,
    pub core_count: usize,
    pub core_map: HashMap<NID0, NID1>,
    pub in_map: HashMap<NID0, usize>,
    pub out_map: HashMap<NID0, usize>,
}

impl <NID0: MemGraphId, NID1: MemGraphId> BaseState<NID0, NID1> where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    
    pub fn new() -> Self {
        BaseState {
            term_in_count: 0,
            term_out_count: 0,
            term_both_count: 0,
            core_count: 0,
            core_map: HashMap::new(),
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
    
    pub fn term_out(&self) -> bool {
        self.core_count < self.term_out_count
    }

    pub fn term_out_vertex(&self, v0: &NID0) -> bool {
        let has_out_count = self.out_map.contains_key(v0);
        has_out_count && !self.core_map.contains_key(v0)
    }
    
    pub fn term_both(&self) -> bool {
        self.core_count < self.term_both_count
    }
    
    pub fn term_both_vertex(&self, v0: &NID0) -> bool {
        let has_in_count = self.in_map.contains_key(v0); 
        let has_out_count = self.out_map.contains_key(v0);
        has_in_count && has_out_count && !self.core_map.contains_key(v0)
    }

    pub fn in_core(&self, v0: &NID0) -> bool
    {
        self.core_map.contains_key(&v0)
    }

    pub fn count(&self) -> usize {
        self.core_count
    }

    pub fn core(&self, v0: &NID0) -> Option<&NID1> {
        self.core_map.get(v0)
    }

    pub fn get_map(&self) ->  &HashMap<NID0, NID1> {
        &self.core_map
    }

    pub fn in_depth(&self, v0: &NID0) -> usize {
        if let Some(count) = self.in_map.get(&v0) {
            *count
        } else {
            0
        }
    }

    pub fn out_depth(&self, v0: &NID0) -> usize {
        if let Some(count) = self.out_map.get(&v0) {
            *count
        } else {
            0
        }
    }

    pub fn term_set(&self) -> (usize, usize, usize) {
        (self.term_in_count, self.term_out_count, self.term_both_count)
    }
}
