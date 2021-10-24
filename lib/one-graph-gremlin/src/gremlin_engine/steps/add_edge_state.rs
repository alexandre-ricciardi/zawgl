use super::gremlin_state::{State, StateContext};
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use super::from_state::FromState;
use super::to_state::ToState;
use super::match_vertex_state::MatchVertexState;

pub struct AddEdgeState {
    label: String,
}

impl AddEdgeState {
    pub fn new(label: &str) -> Self {
        AddEdgeState{label: String::from(label)}
    }
}
impl State for AddEdgeState {
    fn handle_step(&self, _context: &mut StateContext) -> Result<(), StateError> {
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, StateError> {
        match step {
            GStep::From(value) => {
                Ok(Box::new(FromState::new(value).ok_or(StateError::Invalid)?))
            }
            GStep::To(value) => {
                Ok(Box::new(ToState::new(value).ok_or(StateError::Invalid)?))
            }
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}
