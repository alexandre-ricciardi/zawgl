use super::gremlin_state::{State, StateContext};
use super::match_vertex_state::MatchVertexState;
use super::super::super::gremlin::*;
use super::gremlin_state::*;

pub struct MatchState {
    bytecodes: Vec<Vec<GStep>>,
}

impl MatchState {
    pub fn new(bytecodes: &Vec<Vec<GStep>>) -> Self {
        MatchState{bytecodes: bytecodes.clone()}
    }
}
impl State for MatchState {
    
    fn handle_step(&self, _context: &mut StateContext) -> Result<(), GremlinStateError> {
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}
