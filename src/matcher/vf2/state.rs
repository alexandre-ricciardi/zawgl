use super::base_state::BaseState;
use super::super::super::graph::container::{GraphTrait, GraphContainerTrait};
use super::super::super::graph::{NodeIndex};

pub struct State<'g0, 'g1, V0: Eq, V1: Eq, R0: Eq, R1: Eq> {
    state_0: BaseState<'g0, 'g1>,
    state_1: BaseState<'g0, 'g1>,
    graph_0: &'g0 dyn GraphContainerTrait<V0, R0>,
    graph_1: &'g1 dyn GraphContainerTrait<V1, R1>,
}

impl <'g0, 'g1, V0: Eq, V1: Eq, R0: Eq, R1: Eq> State<'g0, 'g1, V0, V1, R0, R1> {
    pub fn push(&mut self, v0: NodeIndex, v1: NodeIndex) {
        self.state_0.push(v0, v1);
        self.state_1.push(v1, v0);
    }

    pub fn pop(&mut self, v0: NodeIndex, v1: NodeIndex) {
        let w = self.state_0.core(v0);
        if let Some(w_val) = w {
            self.state_0.pop(v0, w_val);
            self.state_1.pop(w_val, v0);
        }
    }

    fn feasible(&self, v_new: NodeIndex, w_new: NodeIndex) -> bool {
        let v = self.graph_0.get_node_ref(v_new);
        let w = self.graph_1.get_node_ref(w_new);
        if v != w {
            false
        } else {

            true
        }
    }
}