use std::marker::PhantomData;
use std::collections::HashSet;
use std::collections::HashMap;
use super::base_state::*;
use super::super::super::graph::traits::*;

pub fn push_state_0<'g, NIDA, NIDB, EIDA, GraphA, NA, RA>(base_state: &mut BaseState<NIDA, NIDB>, graph: &'g GraphA, v0: &NIDA, v1: &NIDB)
where  NIDA: std::hash::Hash + Eq + MemGraphId + Copy, NIDB: std::hash::Hash + Eq + MemGraphId + Copy,
EIDA: std::hash::Hash + Eq + MemGraphId + Copy,
NA: std::hash::Hash + Eq, RA: std::hash::Hash + Eq, 
GraphA: GraphContainerTrait<NIDA, EIDA, NA, RA>,
GraphA: GraphIteratorTrait<NIDA, EIDA> {  
    base_state.core_count += 1;
    base_state.core_map.insert(*v0, *v1);
    if !base_state.in_map.contains_key(&v0) {
        base_state.in_map.insert(*v0, base_state.core_count);
        base_state.term_in_count += 1;
        if base_state.out_map.contains_key(&v0) {
            base_state.term_both_count += 1;
        }
    }
    if !base_state.out_map.contains_key(&v0) {
        base_state.out_map.insert(*v0, base_state.core_count);
        base_state.term_out_count += 1;
        if base_state.in_map.contains_key(&v0) {
            base_state.term_both_count += 1;
        }
    }

    for edge_index in graph.in_edges(&v0) {
        let ancestor = graph.get_source_index(&edge_index);
        if !base_state.in_map.contains_key(&ancestor) {
            base_state.in_map.insert(ancestor, base_state.core_count);
            base_state.term_in_count += 1;
            if base_state.out_map.contains_key(&ancestor) {
                base_state.term_both_count += 1;
            }
        }
    }
    for edge_index in graph.out_edges(&v0) {
        let successor = graph.get_target_index(&edge_index);
        if !base_state.out_map.contains_key(&successor) {
            base_state.out_map.insert(successor, base_state.core_count);
            base_state.term_out_count += 1;
            if base_state.in_map.contains_key(&successor) {
                base_state.term_both_count += 1;
            }
        }
    }
}

pub fn pop_state_0<'g, NIDA, NIDB, EIDA, GraphA, NA, RA>(base_state: &mut BaseState<NIDA, NIDB>, graph: &'g GraphA, v0: &NIDA)
where  NIDA: std::hash::Hash + Eq + MemGraphId + Copy, NIDB: std::hash::Hash + Eq + MemGraphId + Copy,
EIDA: std::hash::Hash + Eq + MemGraphId + Copy,
NA: std::hash::Hash + Eq, RA: std::hash::Hash + Eq, 
GraphA: GraphContainerTrait<NIDA, EIDA, NA, RA>,
GraphA: GraphIteratorTrait<NIDA, EIDA>  {  
    if base_state.core_count == 0 {
        return;
    }

    if let Some(in_count) = base_state.in_map.get(&v0) {
        if *in_count == base_state.core_count {
            base_state.in_map.remove(v0);
            base_state.term_in_count -= 1;
            if base_state.out_map.contains_key(&v0) {
                if base_state.term_both_count > 0 {
                    base_state.term_both_count -= 1;
                }
            }
        }
    }

    for in_edge in graph.in_edges(&v0) {
        let source = graph.get_source_index(&in_edge);
        if let Some(in_count) = base_state.in_map.get(&source) {
            if *in_count == base_state.core_count {
                base_state.in_map.remove(&source);
                base_state.term_in_count -= 1;
                if base_state.out_map.contains_key(&source) {
                    if base_state.term_both_count > 0 {
                        base_state.term_both_count -= 1;
                    }
                }
            }
        }
    }

    if let Some(out_count) = base_state.out_map.get(&v0) {
        if *out_count == base_state.core_count {
            base_state.out_map.remove(v0);
            base_state.term_out_count -= 1;
            if base_state.in_map.contains_key(&v0) {
                if base_state.term_both_count > 0 {
                    base_state.term_both_count -= 1;
                }
            }
        }
    }

    for out_edge in graph.out_edges(&v0) {
        let target = graph.get_target_index(&out_edge);
        if let Some(out_count) = base_state.in_map.get(&target) {
            if *out_count == base_state.core_count {
                base_state.out_map.remove(&target);
                base_state.term_out_count -= 1;
                if base_state.in_map.contains_key(&target) {
                    if base_state.term_both_count > 0 {
                        base_state.term_both_count -= 1;
                    }
                }
            }
        }
    }

    base_state.core_map.remove(&v0);

    base_state.core_count -= 1;
}


