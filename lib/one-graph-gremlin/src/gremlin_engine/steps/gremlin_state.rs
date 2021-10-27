use std::collections::HashMap;

use super::super::super::gremlin::*;
use one_graph_core::model::*;
use one_graph_core::graph::*;
use super::add_vertex_state::AddVertexState;
use super::match_vertex_state::MatchVertexState;

#[derive(Debug, Clone)]
pub enum GremlinStateError {
    Invalid(GStep),
    WrongContext(&'static str),
}

pub trait State {
    fn handle_step(&self, context: &mut StateContext) -> Result<(), GremlinStateError>;
    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError>;
}


pub struct InitState {
}

impl InitState {
    pub fn new() -> Self {
        InitState{}
    }
}
impl State for InitState {
    
    fn handle_step(&self, _context: &mut StateContext) -> Result<(), GremlinStateError> {
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            GStep::AddV(label) => {
                Ok(Box::new(AddVertexState::new(label)))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}


pub struct EndState {
}

impl EndState {
    pub fn new() -> Self {
        EndState{}
    }
}
impl State for EndState {
    
    fn handle_step(&self, _context: &mut StateContext) -> Result<(), GremlinStateError> {
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::Empty => {
                Ok(Box::new(InitState::new()))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}

pub struct StateContext {
    pub patterns: Vec<PropertyGraph>,
    pub node_index: Option<NodeIndex>,
    pub relationship_index: Option<EdgeIndex>,
    pub previous_step: GStep,
    pub node_aliases: HashMap<String, NodeIndex>,
    pub add_edge_label: Option<String>,
}

impl StateContext {
    pub fn new() -> Self {
        StateContext{patterns: Vec::new(), node_index: None, relationship_index: None, previous_step: GStep::Empty,
             node_aliases: HashMap::new(), add_edge_label: None}
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
    
    pub fn new_step_state(mut previous: GremlinStateMachine, previous_step: &GStep, current_step: &GStep) -> Result<Self, GremlinStateError> {
        previous.state.handle_step(&mut previous.context)?;
        let new_state = previous.state.create_state(current_step)?;
        previous.context.previous_step = previous_step.clone();
        Ok(GremlinStateMachine{context: previous.context, state: new_state})
    }
}