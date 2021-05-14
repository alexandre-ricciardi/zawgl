use super::{StateContext, State};
use one_graph_core::model::*;
use one_graph_gremlin::gremlin::*;
use super::gremlin_state::*;
use super::match_out_edge_state::MatchOutEdgeState;
use std::convert::TryFrom;

pub struct MatchVertexState {
    vid: Option<u64>,
}

impl MatchVertexState {
    pub fn new(gid: &Option<GValue>) -> Self {
        let vid = gid.as_ref().and_then(|value| u64::try_from(value.clone()).ok());
        MatchVertexState{vid: vid}
    }
}

impl State for MatchVertexState {
    
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        let mut n = Node::new();
        n.set_id(self.vid);
        
        match &context.previous_step {
            GStep::Empty => {
                let mut pattern = PropertyGraph::new();
                let nid = pattern.add_node(n);
                context.patterns.push(pattern);
                context.node_index = Some(nid);
            }
            GStep::OutE(labels) => {
                if let Some(node_index) = context.node_index {
                    let mut rel = Relationship::new();
                    rel.set_labels(labels.clone());
                    let pattern = context.patterns.last_mut().ok_or(StateError::Invalid)?;
                    let nid = pattern.add_node(n);
                    pattern.add_relationship(rel, node_index, nid);
                }
            }
            _ => {

            }
        }
        context.previous_step = step.clone();
        match step {
            GStep::OutE(labels) => {
                Ok(Box::new(MatchOutEdgeState::new(labels)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}

