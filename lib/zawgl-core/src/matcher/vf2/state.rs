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

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use log::trace;

use crate::graph_engine::model::GraphProxy;
use crate::graph_engine::model::ProxyNodeId;
use crate::graph_engine::model::ProxyRelationshipId;
use crate::model::Node;
use crate::model::PropertyGraph;
use crate::model::Relationship;

use super::base_state::*;
use super::super::super::graph::traits::*;
use super::super::super::graph::*;

fn inc_in<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, v0: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    base_state.term_in_count += 1;
    if base_state.out_map.contains_key(v0) {
        base_state.term_both_count += 1;
    }
}
fn inc_out<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, v0: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    base_state.term_out_count += 1;
    if base_state.in_map.contains_key(v0) {
        base_state.term_both_count += 1;
    }
}


fn inc_term<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, v0: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    if !base_state.in_map.contains_key(v0) {
        base_state.in_map.insert(*v0, base_state.core_count);
        inc_in(base_state, v0);
    }
    if !base_state.out_map.contains_key(v0) {
        base_state.out_map.insert(*v0, base_state.core_count);
        inc_out(base_state, v0);
    }
}

fn dec_in<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, v0: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    base_state.term_in_count -= 1;
    if base_state.out_map.contains_key(v0) && base_state.term_both_count > 0 {
        base_state.term_both_count -= 1;
    }
}
fn dec_out<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, v0: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    base_state.term_out_count -= 1;
    if base_state.in_map.contains_key(v0) && base_state.term_both_count > 0 {
        base_state.term_both_count -= 1;
    }
}
fn dec_in_term<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, v0: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    if let Some(in_count) = base_state.in_map.get(v0) {
        if *in_count == base_state.core_count {
            base_state.in_map.remove(v0);
            dec_in(base_state, v0);
        }
    }
}

fn dec_out_term<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, v0: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    if let Some(out_count) = base_state.out_map.get(v0) {
        if *out_count == base_state.core_count {
            base_state.out_map.remove(v0);
            dec_out(base_state, v0);
        }
    }
}

fn inc_source<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, ancestor: NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    if let Entry::Vacant(e) = base_state.in_map.entry(ancestor) {
        e.insert(base_state.core_count);
        inc_in(base_state, &ancestor);
    }
}

fn inc_target<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, successor: NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    if let Entry::Vacant(e) = base_state.out_map.entry(successor) {
        e.insert(base_state.core_count);
        inc_out(base_state, &successor);
    }
}

fn dec_source<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, source: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    if let Some(in_count) = base_state.in_map.get(source) {
        if *in_count == base_state.core_count {
            base_state.in_map.remove(source);
            dec_in(base_state, source);
        }
    }
}

fn dec_target<NID0: MemGraphId, NID1: MemGraphId>(base_state: &mut BaseState<NID0, NID1>, target: &NID0)  where NID0: std::hash::Hash + Eq + Copy, NID1: std::hash::Hash + Eq + Copy {
    if let Some(out_count) = base_state.out_map.get(target) {
        if *out_count == base_state.core_count {
            base_state.out_map.remove(target);
            dec_out(base_state, target);
        }
    }
}

pub fn push_state_0(base_state: &mut BaseState<NodeIndex, ProxyNodeId>, graph: &PropertyGraph, v0: &NodeIndex, v1: &ProxyNodeId) {  
    base_state.core_count += 1;
    base_state.core_map.insert(*v0, *v1);
    inc_term(base_state, v0);

    for edge_index in graph.in_edges(v0) {
        let ancestor = graph.get_source_index(&edge_index);
        inc_source(base_state, ancestor);
    }
    for edge_index in graph.out_edges(v0) {
        let successor = graph.get_target_index(&edge_index);
        inc_target(base_state, successor);
    }
}

pub fn pop_state_0(base_state: &mut BaseState<NodeIndex, ProxyNodeId>, graph: &PropertyGraph, v0: &NodeIndex) {  
    if base_state.core_count == 0 {
        return;
    }

    dec_in_term(base_state, v0);
    for in_edge in graph.in_edges(v0) {
        let source = graph.get_source_index(&in_edge);
        dec_source(base_state, &source);
    }

    dec_out_term(base_state, v0);

    for out_edge in graph.out_edges(v0) {
        let target = graph.get_target_index(&out_edge);
        dec_target(base_state, &target);
    }

    base_state.core_map.remove(v0);

    base_state.core_count -= 1;
}


pub fn push_state_1(base_state: &mut BaseState<ProxyNodeId, NodeIndex>, graph: & mut GraphProxy, v0: &ProxyNodeId, v1: &NodeIndex) {  
    base_state.core_count += 1;
    base_state.core_map.insert(*v0, *v1);
    inc_term(base_state, v0);
    for (_edge_index, ancestor, _rel) in graph.in_edges(v0) {
        inc_source(base_state, ancestor);
    }
    for (_edge_index, successor, _rel) in graph.out_edges(v0) {
        inc_target(base_state, successor);
    }
}

