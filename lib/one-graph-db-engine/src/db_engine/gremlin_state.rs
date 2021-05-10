use one_graph_gremlin::gremlin::*;
use one_graph_core::model::*;
use one_graph_core::graph::*;
use std::convert::TryFrom;
use super::match_out_edge_state::MatchOutEdgeState;

#[derive(Debug)]
pub enum StateError {
    Invalid
}

pub trait State {
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError>;
}

struct MatchVertexState {
    vid: Option<u64>,
}

impl MatchVertexState {
    fn new(gid: &Option<GValue>) -> Self {
        let vid = gid.as_ref().and_then(|value| u64::try_from(value.clone()).ok());
        MatchVertexState{vid: vid}
    }
}

impl State for MatchVertexState {
    
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        let mut n = Node::new();
        n.set_id(self.vid);
        context.node_index = Some(context.pattern.add_node(n));
        match step {
            GStep::OutE(labels) => {
                Ok(Box::new(MatchOutEdgeState::new(labels)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}



pub struct InitState {
}

impl InitState {
    pub fn new() -> Self {
        InitState{}
    }
}
impl State for InitState {
    
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}

pub struct StateContext {
    pub pattern: PropertyGraph,
    pub node_index: Option<NodeIndex>,
}

impl StateContext {
    pub fn new() -> Self {
        StateContext{pattern: PropertyGraph::new(), node_index: None}
    }
}

pub struct GremlinStateMachine {
    context: StateContext,
    state: Box<dyn State>,
}

impl GremlinStateMachine {
    pub fn new() -> Self {
        GremlinStateMachine{context: StateContext::new(), state: Box::new(InitState::new())}
    }

    pub fn new_step_state(mut previous: GremlinStateMachine, step: &GStep) -> Option<Self> {
        let new_state = previous.state.handle_step(step, &mut previous.context).ok()?;
        Some(GremlinStateMachine{context: previous.context, state: new_state})
    }
}
