use one_graph_core::graph::traits::GraphContainerTrait;

use super::add_edge_state::AddEdgeState;
use super::alias_vertex_state::AliasVertexState;
use super::gremlin_state::{State, StateContext};
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use super::match_out_edge_state::MatchOutEdgeState;
use super::match_state::MatchState;
use super::super::utils::*;

pub struct HasPropertyState {
    name: String,
    predicate: GPredicate,
}

impl HasPropertyState {
    pub fn new(name: &str, predicate: &GPredicate) -> Self {
        HasPropertyState{name: String::from(name), predicate: predicate.clone()}
    }
}

impl State for HasPropertyState {
    fn handle_step(&self, context: &mut StateContext) -> Result<(), GremlinStateError> {
        if let Some(nid) = context.node_index {
            let pattern = context.patterns.last_mut().ok_or(GremlinStateError::WrongContext("missing pattern"))?;
            let node = pattern.get_node_mut(&nid);
            node.add_predicate(convert_gremlin_predicate_to_pattern_predicate(&self.name, &self.predicate))
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::OutE(_labels) => {
                Ok(Box::new(MatchOutEdgeState::new()))
            }
            GStep::As(alias) => {
                Ok(Box::new(AliasVertexState::new(alias)))
            }
            GStep::Match(bytecodes) => {
                Ok(Box::new(MatchState::new(bytecodes)))
            }
            GStep::AddE(_label) => {
                Ok(Box::new(AddEdgeState::new()))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}
