use std::marker::PhantomData;
use std::collections::HashSet;
use super::base_state::BaseState;
use super::super::super::graph::traits::{GraphTrait, GraphContainerTrait};
use super::super::super::graph::{NodeIndex};

pub struct State<'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, OutIt0, InIt0, Graph1, OutIt1, InIt1, V0, R0, V1, R1, VCOMP, ECOMP>
    where NID0: std::hash::Hash + Eq, NID1: std::hash::Hash + Eq,
    EID0: std::hash::Hash + Eq, EID1: std::hash::Hash + Eq, 
    Graph0: GraphTrait<NID0, EID0, OutIt0, InIt0>, Graph1: GraphTrait<NID0, EID0, OutIt1, InIt1>,
    VCOMP: Fn(&V0, &V1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {
    state_0: BaseState<'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, OutIt0, InIt0, Graph1, OutIt1, InIt1>,
    state_1: BaseState<'g1, 'g0, NID1, NID0, EID1, EID0, Graph1, OutIt1, InIt1, Graph0, OutIt0, InIt0>,
    graph_0: &'g0 Graph0,
    graph_1: &'g1 Graph1,
    vertex_comp: VCOMP,
    edge_comp: ECOMP,
    phantom_v_0: PhantomData<V0>,
    phantom_v_1: PhantomData<V1>,
    phantom_r_0: PhantomData<R0>,
    phantom_r_1: PhantomData<R1>,

}

impl <'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, OutIt0, InIt0, Graph1, OutIt1, InIt1, V0, R0, V1, R1, VCOMP, ECOMP>
    State<'g0, 'g1, NID0, NID1, EID0, EID1, Graph0, OutIt0, InIt0, Graph1, OutIt1, InIt1, V0, R0, V1, R1, VCOMP, ECOMP>
    where NID0: std::hash::Hash + Eq, NID1: std::hash::Hash + Eq,
    EID0: std::hash::Hash + Eq, EID1: std::hash::Hash + Eq, 
    Graph0: GraphTrait<NID0, EID0, OutIt0, InIt0>, Graph1: GraphTrait<NID0, EID0, OutIt1, InIt1>,
    VCOMP: Fn(&V0, &V1) -> bool, ECOMP: Fn(&R0, &R1) -> bool {
    pub fn push(&mut self, v0: NID0, v1: NID1) {
        self.state_0.push(v0, v1);
        self.state_1.push(v1, v0);
    }

    pub fn pop(&mut self, v0: NID0, v1: NID1) {
        let w = self.state_0.core(v0);
        if let Some(w_val) = w {
            self.state_0.pop(v0, w_val);
            self.state_1.pop(w_val, v0);
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

            //let mut edge_set = HashSet::new();
            for edge_index in self.graph_0.in_edges(&v_new) {
                let ancestor_index = self.graph_0.get_source_index(&edge_index);
                if self.state_0.in_core(ancestor_index) || v_new == ancestor_index {
                    let mut w = Some(w_new);
                    if ancestor_index != v_new {
                        w = self.state_0.core(ancestor_index);
                        let e0 = self.graph_0.get_relationship_ref(&edge_index);
                        for edges_1_index in self.graph_1.out_edges(&w.unwrap()) {
                            let e1 = self.graph_1.get_relationship_ref(&edges_1_index);
                            if (self.edge_comp)(e0, e1) {
                                
                            }
                        }

                    }
                            
                }
            }

            true
        }
    }
}