use std::marker::PhantomData;
use std::collections::HashSet;
use super::base_state::BaseState;
use super::super::super::graph::traits::*;

pub struct State<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphTrait<'g0, NID0, EID0> + GraphContainerTrait<'g0, NID0, EID0, N0, R0>,
    Graph1: GraphTrait<'g1, NID1, EID1> + GraphContainerTrait<'g1, NID1, EID1, N1, R1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {
    state_0: BaseState<'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, Graph1>,
    state_1: BaseState<'g1, 'g0, NID1, NID0, EID1, EID0, Graph1, Graph0>,
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
    Graph0: GraphTrait<'g0, NID0, EID0> + GraphContainerTrait<'g0, NID0, EID0, N0, R0>,
    Graph1: GraphTrait<'g1, NID1, EID1> + GraphContainerTrait<'g1, NID1, EID1, N1, R1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {
    pub fn push(&mut self, v0: &NID0, v1: &NID1) {
        self.state_0.push(v0, v1);
        self.state_1.push(v1, v0);
    }

    pub fn pop(&mut self, v0: &NID0, v1: &NID1) {
        if let Some(w_val) = self.state_0.core(v0) {
            self.state_0.pop(v0, &w_val);
            self.state_1.pop(&w_val, v0);
        }
    }

    fn feasible(&self, v_new: NID0, w_new: NID1) -> bool {
        let v = self.graph_0.get_node_ref(&v_new);
        let w = self.graph_1.get_node_ref(&w_new);
        if !(self.vertex_comp)(v, w) {
            false
        } else {
            let term_in0_count = 0;
            let term_out0_count = 0;
            let rest0_count = 0;

            let mut edge_set = HashSet::new();
            for edge_index in self.graph_0.in_edges(&v_new) {
                let ancestor_index = self.graph_0.get_source_index(&edge_index);
                if self.state_0.in_core(&ancestor_index) || v_new == *ancestor_index {
                    let mut w_ancestor;
                    if *ancestor_index != v_new {
                        if let Some(w) = self.state_0.core(ancestor_index) {
                            w_ancestor = w;
                        }
                    }
                    
                    let e0 = self.graph_0.get_relationship_ref(&edge_index);

                    for edge_1_index in self.graph_1.out_edges(&w_ancestor) {
                        let w_target = self.graph_1.get_target_index(&edge_1_index);
                        let e1 = self.graph_1.get_relationship_ref(&edge_1_index);
                        if *w_target == w_ancestor && (self.edge_comp)(e0, e1) && !edge_set.contains(&edge_1_index) {
                            edge_set.insert(edge_1_index);
                            
                        }
                    }

                }
            }

            true
        }
    }
}

struct EquivalentEdgeExists<'g, NID, EID, N, R, Graph> 
    where NID: std::hash::Hash + Eq + MemGraphId + Copy, EID: std::hash::Hash + Eq + MemGraphId + Copy,
    N: std::hash::Hash + Eq, R: std::hash::Hash + Eq, 
    Graph: GraphTrait<'g, NID, EID> + GraphContainerTrait<'g, NID, EID, N, R> {
    edge_set: HashSet<NID>,
    graph: &'g Graph
}

impl <'g, NID, EID, N, R, Graph> EquivalentEdgeExists<'g, NID, EID, N, R, Graph> 
    where NID: std::hash::Hash + Eq + MemGraphId + Copy, EID: std::hash::Hash + Eq + MemGraphId + Copy,
    N: std::hash::Hash + Eq, R: std::hash::Hash + Eq, 
    Graph: GraphTrait<'g, NID, EID> + GraphContainerTrait<'g, NID, EID, N, R> {
    fn edge_exists(&self, &ancestor: &NID) -> bool {
        for edge_index in self.graph.out_edges(ancestor) {
            let ancestor_target = self.graph.get_target_index(&edge_index);
            let r = self.graph.get_relationship_ref(&edge_index);
            if *w_target == w_ancestor && (self.edge_comp)(e0, e1) && !edge_set.contains(&edge_1_index) {
                edge_set.insert(edge_1_index);
                return true;
            }
        }
        return  false;
    }
}