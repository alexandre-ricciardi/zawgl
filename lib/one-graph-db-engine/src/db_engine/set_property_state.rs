use super::{State, StateContext, match_vertex_state::MatchVertexState};
use one_graph_gremlin::gremlin::*;
use super::gremlin_state::*;

pub struct SetPropertyState {
    name: String,
    value: GValue,
}

impl SetPropertyState {
    pub fn new(name: &str, value: &GValue) -> Self {
        SetPropertyState{name: String::from(name), value: value.clone()}
    }
}
impl State for SetPropertyState {
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
