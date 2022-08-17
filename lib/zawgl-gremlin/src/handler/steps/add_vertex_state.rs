// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::gremlin_state::{State, StateContext};
use super::super::super::gremlin::*;
use super::set_property_state::SetPropertyState;
use super::super::utils::init_pattern;

use zawgl_core::model::{Node, Status};
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
    fn handle_step(&self, context: &mut StateContext) -> Result<(), GremlinStateError> {
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

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::SetProperty(name, value) => {
                Ok(Box::new(SetPropertyState::new(name, value)))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}
