use super::gremlin_state::{State, StateContext};
use super::super::super::gremlin::*;
use super::set_property_state::SetPropertyState;
use super::super::utils::init_pattern;

use one_graph_core::model::{Node, Status};
use super::super::super::gremlin::*;
use super::gremlin_state::*;

pub struct AddVertexState {
    label: String,
}

impl AddVertexState {
    pub fn new(label: &str) -> Self {
        AddVertexState{label: String::from(label)}
    }
}
impl State for AddVertexState {
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<(), StateError> {
        let mut n = Node::new();
        n.set_status(Status::Create);
        n.get_labels_mut().push(self.label.clone());
        match &context.previous_step {
            GStep::Empty => {
                init_pattern(context, n);
            }
            _ => {}
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        match step {
            GStep::SetProperty(name, value) => {
                Ok(Box::new(SetPropertyState::new(name, value)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}
