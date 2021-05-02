use super::{StateContext, State};
use one_graph_core::model::*;

pub struct MatchOutEdgeState {
    vid: Option<u64>,
}

impl MatchOutEdgeState {
    pub fn new(vid: Option<u64>) -> Self {
        MatchOutEdgeState{vid: vid}
    }
}
impl State for MatchOutEdgeState {
    
    fn handle_match_vertex(&self, context: &mut StateContext, vid: Option<u64>) {
        let mut n = Node::new();
        n.set_id(vid);
        context.pattern.add_node(n);
    }
    fn handle_add_vertex(&mut self, label: &str) {

    }
    fn handle_add_edge(&mut self, label: &str) {

    }
    fn handle_alias(&mut self, name: &str) {

    }
}