pub fn push_state_1<'g, NIDA, NIDB, EIDA, GraphA, NA, RA>(base_state: &mut BaseState<NIDA, NIDB>, graph: &'g mut GraphA, v0: &NIDA, v1: &NIDB)
where  NIDA: std::hash::Hash + Eq + MemGraphId + Copy, NIDB: std::hash::Hash + Eq + MemGraphId + Copy,
EIDA: std::hash::Hash + Eq + MemGraphId + Copy,
NA: std::hash::Hash + Eq, RA: std::hash::Hash + Eq, 
GraphA: GrowableGraphContainerTrait<NIDA, EIDA, NA, RA>,
GraphA: GrowableGraphIteratorTrait<NIDA, EIDA> {  
    base_state.core_count += 1;
    base_state.core_map.insert(*v0, *v1);
    if !base_state.in_map.contains_key(&v0) {
        base_state.in_map.insert(*v0, base_state.core_count);
        base_state.term_in_count += 1;
        if base_state.out_map.contains_key(&v0) {
            base_state.term_both_count += 1;
        }
    }
    if !base_state.out_map.contains_key(&v0) {
        base_state.out_map.insert(*v0, base_state.core_count);
        base_state.term_out_count += 1;
        if base_state.in_map.contains_key(&v0) {
            base_state.term_both_count += 1;
        }
    }

    for edge_index in graph.in_edges(&v0) {
        let ancestor = graph.get_source_index(&edge_index);
        if !base_state.in_map.contains_key(&ancestor) {
            base_state.in_map.insert(ancestor, base_state.core_count);
            base_state.term_in_count += 1;
            if base_state.out_map.contains_key(&ancestor) {
                base_state.term_both_count += 1;
            }
        }
    }
    for edge_index in graph.out_edges(&v0) {
        let successor = graph.get_target_index(&edge_index);
        if !base_state.out_map.contains_key(&successor) {
            base_state.out_map.insert(successor, base_state.core_count);
            base_state.term_out_count += 1;
            if base_state.in_map.contains_key(&successor) {
                base_state.term_both_count += 1;
            }
        }
    }
}

pub fn pop_state_1<'g, NIDA, NIDB, EIDA, GraphA, NA, RA>(base_state: &mut BaseState<NIDA, NIDB>, graph: &'g mut GraphA, v0: &NIDA)
where  NIDA: std::hash::Hash + Eq + MemGraphId + Copy, NIDB: std::hash::Hash + Eq + MemGraphId + Copy,
EIDA: std::hash::Hash + Eq + MemGraphId + Copy,
NA: std::hash::Hash + Eq, RA: std::hash::Hash + Eq, 
GraphA: GrowableGraphContainerTrait<NIDA, EIDA, NA, RA>,
GraphA: GrowableGraphIteratorTrait<NIDA, EIDA>  {  
    if base_state.core_count == 0 {
        return;
    }

    if let Some(in_count) = base_state.in_map.get(&v0) {
        if *in_count == base_state.core_count {
            base_state.in_map.remove(v0);
            base_state.term_in_count -= 1;
            if base_state.out_map.contains_key(&v0) {
                if base_state.term_both_count > 0 {
                    base_state.term_both_count -= 1;
                }
            }
        }
    }

    for in_edge in graph.in_edges(&v0) {
        let source = graph.get_source_index(&in_edge);
        if let Some(in_count) = base_state.in_map.get(&source) {
            if *in_count == base_state.core_count {
                base_state.in_map.remove(&source);
                base_state.term_in_count -= 1;
                if base_state.out_map.contains_key(&source) {
                    if base_state.term_both_count > 0 {
                        base_state.term_both_count -= 1;
                    }
                }
            }
        }
    }

    if let Some(out_count) = base_state.out_map.get(&v0) {
        if *out_count == base_state.core_count {
            base_state.out_map.remove(v0);
            base_state.term_out_count -= 1;
            if base_state.in_map.contains_key(&v0) {
                if base_state.term_both_count > 0 {
                    base_state.term_both_count -= 1;
                }
            }
        }
    }

    for out_edge in graph.out_edges(&v0) {
        let target = graph.get_target_index(&out_edge);
        if let Some(out_count) = base_state.in_map.get(&target) {
            if *out_count == base_state.core_count {
                base_state.out_map.remove(&target);
                base_state.term_out_count -= 1;
                if base_state.in_map.contains_key(&target) {
                    if base_state.term_both_count > 0 {
                        base_state.term_both_count -= 1;
                    }
                }
            }
        }
    }

    base_state.core_map.remove(&v0);

    base_state.core_count -= 1;
}

