use std::collections::HashMap;
use super::super::super::graph::traits::GraphTrait;
use super::super::super::graph::{NodeIndex};

pub struct BaseState<'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, OutIt0: Iterator<Item=EID0>, InIt0: Iterator<Item=EID0>,
    Graph1, OutIt1: Iterator<Item=EID1>, InIt1: Iterator<Item=EID1>> 
    where NID0: std::hash::Hash + Eq, NID1: std::hash::Hash + Eq,
    EID0: std::hash::Hash + Eq, EID1: std::hash::Hash + Eq, 
    Graph0: GraphTrait<NID0, EID0, OutIt0, InIt0>, Graph1: GraphTrait<NID0, EID0, OutIt1, InIt1> {
    term_in_count: usize,
    term_out_count: usize,
    term_both_count: usize,
    core_count: usize,
    core_map: HashMap<NID0, NID1>,
    in_map: HashMap<NID0, usize>,
    out_map: HashMap<NID0, usize>,
    graph_0: &'g0 Graph0,
    graph_1: &'g1 Graph1,

}

impl <'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, OutIt0, InIt0, Graph1, OutIt1, InIt1> BaseState<'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, OutIt0, InIt0, Graph1, OutIt1, InIt1> 
    where NID0: std::hash::Hash + Eq, NID1: std::hash::Hash + Eq,
    EID0: std::hash::Hash + Eq, EID1: std::hash::Hash + Eq, 
    Graph0: GraphTrait<NID0, EID0, OutIt0, InIt0>, Graph1: GraphTrait<NID0, EID0, OutIt1, InIt1> {
    pub fn push(&mut self, v0: NID0, v1: NID1) {  
        self.core_count += 1;
        self.core_map.insert(v0, v1);
        if !self.in_map.contains_key(&v0) {
            self.in_map.insert(v0, self.core_count);
            self.term_in_count += 1;
            if self.out_map.contains_key(&v0) {
                self.term_both_count += 1;
            }
        }
        if !self.out_map.contains_key(&v0) {
            self.out_map.insert(v0, self.core_count);
            self.term_out_count += 1;
            if self.in_map.contains_key(&v0) {
                self.term_both_count += 1;
            }
        }

        for edge_index in self.graph_0.in_edges(&v0) {
            let ancestor = self.graph_0.get_source_index(&edge_index);
            if !self.in_map.contains_key(&ancestor) {
                self.in_map.insert(ancestor, self.core_count);
                self.term_in_count += 1;
                if self.out_map.contains_key(&ancestor) {
                    self.term_both_count += 1;
                }
            }
        }
        for edge_index in self.graph_0.out_edges(&v0) {
            let successor = self.graph_0.get_target_index(&edge_index);
            if !self.out_map.contains_key(&successor) {
                self.out_map.insert(successor, self.core_count);
                self.term_out_count += 1;
                if self.in_map.contains_key(&successor) {
                    self.term_both_count += 1;
                }
            }
        }
    }

    pub fn pop(&mut self, v0: NodeIndex, v1: NodeIndex) {  
        if self.core_count == 0 {
            return;
        }

        if let Some(in_count) = self.in_map.get(&v0) {
            if *in_count == self.core_count {
                self.in_map.insert(v0, 0);
                self.term_in_count -= 1;
                if let Some(_out_count) = self.out_map.get(&v0) {
                    self.term_both_count -= 1;
                }
            }
        }

        for in_edge in self.graph_0.in_edges(&v0) {
            let source = self.graph_0.get_source_index(&in_edge);
            if let Some(in_count) = self.in_map.get(&source) {
                if *in_count == self.core_count {
                    self.in_map.insert(source, 0);
                    self.term_in_count -= 1;
                    if let Some(_out_count) = self.out_map.get(&source) {
                        self.term_both_count -= 1;
                    }
                }
            }
        }

        if let Some(out_count) = self.out_map.get(&v0) {
            if *out_count == self.core_count {
                self.out_map.insert(v0, 0);
                self.term_out_count -= 1;
                if let Some(_in_count) = self.in_map.get(&v0) {
                    self.term_both_count -= 1;
                }
            }
        }

        for out_edge in self.graph_0.out_edges(&v0) {
            let target = self.graph_0.get_target_index(&out_edge);
            if let Some(out_count) = self.in_map.get(&target) {
                if *out_count == self.core_count {
                    self.out_map.insert(target, 0);
                    self.term_out_count -= 1;
                    if let Some(_in_count) = self.in_map.get(&target) {
                        self.term_both_count -= 1;
                    }
                }
            }
        }

        self.core_map.remove(&v0);

        self.core_count -= 1;
    }

    pub fn term_in(&self) -> bool {
        self.core_count < self.term_in_count
    }

    pub fn term_in_vertex(&self, v0: NodeIndex) -> bool {
        let has_in_count = self.in_map.get(&v0).map(|count| *count > 0);
        has_in_count.map(|has_in| has_in && !self.core_map.contains_key(&v0)) == Some(true)
    }
    
    pub fn term_out(&self) -> bool {
        self.core_count < self.term_out_count
    }

    pub fn term_out_vertex(&self, v0: NodeIndex) -> bool {
        let has_out_count = self.out_map.get(&v0).map(|count| *count > 0);
        has_out_count.map(|has_out| has_out && self.core_map.contains_key(&v0)) == Some(true)
    }
    
    pub fn term_both(&self) -> bool {
        self.core_count < self.term_both_count
    }
    
    pub fn term_both_vertex(&self, v0: NodeIndex) -> bool {
        let has_in_count = self.in_map.get(&v0).map(|count| *count > 0); 
        let has_out_count = self.out_map.get(&v0).map(|count| *count > 0);
        has_in_count.and_then(|has_in|has_out_count.map(|has_out| self.core_map.contains_key(&v0) && has_in && has_out)) == Some(true)
    }

    pub fn in_core(&self, v0: NodeIndex) -> bool
    {
        self.core_map.contains_key(&v0)
    }

    pub fn count(&self) -> usize {
        self.core_count
    }

    pub fn core(&self, v0: NodeIndex) -> Option<NodeIndex> {
        self.core_map.get(&v0).map(|v1| *v1)
    }

    pub fn get_map(&self) ->  &HashMap<NodeIndex, NodeIndex> {
        &self.core_map
    }

    pub fn in_depth(&self, v0: NodeIndex) -> usize {
        if let Some(count) = self.in_map.get(&v0) {
            *count
        } else {
            0
        }
    }

    pub fn out_depth(&self, v0: NodeIndex) -> usize {
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

