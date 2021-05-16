use std::collections::HashMap;

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
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<(), StateError>;
    fn create_state(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError>;
}


pub struct InitState {
}

impl InitState {
    pub fn new() -> Self {
        InitState{}
    }
}
impl State for InitState {
    
    fn handle_step(&self, step: &GStep, _context: &mut StateContext) -> Result<(), StateError> {
        Ok(())
    }

    fn create_state(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
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
    pub patterns: Vec<PropertyGraph>,
    pub node_index: Option<NodeIndex>,
    pub previous_step: GStep,
    pub node_aliases: HashMap<String, NodeIndex>,
}

impl StateContext {
    pub fn new() -> Self {
        StateContext{patterns: Vec::new(), node_index: None, previous_step: GStep::Empty, node_aliases: HashMap::new()}
    }
}

pub struct GremlinStateMachine {
    pub context: StateContext,
    state: Box<dyn State>,
}

impl GremlinStateMachine {
    pub fn new() -> Self {
        GremlinStateMachine{context: StateContext::new(), state: Box::new(InitState::new())}
    }
    
    pub fn new_step_state(mut previous: GremlinStateMachine, step: &GStep) -> Option<Self> {
        previous.state.handle_step(step, &mut previous.context).ok()?;
        let new_state = previous.state.create_state(step, &mut previous.context).ok()?;
        previous.context.previous_step = step.clone();
        Some(GremlinStateMachine{context: previous.context, state: new_state})
    }
}