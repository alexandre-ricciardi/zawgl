use super::gremlin_state::{State, StateContext};
use super::alias_vertex_state::AliasVertexState;
use one_graph_core::model::*;
use one_graph_gremlin::gremlin::*;
use super::gremlin_state::*;
use super::match_out_edge_state::MatchOutEdgeState;
use super::match_state::MatchState;
use super::add_edge_state::AddEdgeState;
use std::convert::TryFrom;
use super::super::utils::*;

pub struct MatchInVertexState {
}

impl MatchInVertexState {
    pub fn new() -> Self {
        MatchInVertexState{}
    }
}

impl State for MatchInVertexState {
    
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<(), StateError> {
        let mut n = Node::new();
        n.set_status(Status::Match);

        match &context.previous_step {
            GStep::OutE(labels) => {
                if let Some(node_index) = context.node_index {
                    let mut rel = Relationship::new();
                    rel.set_labels(labels.clone());
                    rel.set_status(Status::Match);
                    let pattern = context.patterns.last_mut().ok_or(StateError::Invalid)?;
                    let nid = pattern.add_node(n);
                    pattern.add_relationship(rel, node_index, nid);
                    context.node_index = Some(nid);
                }
            }
            _ => {

            }
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        match step {
            GStep::OutE(labels) => {
                Ok(Box::new(MatchOutEdgeState::new(labels)))
            }
            GStep::As(alias) => {
                Ok(Box::new(AliasVertexState::new(alias)))
            }
            GStep::Match(bytecodes) => {
                Ok(Box::new(MatchState::new(bytecodes)))
            }
            GStep::AddE(label) => {
                Ok(Box::new(AddEdgeState::new(label)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}

