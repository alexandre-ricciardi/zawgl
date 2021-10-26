use one_graph_core::graph::traits::GraphContainerTrait;

use super::gremlin_state::{State, StateContext};
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use super::from_state::FromState;
use super::to_state::ToState;
use super::match_vertex_state::MatchVertexState;

pub struct HasPropertyState {
    name: String,
    predicate: Predicate,
}

impl HasPropertyState {
    pub fn new(name: &str, predicate: &Predicate) -> Self {
        HasPropertyState{name: String::from(name), predicate: predicate.clone()}
    }
}
impl State for HasPropertyState {
    fn handle_step(&self, context: &mut StateContext) -> Result<(), GremlinStateError> {
        if let Some(nid) = context.node_index {
            let pattern = context.patterns.last_mut().ok_or(GremlinStateError::WrongContext("missing pattern"))?;
            let node = pattern.get_node_mut(&nid);
            node.set_properties(properties)
        }
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
