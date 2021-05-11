use one_graph_gremlin::gremlin::*;
use one_graph_core::model::*;
use one_graph_core::graph::*;
use super::match_out_edge_state::MatchOutEdgeState;
use super::match_vertex_state::MatchVertexState;
#[derive(Debug)]
pub enum StateError {
    Invalid
}

pub trait State {
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError>;
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
    pub relationship_labels: Option<Vec<String>>,
}

impl StateContext {
    pub fn new() -> Self {
        StateContext{pattern: PropertyGraph::new(), node_index: None, relationship_labels: None}
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

    pub fn get_context(&mut self) -> &StateContext {
        &self.context
    }
}
