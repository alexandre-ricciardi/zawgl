use super::{State, StateContext, match_vertex_state::MatchVertexState};
use one_graph_core::model::*;
use one_graph_gremlin::gremlin::*;
use super::gremlin_state::*;

pub struct MatchOutEdgeState {
    labels: Vec<String>,
}

impl MatchOutEdgeState {
    pub fn new(labels: &Vec<String>) -> Self {
        MatchOutEdgeState{labels: labels.clone()}
    }
}
impl State for MatchOutEdgeState {
    
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        context.relationship_labels = Some(self.labels.clone());
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
