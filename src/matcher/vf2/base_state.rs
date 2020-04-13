use std::collections::HashMap;
use super::super::super::graph::container::GraphTrait;
use super::super::super::graph::{NodeIndex};

pub struct BaseState<'g0, 'g1, V0: Eq + std::hash::Hash, V1: Eq + std::hash::Hash, R0, R1> {
    term_in_count: usize,
    term_out_count: usize,
    term_both_count: usize,
    core_count: usize,
    core_map: HashMap<NodeIndex, NodeIndex>,
    in_map: HashMap<NodeIndex, usize>,
    out_map: HashMap<NodeIndex, usize>,
    graph_0: &'g0 dyn GraphTrait<V0, R0>,
    graph_1: &'g1 dyn GraphTrait<V1, R1>,

}

impl <'g0, 'g1, V0: Eq + std::hash::Hash, V1: Eq + std::hash::Hash, R0, R1> BaseState<'g0, 'g1, V0, V1, R0, R1> {
    pub fn push(&mut self, v0: NodeIndex, v1: NodeIndex) {  
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
        let has_not_core = self.core_map.get(&v0).map(|v1| *v1 == 0);
        has_in_count.and_then(|has_in| has_not_core.map(|no_core| has_in && no_core)) == Some(true)
    }
    
    pub fn term_out(&self) -> bool {
        self.core_count < self.term_out_count
    }

    pub fn term_out_vertex(&self, v0: NodeIndex) -> bool {
        let has_out_count = self.out_map.get(&v0).map(|count| *count > 0);
        let has_not_core = self.core_map.get(&v0).map(|v1| *v1 == 0);
        has_out_count.and_then(|has_out| has_not_core.map(|no_core| has_out && no_core)) == Some(true)
    }
    
    pub fn term_both(&self) -> bool {
        self.core_count < self.term_both_count
    }
    
    pub fn term_both_vertex(&self, v0: NodeIndex) -> bool {
        let has_out_count = self.in_map.get(&v0).map(|count| *count > 0); 
        let has_out_count = self.out_map.get(&v0).map(|count| *count > 0);
        let has_not_core = self.core_map.get(&v0).map(|v1| *v1 == 0);
        has_out_count.and_then(|has_out| has_not_core.map(|no_core| has_out && no_core)) == Some(true)
    }
    
}

