use super::{StateContext, State};
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
        
        match step {
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}
