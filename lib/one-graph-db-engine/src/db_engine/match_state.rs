use super::{State, StateContext, match_vertex_state::MatchVertexState};
use one_graph_core::model::*;
use one_graph_gremlin::gremlin::*;
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
    
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<(), StateError> {
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
