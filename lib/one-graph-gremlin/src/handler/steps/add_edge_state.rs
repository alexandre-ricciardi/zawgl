use super::gremlin_state::{State, StateContext};
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use super::from_state::FromState;
use super::to_state::ToState;
use super::match_vertex_state::MatchVertexState;

pub struct AddEdgeState {

}

impl AddEdgeState {
    pub fn new() -> Self {
        AddEdgeState{}
    }
}
impl State for AddEdgeState {
    fn handle_step(&self, _context: &mut StateContext) -> Result<(), GremlinStateError> {
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::From(value) => {
                Ok(Box::new(FromState::new(value).ok_or(GremlinStateError::Invalid(step.clone()))?))
            }
            GStep::To(value) => {
                Ok(Box::new(ToState::new(value).ok_or(GremlinStateError::Invalid(step.clone()))?))
            }
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}