pub struct State<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<NID0, EID0, N0, R0> + GraphIteratorTrait<NID0, EID0>,
    Graph1: GrowableGraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraphIteratorTrait<NID1, EID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {
    graph_0: &'g0 Graph0,
    graph_1: &'g1 mut Graph1,
    vertex_comp: VCOMP,
    edge_comp: ECOMP,
    phantom_v_0: PhantomData<N0>,
    phantom_v_1: PhantomData<N1>,
    phantom_r_0: PhantomData<R0>,
    phantom_r_1: PhantomData<R1>,
    phantom_e_0: PhantomData<EID0>,
    phantom_e_1: PhantomData<EID1>,
    base_state_0: BaseState<NID0, NID1>,
    base_state_1: BaseState<NID1, NID0>,

}

impl <'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>
    State<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<NID0, EID0, N0, R0> + GraphIteratorTrait<NID0, EID0>,
    Graph1: GrowableGraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraphIteratorTrait<NID1, EID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {


        pub fn new(graph_0: &'g0 Graph0, graph_1: &'g1 mut Graph1, vcomp: VCOMP, ecomp: ECOMP) -> Self {
            State {
                graph_0: graph_0,
                graph_1: graph_1,
                vertex_comp: vcomp,
                edge_comp: ecomp,
                phantom_e_0: PhantomData,
                phantom_e_1: PhantomData,
                phantom_r_0: PhantomData,
                phantom_r_1: PhantomData,
                phantom_v_0: PhantomData,
                phantom_v_1: PhantomData,
                base_state_0: BaseState::new(),
                base_state_1: BaseState::new(),
            }
        }

        pub fn push(&mut self, v0: &NID0, v1: &NID1) {
            push_state_0(&mut self.base_state_0, self.graph_0, v0, v1);
            push_state_1(&mut self.base_state_1, self.graph_1, v1, v0);
        }

        pub fn pop(&mut self, v0: &NID0, _v1: &NID1) {
            if let Some(&w) = self.base_state_0.core(v0) {
                pop_state_0(&mut self.base_state_0, self.graph_0, v0);
                pop_state_1(&mut self.base_state_1, self.graph_1, &w);
            }
            
        }

        pub fn feasible(&mut self, v_new: &NID0, w_new: &NID1) -> Option<bool> {
            let v = self.graph_0.get_node_ref(v_new);
            let w = self.graph_1.get_node_ref(w_new)?;
            if !(self.vertex_comp)(v, w) {
                Some(false)
            } else {
                let mut term_in0_count = 0;
                let mut term_out0_count = 0;
                let mut rest0_count = 0;

                {
                    let mut matched_edge_set = HashSet::new();
                    for edge_index in self.graph_0.in_edges(v_new) {
                        let source_index = self.graph_0.get_source_index(&edge_index);
                        if !self.inc_counters_match_edge_0(true, &mut term_in0_count, &mut term_out0_count, &mut rest0_count, v_new, &source_index, w_new, &edge_index, 
                            &mut matched_edge_set)? {
                            return Some(false);
                        }
                    }
                }
                {
                    let mut matched_edge_set = HashSet::new();
                    for edge_index in self.graph_0.out_edges(v_new) {
                        let target_index = self.graph_0.get_target_index(&edge_index);
                        if !self.inc_counters_match_edge_0(false, &mut term_in0_count, &mut term_out0_count, &mut rest0_count, v_new, &target_index, w_new, &edge_index, 
                            &mut matched_edge_set)? {
                            return Some(false);
                        }
                    }
                }


                let mut term_in1_count = 0;
                let mut term_out1_count = 0;
                let mut rest1_count = 0;

                {
                    let mut matched_edge_set = HashSet::new();
                    for edge_index in self.graph_1.in_edges(&w_new) {
                        let source_index = self.graph_1.get_source_index(&edge_index);
                        if !self.inc_counters_match_edge_1(true, &mut term_in1_count, &mut term_out1_count, &mut rest1_count, w_new, &source_index, v_new, &edge_index, 
                            &mut matched_edge_set)? {
                            return Some(false);
                        }
                    }
                }
                {
                    let mut matched_edge_set = HashSet::new();
                    for edge_index in self.graph_1.out_edges(&w_new) {
                        let target_index = self.graph_1.get_target_index(&edge_index);
                        if !self.inc_counters_match_edge_1(false, &mut term_in1_count, &mut term_out1_count, &mut rest1_count, w_new, &target_index, v_new, &edge_index, 
                            &mut matched_edge_set)? {
                            return Some(false);
                        }
                    }
                }
                Some(term_in0_count <= term_in1_count && term_out0_count <= term_out1_count && rest0_count <= rest1_count)
            }
        }

        fn inc_counters_match_edge_0(&mut self, is_inbound: bool, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, v_new: &NID0, v_adj: &NID0, w_new: &NID1, edge_index: &EID0, matched_edge_set: &mut HashSet<EID1>) -> Option<bool> {
            if self.base_state_0.in_core(v_adj) || v_new == v_adj {
                let mut w = *w_new;
                if *v_adj != *v_new {
                    if let Some(&ws) = self.base_state_0.core(v_adj) {
                        w =  ws;
                    }
                }
                
                let r0 = self.graph_0.get_relationship_ref(edge_index);

                if is_inbound {
                    if !self.edge_exists_1(&w, &w_new, r0, matched_edge_set)? {
                        return Some(false);
                    }
                } else {
                    if !self.edge_exists_1(&w_new, &w, r0, matched_edge_set)? {
                        return Some(false);
                    }
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
            return Some(true);
        }

        fn edge_exists_0(&mut self, source: &NID0, target: &NID0, e1: &EID1, matched_edge_set: &mut HashSet<EID0>) -> Option<bool> {
            for out_edge_index in self.graph_0.out_edges(source) {
                let curr_target = self.graph_0.get_target_index(&out_edge_index);
                if curr_target == *target && !matched_edge_set.contains(&out_edge_index) {
                    let r = self.graph_0.get_relationship_ref(&out_edge_index);
                    let r1 = self.graph_1.get_relationship_ref(e1)?;
                    if (self.edge_comp)(r, r1) {
                        matched_edge_set.insert(out_edge_index);
                        return Some(true);
                    }
                }
            }
            return  Some(false);
        }

        fn edge_exists_1(&mut self, source: &NID1, target: &NID1, r0: &R0, matched_edge_set: &mut HashSet<EID1>) -> Option<bool> {
            for out_edge_index in self.graph_1.out_edges(source) {
                let curr_target = self.graph_1.get_target_index(&out_edge_index);
                if curr_target == *target && !matched_edge_set.contains(&out_edge_index) {
                    let r = self.graph_1.get_relationship_ref(&out_edge_index)?;
                    if (self.edge_comp)(r0, r) {
                        matched_edge_set.insert(out_edge_index);
                        return Some(true);
                    }
                }
            }
            return  Some(false);
        }

        fn inc_counters_match_edge_1(&mut self, is_inbound: bool, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, w_new: &NID1, w_adj: &NID1, v_new: &NID0, edge_index: &EID1, matched_edge_set: &mut HashSet<EID0>) -> Option<bool> {
            if self.base_state_1.in_core(w_adj) || w_new == w_adj {
                let mut v = *v_new;
                if *w_adj != *w_new {
                    if let Some(&vs) = self.base_state_1.core(w_adj) {
                        v = vs;
                    }
                }
                if is_inbound {
                    if !self.edge_exists_0(&v, v_new, edge_index, matched_edge_set)? {
                        return Some(false);
                    }
                } else {
                    if !self.edge_exists_0(v_new, &v, edge_index, matched_edge_set)? {
                        return Some(false);
                    }
                }
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
            return Some(true);
        }

        pub fn possible_candidate_0(&self, v0: &NID0) -> bool {
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

        pub fn possible_candidate_1(&self, v1: &NID1) -> bool {
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

        pub fn call_back<CALLBACK>(&mut self, callback: &mut CALLBACK) -> Option<bool>
        where CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &mut Graph1) -> Option<bool>
        {
            callback(self.base_state_0.get_map(), self.base_state_1.get_map(), self.graph_0, self.graph_1)
        }

}
