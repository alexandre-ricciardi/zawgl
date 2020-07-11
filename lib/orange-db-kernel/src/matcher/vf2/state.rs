use std::marker::PhantomData;
use std::collections::HashSet;
use std::collections::HashMap;
use super::base_state::*;
use super::super::super::graph::traits::*;

pub struct State<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<'g0, NID0, EID0, N0, R0>,
    Graph1: GraphContainerTrait<'g1, NID1, EID1, N1, R1> + GrowableGraph<NID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {
    state_0: SubState<'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, Graph1>,
    state_1: SubState<'g1, 'g0, NID1, NID0, EID1, EID0, Graph1, Graph0>,
    graph_0: &'g0 Graph0,
    graph_1: &'g1 Graph1,
    vertex_comp: VCOMP,
    edge_comp: ECOMP,
    phantom_v_0: PhantomData<N0>,
    phantom_v_1: PhantomData<N1>,
    phantom_r_0: PhantomData<R0>,
    phantom_r_1: PhantomData<R1>,
    phantom_e_0: PhantomData<EID0>,
    phantom_e_1: PhantomData<EID1>,

}

impl <'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>
    State<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<'g0, NID0, EID0, N0, R0>,
    Graph1: GraphContainerTrait<'g1, NID1, EID1, N1, R1> + GrowableGraph<NID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {


        pub fn new(graph_0: &'g0 Graph0, graph_1: &'g1 Graph1, vcomp: VCOMP, ecomp: ECOMP) -> Self {
            State {
                state_0: SubState::new(graph_0, graph_1),
                state_1: SubState::new(graph_1, graph_0),
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
            }
        }

        pub fn push(&mut self, v0: &NID0, v1: &NID1) {
            self.state_0.push(v0, v1);
            self.state_1.push(v1, v0);
        }

        pub fn pop(&mut self, v0: &NID0, _v1: &NID1) {
            if let Some(w_val) = self.state_0.get_base_state().core(v0) {
                self.state_0.pop(v0, &w_val);
                self.state_1.pop(&w_val, v0);
            }
        }

        pub fn feasible(&mut self, v_new: &NID0, w_new: &NID1) -> bool {
            let v = self.graph_0.get_node_ref(&v_new);
            let w = self.graph_1.get_node_ref(&w_new);
            if !(self.vertex_comp)(v, w) {
                false
            } else {
                let mut term_in0_count = 0;
                let mut term_out0_count = 0;
                let mut rest0_count = 0;

                {
                    let mut edge_predicate = EquivalentEdgePredicate::new(self.graph_1, |r1, r0| (self.edge_comp)(r0, r1));
                    for edge_index in self.graph_0.in_edges(&v_new) {
                        let source_index = self.graph_0.get_source_index(&edge_index);
                        if !self.inc_counters_match_edge_0(&mut term_in0_count, &mut term_out0_count, &mut rest0_count, &v_new, source_index, &w_new, &edge_index, |&w_source, &w_new, r0| edge_predicate.edge_exists(&w_source, &w_new, r0)) {
                            return false;
                        }
                    }
                }
                {
                    let mut edge_predicate = EquivalentEdgePredicate::new(self.graph_1, |r1, r0| (self.edge_comp)(r0, r1));
                    for edge_index in self.graph_0.out_edges(&v_new) {
                        let target_index = self.graph_0.get_target_index(&edge_index);
                        if !self.inc_counters_match_edge_0(&mut term_in0_count, &mut term_out0_count, &mut rest0_count, &v_new, target_index, &w_new, &edge_index, |&w_source, &w_new, r0| edge_predicate.edge_exists(&w_source, &w_new, r0)) {
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
                        if !self.inc_counters_match_edge_1(&mut term_in1_count, &mut term_out1_count, &mut rest1_count, &w_new, source_index, &v_new, &edge_index, |&v_source, &v_new, r1| edge_predicate.edge_exists(&v_source, &v_new, r1)) {
                            return false;
                        }
                    }
                }
                {
                    let mut edge_predicate = EquivalentEdgePredicate::new(self.graph_0, |r0, r1| (self.edge_comp)(r0, r1));
                    for edge_index in self.graph_1.out_edges(&w_new) {
                        let target_index = self.graph_1.get_target_index(&edge_index);
                        if !self.inc_counters_match_edge_1(&mut term_in1_count, &mut term_out1_count, &mut rest1_count, &w_new, target_index, &v_new, &edge_index, |&v_source, &v_new, r1| edge_predicate.edge_exists(&v_source, &v_new, r1)) {
                            return false;
                        }
                    }
                }
                term_in0_count <= term_in1_count && term_out0_count <= term_out1_count && rest0_count <= rest1_count
            }
        }

        fn inc_counters_match_edge_0<PREDICATE>(&self, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, v_new: &NID0, v_adj: &NID0, w_new: &NID1, edge_index: &EID0, mut edge_predicate: PREDICATE) -> bool where PREDICATE: FnMut(&NID1, &NID1, &R0) -> bool {
            if self.state_0.get_base_state().in_core(v_adj) || v_new == v_adj {
                let mut w_source = *w_new;
                if *v_adj != *v_new {
                    if let Some(ws) = self.state_0.get_base_state().core(v_adj) {
                        w_source = ws;
                    }
                }
                
                let r0 = self.graph_0.get_relationship_ref(&edge_index);

                if !edge_predicate(&w_source, &w_new, r0) {
                    return false;
                }
            } else {
                if self.state_0.get_base_state().in_depth(v_adj) > 0 {
                    *term_in += 1;
                }
                if self.state_0.get_base_state().out_depth(v_adj) > 0 {
                    *term_out += 1;
                }
                if self.state_0.get_base_state().in_depth(v_adj) == 0 && self.state_0.get_base_state().out_depth(v_adj) == 0 {
                    *rest += 1;
                }
            }
            return true;
        }

        fn inc_counters_match_edge_1<PREDICATE>(&self, term_in: &mut i32, term_out: &mut i32, rest: &mut i32, w_new: &NID1, w_adj: &NID1, v_new: &NID0, edge_index: &EID1, mut edge_predicate: PREDICATE) -> bool where PREDICATE: FnMut(&NID0, &NID0, &R1) -> bool {
            if self.state_1.get_base_state().in_core(w_adj) || w_new == w_adj {
                let mut v_source = *v_new;
                if *w_adj != *w_new {
                    if let Some(vs) = self.state_1.get_base_state().core(w_adj) {
                        v_source = vs;
                    }
                }
                
                let r0 = self.graph_1.get_relationship_ref(&edge_index);

                if !edge_predicate(&v_source, &v_new, r0) {
                    return false;
                }
            } else {
                if self.state_1.get_base_state().in_depth(w_adj) > 0 {
                    *term_in += 1;
                }
                if self.state_1.get_base_state().out_depth(w_adj) > 0 {
                    *term_out += 1;
                }
                if self.state_1.get_base_state().in_depth(w_adj) == 0 && self.state_1.get_base_state().out_depth(w_adj) == 0 {
                    *rest += 1;
                }
            }
            return true;
        }

        pub fn possible_candidate_0(&self, v0: &NID0) -> bool {
            if self.state_0.get_base_state().term_both() && self.state_1.get_base_state().term_both() {
                self.state_0.get_base_state().term_both_vertex(v0)
            } else if self.state_0.get_base_state().term_out() && self.state_1.get_base_state().term_out() {
                self.state_0.get_base_state().term_out_vertex(v0)
            } else if self.state_0.get_base_state().term_in() && self.state_1.get_base_state().term_in() {
                self.state_0.get_base_state().term_in_vertex(v0)
            } else {
                !self.state_0.get_base_state().in_core(v0)
            }
        }

        pub fn possible_candidate_1(&self, v1: &NID1) -> bool {
            if self.state_0.get_base_state().term_both() && self.state_1.get_base_state().term_both() {
                self.state_1.get_base_state().term_both_vertex(v1)
            } else if self.state_0.get_base_state().term_out() && self.state_1.get_base_state().term_out() {
                self.state_1.get_base_state().term_out_vertex(v1)
            } else if self.state_0.get_base_state().term_in() && self.state_1.get_base_state().term_in() {
                self.state_1.get_base_state().term_in_vertex(v1)
            } else {
                !self.state_1.get_base_state().in_core(v1)
            }
        }

        pub fn success(&self) -> bool {
            self.state_0.get_base_state().count() == self.graph_0.nodes_len()
        }

        pub fn valid(&self) -> bool {
            let term_set_0 = self.state_0.get_base_state().term_set();
            let term_set_1 = self.state_1.get_base_state().term_set();
            term_set_0.0 <= term_set_1.0 && term_set_0.1 <= term_set_1.1 && term_set_0.2 <= term_set_1.2
        }

        pub fn call_back<CALLBACK>(&self, callback: &mut CALLBACK) -> bool
        where CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &'g0 Graph0, &'g1 Graph1) -> bool
        {
            callback(self.state_0.get_base_state().get_map(), self.state_1.get_base_state().get_map(), self.graph_0, self.graph_1)
        }
}

