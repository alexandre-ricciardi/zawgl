use super::gremlin_state::{State, StateContext};
use super::match_vertex_state::MatchVertexState;
use super::super::utils::prop_value_from_gremlin_value;
use one_graph_core::{graph::traits::GraphContainerTrait, model::Property};
use super::super::super::gremlin::*;
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
    fn handle_step(&self, context: &mut StateContext) -> Result<(), StateError> {
        match &context.previous_step {
            GStep::AddV(_label) => {
                let pattern = context.patterns.last_mut().ok_or(StateError::Invalid)?;
                if let Some(nid) = &context.node_index {
                    let n = pattern.get_node_mut(nid);
                    let mut prop = Property::new();
                    prop.set_name(&self.name);
                    prop.set_value(Some(prop_value_from_gremlin_value(&self.value)));
                    n.get_properties_mut().push(prop);
                }
                
            },
            GStep::From(_alias) => {
                let pattern = context.patterns.last_mut().ok_or(StateError::Invalid)?;
                if let Some(rid) = &context.relationship_index {
                    let r = pattern.get_relationship_mut(rid);
                    let mut prop = Property::new();
                    prop.set_name(&self.name);
                    prop.set_value(Some(prop_value_from_gremlin_value(&self.value)));
                    r.get_properties_mut().push(prop);
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
            GStep::Empty => {
                Ok(Box::new(EndState::new()))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}
