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

use zawgl_core::graph::traits::GraphContainerTrait;

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
