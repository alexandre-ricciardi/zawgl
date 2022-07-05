use super::gremlin_state::{State, StateContext};
use super::match_vertex_state::MatchVertexState;
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use super::match_in_vertex_state::*;

pub struct MatchOutEdgeState {

}

impl MatchOutEdgeState {
    pub fn new() -> Self {
        MatchOutEdgeState{}
    }
}
impl State for MatchOutEdgeState {
    
    fn handle_step(&self, _context: &mut StateContext) -> Result<(), GremlinStateError> {
        Ok(())
    }
    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            GStep::InV => {
                Ok(Box::new(MatchInVertexState::new()))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}