struct EquivalentEdgePredicate<'g, NID, EID, N, R, RCOMP, Graph, ECOMP> 
    where  NID: MemGraphId, EID: MemGraphId,
    Graph: GraphContainerTrait<'g, NID, EID, N, R>,
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
    Graph: GraphContainerTrait<'g, NID, EID, N, R>,
    ECOMP: Fn(&R, &RCOMP) -> bool {

    fn new(g: &'g Graph, ecomp: ECOMP) -> Self {
        EquivalentEdgePredicate {graph: g, matched_edge_set: HashSet::new(), edge_comp: ecomp,
        phantom_e: PhantomData, phantom_n: PhantomData, phantom_v: PhantomData, phantom_r_0: PhantomData, phantom_r_1: PhantomData}
    }

    fn edge_exists(&mut self, source: &NID, target: &NID, rcomp: &RCOMP) -> bool {
        for out_edge_index in self.graph.out_edges(source) {
            let curr_target = self.graph.get_target_index(&out_edge_index);
            let r = self.graph.get_relationship_ref(&out_edge_index);
            if *curr_target == *target && !self.matched_edge_set.contains(&out_edge_index) && (self.edge_comp)(r, rcomp) {
                self.matched_edge_set.insert(out_edge_index);
                return true;
            }
        }
        return  false;
    }
}