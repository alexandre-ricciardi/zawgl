use std::marker::PhantomData;
use std::collections::HashSet;
use std::collections::HashMap;
use super::base_state::*;
use super::super::super::graph::traits::*;

pub fn push_state<'g, NIDA, NIDB, EIDA, GraphA, NA, RA>(base_state: &mut BaseState<NIDA, NIDB>, graph: &'g GraphA, v0: &NIDA, v1: &NIDB)
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

pub fn pop_state<'g, NIDA, NIDB, EIDA, GraphA, NA, RA>(base_state: &mut BaseState<NIDA, NIDB>, graph: &'g GraphA, v0: &NIDA)
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
            base_state.in_map.insert(*v0, 0);
            base_state.term_in_count -= 1;
            if let Some(_out_count) = base_state.out_map.get(&v0) {
                base_state.term_both_count -= 1;
            }
        }
    }

    for in_edge in graph.in_edges(&v0) {
        let source = graph.get_source_index(&in_edge);
        if let Some(in_count) = base_state.in_map.get(&source) {
            if *in_count == base_state.core_count {
                base_state.in_map.insert(source, 0);
                base_state.term_in_count -= 1;
                if let Some(_out_count) = base_state.out_map.get(&source) {
                    base_state.term_both_count -= 1;
                }
            }
        }
    }

    if let Some(out_count) = base_state.out_map.get(&v0) {
        if *out_count == base_state.core_count {
            base_state.out_map.insert(*v0, 0);
            base_state.term_out_count -= 1;
            if let Some(_in_count) = base_state.in_map.get(&v0) {
                base_state.term_both_count -= 1;
            }
        }
    }

    for out_edge in graph.out_edges(&v0) {
        let target = graph.get_target_index(&out_edge);
        if let Some(out_count) = base_state.in_map.get(&target) {
            if *out_count == base_state.core_count {
                base_state.out_map.insert(target, 0);
                base_state.term_out_count -= 1;
                if let Some(_in_count) = base_state.in_map.get(&target) {
                    base_state.term_both_count -= 1;
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
    Graph1: GraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraph<NID1, EID1> + GraphIteratorTrait<NID1, EID1>,
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
    Graph1: GraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraph<NID1, EID1> + GraphIteratorTrait<NID1, EID1>,
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
            push_state(&mut self.base_state_0, self.graph_0, v0, v1);
            push_state(&mut self.base_state_1, self.graph_1, v1, v0);
        }

        pub fn pop(&mut self, v0: &NID0, _v1: &NID1) {
            if let Some(w_val) = self.base_state_0.core(v0) {
                pop_state(&mut self.base_state_0, self.graph_0, v0);
                pop_state(&mut self.base_state_1, self.graph_1, &w_val);
            }
        }

        fn retrieve_graph_1_around(&mut self, node_id: &NID1) {
            self.graph_1.retrieve_node(node_id);
            self.graph_1.retrieve_in_edges(node_id);
            for eid in self.graph_1.in_edges(node_id) {
                self.graph_1.retrieve_relationship(&eid);
            }
            self.graph_1.retrieve_out_edges(node_id);
            for eid in self.graph_1.out_edges(node_id) {
                self.graph_1.retrieve_relationship(&eid);
            }
        }

        pub fn feasible(&mut self, v_new: &NID0, w_new: &NID1) -> bool {
            self.retrieve_graph_1_around(w_new);
            let v = self.graph_0.get_node_ref(v_new);
            let w = self.graph_1.get_node_ref(w_new);
            if !(self.vertex_comp)(v, w) {
                false
            } else {
                let mut term_in0_count = 0;
                let mut term_out0_count = 0;
                let mut rest0_count = 0;

                {
                    let mut edge_predicate = EquivalentEdgePredicate::new(self.graph_1, |r1, r0| (self.edge_comp)(r0, r1));
                    for edge_index in self.graph_0.in_edges(v_new) {
                        let source_index = self.graph_0.get_source_index(&edge_index);
                        if !self.inc_counters_match_edge_0(&mut term_in0_count, &mut term_out0_count, &mut rest0_count, v_new, &source_index, w_new, &edge_index, 
                            |w_source, w_new, r0| {
                                edge_predicate.edge_exists(w_source, w_new, r0)
                            }) {
                            return false;
                        }
                    }
                }
                {
                    let mut edge_predicate = EquivalentEdgePredicate::new(self.graph_1, |r1, r0| (self.edge_comp)(r0, r1));
                    for edge_index in self.graph_0.out_edges(v_new) {
                        let target_index = self.graph_0.get_target_index(&edge_index);
                        if !self.inc_counters_match_edge_0(&mut term_in0_count, &mut term_out0_count, &mut rest0_count, &v_new, &target_index, &w_new, &edge_index, |&w_source, &w_new, r0| edge_predicate.edge_exists(&w_source, &w_new, r0)) {
                            return false;
                        }
                    }
                }


                let mut term_in1_count = 0;
                let mut term_out1_count = 0;
                let mut rest1_count = 0;

                {
                    let mut edge_predicate = EquivalentEdgePredicate::new(self.graph_0, |r0, r1| (self.edge_comp)(r0, r1));
                    for edge_index in self.graph_1.in_edges(&w_new) {
                        let source_index = self.graph_1.get_source_index(&edge_index);
                        if !self.inc_counters_match_edge_1(&mut term_in1_count, &mut term_out1_count, &mut rest1_count, &w_new, &source_index, &v_new, &edge_index, |&v_source, &v_new, r1| edge_predicate.edge_exists(&v_source, &v_new, r1)) {
                            return false;
                        }
                    }
                }
                {
                    let mut edge_predicate = EquivalentEdgePredicate::new(self.graph_0, |r0, r1| (self.edge_comp)(r0, r1));
                    for edge_index in self.graph_1.out_edges(&w_new) {
                        let target_index = self.graph_1.get_target_index(&edge_index);
                        if !self.inc_counters_match_edge_1(&mut term_in1_count, &mut term_out1_count, &mut rest1_count, &w_new, &target_index, &v_new, &edge_index, |&v_source, &v_new, r1| edge_predicate.edge_exists(&v_source, &v_new, r1)) {
                            return false;
                        }
                    }
                }
                term_in0_count <= term_in1_count && term_out0_count <= term_out1_count && rest0_count <= rest1_count
            }
        }

        fn inc_counters_match_edge_0<PREDICATE>(&self, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, v_new: &NID0, v_adj: &NID0, w_new: &NID1, edge_index: &EID0, mut edge_predicate: PREDICATE) -> bool where PREDICATE: FnMut(&NID1, &NID1, &R0) -> bool {
            if self.base_state_0.in_core(v_adj) || v_new == v_adj {
                let mut w_source = *w_new;
                if *v_adj != *v_new {
                    if let Some(ws) =  self.base_state_0.core(v_adj) {
                        w_source = ws;
                    }
                }
                
                let r0 = self.graph_0.get_relationship_ref(&edge_index);

                if !edge_predicate(&w_source, &w_new, r0) {
                    return false;
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
            return true;
        }

        fn inc_counters_match_edge_1<PREDICATE>(&self, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, w_new: &NID1, w_adj: &NID1, v_new: &NID0, edge_index: &EID1, mut edge_predicate: PREDICATE) -> bool where PREDICATE: FnMut(&NID0, &NID0, &R1) -> bool {
            if  self.base_state_1.in_core(w_adj) || w_new == w_adj {
                let mut v_source = *v_new;
                if *w_adj != *w_new {
                    if let Some(vs) = self.base_state_1.core(w_adj) {
                        v_source = vs;
                    }
                }
                
                let r1 = self.graph_1.get_relationship_ref(&edge_index);

                if !edge_predicate(&v_source, &v_new, r1) {
                    return false;
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
            return true;
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

        pub fn call_back<CALLBACK>(&self, callback: &mut CALLBACK) -> bool
        where CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &Graph1) -> bool
        {
            callback(self.base_state_0.get_map(), self.base_state_1.get_map(), self.graph_0, self.graph_1)
        }

}

struct EquivalentEdgePredicate<'g, NID, EID, N, R, RCOMP, Graph, ECOMP> 
    where  NID: MemGraphId, EID: MemGraphId,
    Graph: GraphContainerTrait<NID, EID, N, R>,
    ECOMP: Fn(&R, &RCOMP) -> bool {
    matched_edge_set: HashSet<EID>,
    graph: &'g Graph,
    phantom_v: PhantomData<NID>,
    phantom_e: PhantomData<EID>,
    phantom_n: PhantomData<N>,
    phantom_r_0: PhantomData<R>,
    phantom_r_1: PhantomData<RCOMP>,
    edge_comp: ECOMP,
}

impl <'g, NID, EID, N, R, RCOMP, Graph, ECOMP> EquivalentEdgePredicate<'g, NID, EID, N, R, RCOMP, Graph, ECOMP> 
    where  NID: MemGraphId + Eq, EID: MemGraphId + std::hash::Hash + Eq,
    Graph: GraphContainerTrait<NID, EID, N, R>,
    Graph: GraphIteratorTrait<NID, EID>,
    ECOMP: Fn(&R, &RCOMP) -> bool {

    fn new(g: &'g Graph, ecomp: ECOMP) -> Self {
        EquivalentEdgePredicate {graph: g, matched_edge_set: HashSet::new(), edge_comp: ecomp,
        phantom_e: PhantomData, phantom_n: PhantomData, phantom_v: PhantomData, phantom_r_0: PhantomData, phantom_r_1: PhantomData}
    }

    fn edge_exists(&mut self, source: &NID, target: &NID, rcomp: &RCOMP) -> bool {
        for out_edge_index in self.graph.out_edges(source) {
            let curr_target = self.graph.get_target_index(&out_edge_index);
            let r = self.graph.get_relationship_ref(&out_edge_index);
            if curr_target == *target && !self.matched_edge_set.contains(&out_edge_index) && (self.edge_comp)(r, rcomp) {
                self.matched_edge_set.insert(out_edge_index);
                return true;
            }
        }
        return  false;
    }
}