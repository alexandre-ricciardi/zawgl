use super::gremlin_state::{State, StateContext};
use super::match_vertex_state::MatchVertexState;
use super::super::super::gremlin::*;
use super::gremlin_state::*;

pub struct AliasVertexState {
    name: String,
}

impl AliasVertexState {
    pub fn new(name: &str) -> Self {
        AliasVertexState{name: String::from(name)}
    }
}
impl State for AliasVertexState {

    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<(), StateError> {
        match &context.previous_step {
            GStep::V(_vid) => {
                if let Some(nid) = context.node_index {
                    context.node_aliases.insert(self.name.clone(), nid);
                }
            }
            _ => {}
        }
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
