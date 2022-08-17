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
use super::match_vertex_state::MatchVertexState;
use super::super::utils::prop_value_from_gremlin_value;
use zawgl_core::{graph::traits::GraphContainerTrait, model::Property};
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
    fn handle_step(&self, context: &mut StateContext) -> Result<(), GremlinStateError> {
        match &context.previous_step {
            GStep::AddV(_label) => {
                let pattern = context.patterns.last_mut().ok_or(GremlinStateError::WrongContext("missing pattern"))?;
                if let Some(nid) = &context.node_index {
                    let n = pattern.get_node_mut(nid);
                    let prop = Property::new(self.name.clone(), prop_value_from_gremlin_value(&self.value));
                    n.get_properties_mut().push(prop);
                }
                
            },
            GStep::From(_alias) => {
                let pattern = context.patterns.last_mut().ok_or(GremlinStateError::WrongContext("missing pattern"))?;
                if let Some(rid) = &context.relationship_index {
                    let r = pattern.get_relationship_mut(rid);
                    let prop = Property::new(self.name.clone(), prop_value_from_gremlin_value(&self.value));
                    r.get_properties_mut().push(prop);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            GStep::Empty => {
                Ok(Box::new(EndState::new()))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}
