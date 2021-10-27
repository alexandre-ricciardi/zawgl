use super::gremlin_state::{State, StateContext};
use super::match_vertex_state::MatchVertexState;
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use super::match_in_vertex_state::*;

pub struct MatchOutEdgeState {
    labels: Vec<String>,
}

impl MatchOutEdgeState {
    pub fn new(labels: &Vec<String>) -> Self {
        MatchOutEdgeState{labels: labels.clone()}
    }
}
impl State for MatchOutEdgeState {
    
    fn handle_step(&self, context: &mut StateContext) -> Result<(), StateError> {
        Ok(())
    }
    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, StateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            GStep::InV => {
                Ok(Box::new(MatchInVertexState::new()))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}
