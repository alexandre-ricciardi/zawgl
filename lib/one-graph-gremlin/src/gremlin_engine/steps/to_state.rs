use super::gremlin_state::{State, StateContext};
use super::match_vertex_state::MatchVertexState;
use super::set_property_state::SetPropertyState;
use one_graph_core::model::{Relationship, Status, Node};
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use std::convert::TryFrom;

pub struct ToState {
    target: Target,
}

enum Target {
    Alias(String),
    VertexId(u64),
}

impl ToState {
    pub fn new(target: &GValueOrVertex) -> Option<Self> {
        match target {
            GValueOrVertex::Value(value) => {Some(ToState{target: Target::Alias(String::from(value.as_str()?))})}
            GValueOrVertex::Vertex(vertex) => {Some(ToState{target: Target::VertexId(u64::try_from(vertex.id.clone()).ok()?)})}
        }
    }
}
impl State for ToState {


    fn handle_step(&self, context: &mut StateContext) -> Result<(), StateError> {
        match &context.previous_step {
            GStep::AddE(label) => {
                let mut r = Relationship::new();
                r.set_labels(vec![label.clone()]);
                r.set_status(Status::Create);
                let pattern = context.patterns.last_mut().ok_or(StateError::Invalid)?;
                let source_id = context.node_index;
                let target_id = match &self.target {
                    Target::Alias(a) => {*context.node_aliases.get(a).ok_or(StateError::Invalid)?}
                    Target::VertexId(vid) => {
                        let mut target = Node::new();
                        target.set_id(Some(*vid));
                        target.set_status(Status::Match);
                        pattern.add_node(target)
                    }
                };
                if let Some(sid) = source_id {
                    let rid = pattern.add_relationship(r, sid, target_id);
                    context.relationship_index = Some(rid);
                } else {
                    return Err(StateError::Invalid);
                }
            }
            _ => {} 
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, StateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            GStep::SetProperty(name, value) => {
                Ok(Box::new(SetPropertyState::new(name, value)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}