pub fn pop_state_1(base_state: &mut BaseState<ProxyNodeId, NodeIndex>, graph: &mut GraphProxy, v0: &ProxyNodeId) {  
    if base_state.core_count == 0 {
        return;
    }

    dec_in_term(base_state, v0);

    for (_in_edge, source, _rel) in graph.in_edges(v0) {
        dec_source(base_state, &source);
    }

    dec_out_term(base_state, v0);

    for (_out_edge, target, _rel) in graph.out_edges(v0) {
        dec_target(base_state, &target);
    }

    base_state.core_map.remove(v0);

    base_state.core_count -= 1;
}

pub struct State<'g0, VCOMP, ECOMP>
    where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool {
    graph_0: &'g0 PropertyGraph,
    vertex_comp: VCOMP,
    edge_comp: ECOMP,
    base_state_0: BaseState<NodeIndex, ProxyNodeId>,
    base_state_1: BaseState<ProxyNodeId, NodeIndex>,

}

impl <'g0, VCOMP, ECOMP> State<'g0, VCOMP, ECOMP>
    where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool {


        pub fn new(graph_0: &'g0 PropertyGraph, vcomp: VCOMP, ecomp: ECOMP) -> Self {
            State {
                graph_0,
                vertex_comp: vcomp,
                edge_comp: ecomp,
                base_state_0: BaseState::new(),
                base_state_1: BaseState::new(),
            }
        }

        pub fn push(&mut self, v0: &NodeIndex, v1: &ProxyNodeId, graph_1: &mut GraphProxy<'_>) {
            push_state_0(&mut self.base_state_0, self.graph_0, v0, v1);
            push_state_1(&mut self.base_state_1, graph_1, v1, v0);
        }

        pub fn pop(&mut self, v0: &NodeIndex, _v1: &ProxyNodeId, graph_1: &mut GraphProxy<'_>) {
            if let Some(&w) = self.base_state_0.core(v0) {
                pop_state_0(&mut self.base_state_0, self.graph_0, v0);
                pop_state_1(&mut self.base_state_1, graph_1, &w);
            }
            
        }

        pub fn feasible(&mut self, v_new: &NodeIndex, w_new: &ProxyNodeId, graph_1: &mut GraphProxy<'_>) -> Option<bool> {
            let v = self.graph_0.get_node_ref(v_new);
            let w = graph_1.get_node_ref(w_new)?;
            if !(self.vertex_comp)(v, w) {
                Some(false)
            } else {
                trace!("vertex match: {:?} {:?}", v, w);
                let mut term_in0_count = 0;
                let mut term_out0_count = 0;
                let mut rest0_count = 0;

                {
                    let mut matched_edge_set = HashSet::new();
                    for edge_index in self.graph_0.in_edges(v_new) {
                        let source_index = self.graph_0.get_source_index(&edge_index);
                        if !self.inc_counters_match_edge_0(true, &mut term_in0_count, &mut term_out0_count, &mut rest0_count, v_new, &source_index, w_new, &edge_index, 
                            &mut matched_edge_set, graph_1)? {
                            return Some(false);
                        }
                    }
                }
                {
                    let mut matched_edge_set = HashSet::new();
                    for edge_index in self.graph_0.out_edges(v_new) {
                        let target_index = self.graph_0.get_target_index(&edge_index);
                        if !self.inc_counters_match_edge_0(false, &mut term_in0_count, &mut term_out0_count, &mut rest0_count, v_new, &target_index, w_new, &edge_index, 
                            &mut matched_edge_set, graph_1)? {
                            return Some(false);
                        }
                    }
                }


                let mut term_in1_count = 0;
                let mut term_out1_count = 0;
                let mut rest1_count = 0;
                {
                    let mut matched_edge_set = HashSet::new();
                    for (edge_index, source_index, _rel) in graph_1.in_edges(w_new) {
                        if !self.inc_counters_match_edge_1(true, &mut term_in1_count, &mut term_out1_count, &mut rest1_count, w_new, &source_index, v_new, &edge_index, 
                            &mut matched_edge_set)? {
                            return Some(false);
                        }
                    }
                }
                {
                    let mut matched_edge_set = HashSet::new();
                    for (edge_index, target_index, _rel) in graph_1.out_edges(w_new) {
                        if !self.inc_counters_match_edge_1(false, &mut term_in1_count, &mut term_out1_count, &mut rest1_count, w_new, &target_index, v_new, &edge_index, 
                            &mut matched_edge_set)? {
                            return Some(false);
                        }
                    }
                }
                Some(term_in0_count <= term_in1_count && term_out0_count <= term_out1_count && rest0_count <= rest1_count)
            }
        }

        fn inc_counters_match_edge_0(&mut self, is_inbound: bool, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, v_new: &NodeIndex, v_adj: &NodeIndex, w_new: &ProxyNodeId, edge_index: &EdgeIndex, matched_edge_set: &mut HashSet<ProxyRelationshipId>, graph_1: &mut GraphProxy<'_>) -> Option<bool> {
            if self.base_state_0.in_core(v_adj) || v_new == v_adj {
                let mut w = *w_new;
                if v_adj != v_new {
                    if let Some(&ws) = self.base_state_0.core(v_adj) {
                        w =  ws;
                    }
                }
                
                let r0 = self.graph_0.get_relationship_ref(edge_index);

                if is_inbound {
                    if !self.edge_exists_1(&w, w_new, r0, matched_edge_set, graph_1)? {
                        return Some(false);
                    }
                } else if !self.edge_exists_1(w_new, &w, r0, matched_edge_set, graph_1)? {
                    return Some(false);
                }
            } else {
                if  self.base_state_0.in_depth(v_adj) > 0 {
                    *term_in += 1;
                }
                if  self.base_state_0.out_depth(v_adj) > 0 {
                    *term_out += 1;
                }
                if  self.base_state_0.in_depth(v_adj) == 0 &&  self.base_state_0.out_depth(v_adj) == 0 {
                    *rest += 1;
                }
            }
            Some(true)
        }

        fn edge_exists_0(&mut self, source: &NodeIndex, target: &NodeIndex, r1: &Relationship, matched_edge_set: &mut HashSet<EdgeIndex>) -> Option<bool> {
            for out_edge_index in self.graph_0.out_edges(source) {
                let curr_target = self.graph_0.get_target_index(&out_edge_index);
                if curr_target == *target && !matched_edge_set.contains(&out_edge_index) {
                    let r = self.graph_0.get_relationship_ref(&out_edge_index);
                    if (self.edge_comp)(r, r1) {
                        matched_edge_set.insert(out_edge_index);
                        return Some(true);
                    }
                }
            }
            return  Some(false);
        }

        fn edge_exists_1(&mut self, source: &ProxyNodeId, target: &ProxyNodeId, r0: &Relationship, matched_edge_set: &mut HashSet<ProxyRelationshipId>, graph_1: &mut GraphProxy<'_>) -> Option<bool> {
            for (out_edge_index, curr_target, r) in graph_1.out_edges(source) {
                if curr_target == *target && !matched_edge_set.contains(&out_edge_index) && (self.edge_comp)(r0, &r){
                    matched_edge_set.insert(out_edge_index);
                    return Some(true);
                }
            }
            Some(false)
        }

        fn inc_counters_match_edge_1(&self, _is_inbound: bool, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, w_new: &ProxyNodeId, w_adj: &ProxyNodeId, _v_new: &NodeIndex, _edge_index: &ProxyRelationshipId, _matched_edge_set: &mut HashSet<EdgeIndex>) -> Option<bool> {
            if self.base_state_1.in_core(w_adj) || w_new == w_adj {
                
            } else {
                if self.base_state_1.in_depth(w_adj) > 0 {
                    *term_in += 1;
                }
                if self.base_state_1.out_depth(w_adj) > 0 {
                    *term_out += 1;
                }
                if self.base_state_1.in_depth(w_adj) == 0 && self.base_state_1.out_depth(w_adj) == 0 {
                    *rest += 1;
                }
            }
            Some(true)
        }

        pub fn possible_candidate_0(&self, v0: &NodeIndex) -> bool {
            if self.base_state_0.term_both() && self.base_state_1.term_both() {
                self.base_state_0.term_both_vertex(v0)
            } else if self.base_state_0.term_out() && self.base_state_1.term_out() {
                self.base_state_0.term_out_vertex(v0)
            } else if self.base_state_0.term_in() && self.base_state_1.term_in() {
                self.base_state_0.term_in_vertex(v0)
            } else {
                !self.base_state_0.in_core(v0)
            }
        }

        pub fn possible_candidate_1(&self, v1: &ProxyNodeId) -> bool {
            if self.base_state_0.term_both() && self.base_state_1.term_both() {
                self.base_state_1.term_both_vertex(v1)
            } else if self.base_state_0.term_out() && self.base_state_1.term_out() {
                self.base_state_1.term_out_vertex(v1)
            } else if self.base_state_0.term_in() && self.base_state_1.term_in() {
                self.base_state_1.term_in_vertex(v1)
            } else {
                !self.base_state_1.in_core(v1)
            }
        }

        pub fn success(&self) -> bool {
            self.base_state_0.count() == self.graph_0.nodes_len()
        }

        pub fn valid(&self) -> bool {
            let term_set_0 = self.base_state_0.term_set();
            let term_set_1 = self.base_state_1.term_set();
            term_set_0.0 <= term_set_1.0 && term_set_0.1 <= term_set_1.1 && term_set_0.2 <= term_set_1.2
        }

        pub fn call_back<CALLBACK>(&mut self, callback: &mut CALLBACK, graph_1: &mut GraphProxy<'_>) -> Option<bool>
        where CALLBACK: FnMut(&HashMap<NodeIndex, ProxyNodeId>, &HashMap<ProxyNodeId, NodeIndex>, &PropertyGraph, &mut GraphProxy<'_>) -> Option<bool>
        {
            callback(self.base_state_0.get_map(), self.base_state_1.get_map(), self.graph_0, graph_1)
        }

}
