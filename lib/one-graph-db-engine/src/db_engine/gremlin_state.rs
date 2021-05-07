use one_graph_gremlin::gremlin::*;
use one_graph_core::model::*;
use std::convert::TryFrom;
use super::match_out_edge_state::MatchOutEdgeState;

#[derive(Debug)]
pub enum StateError {
    Invalid
}

pub trait State {
    fn handle_match_vertex(&self, context: &mut StateContext, vid: Option<u64>);
    fn handle_add_vertex(&mut self, label: &str);
    fn handle_add_edge(&mut self, label: &str);
    fn handle_alias(&mut self, name: &str);
}

struct MatchVertexState {
    vid: Option<u64>,
}

impl MatchVertexState {
    fn new(vid: Option<u64>) -> Self {
        MatchVertexState{vid: vid}
    }
}
impl State for MatchVertexState {
    
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

struct InitState {
}

impl InitState {
    fn new() -> Self {
        InitState{}
    }
}
impl State for InitState {
    
    fn handle_match_vertex(&self, context: &mut StateContext, vid: Option<u64>) {

    }
    fn handle_add_vertex(&mut self, label: &str) {

    }
    fn handle_add_edge(&mut self, label: &str) {

    }
    fn handle_alias(&mut self, name: &str) {

    }
}

pub struct StateContext {
    pub pattern: PropertyGraph,
}

impl StateContext {
    pub fn new() -> Self {
        StateContext{pattern: PropertyGraph::new()}
    }
}

pub struct GremlinStateMachine {
    context: StateContext,
}

impl GremlinStateMachine {
    pub fn new() -> Self {
        GremlinStateMachine{context: StateContext::new()}
    }

    pub fn new_match_vertex_state(mut previous: GremlinStateMachine, gid: &Option<GValue>) -> Self {
        let vid = gid.as_ref().and_then(|value| u64::try_from(value.clone()).ok());
        let mut state = MatchVertexState::new(vid);
        state.handle_match_vertex(&mut previous.context, vid);
        GremlinStateMachine{context: previous.context}
    }

    pub fn new_match_edge_state(mut previous: GremlinStateMachine, gid: &Option<GValue>) -> Self {
        let vid = gid.as_ref().and_then(|value| u64::try_from(value.clone()).ok());
        let mut state = MatchOutEdgeState::new(vid);
        state.handle_match_vertex(&mut previous.context, vid);
        GremlinStateMachine{context: previous.context}
    }
}
