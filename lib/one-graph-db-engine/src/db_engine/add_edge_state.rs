use super::{State, StateContext, match_vertex_state::MatchVertexState};
use one_graph_gremlin::gremlin::*;
use super::gremlin_state::*;

pub struct AddEdgeState {
    labels: Vec<String>,
}

impl AddEdgeState {
    pub fn new(labels: &Vec<String>) -> Self {
        AddEdgeState{labels: labels.clone()}
    }
}
impl State for AddEdgeState {
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
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
