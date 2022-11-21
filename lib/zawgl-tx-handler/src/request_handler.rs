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

use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use zawgl_core::graph_engine::GraphEngine;
use zawgl_core::model::{PropertyGraph};
use zawgl_core::model::init::InitContext;
use zawgl_cypher_query_model::QueryStep;
use zawgl_cypher_query_planner::handle_query_steps;

use crate::tx_context::TxContext;
use crate::{DatabaseError};


pub type RequestHandler<'a> = Arc<RwLock<GraphRequestHandler<'a>>>;

pub struct GraphRequestHandler <'a> {
    conf: InitContext<'a>,
    map_session_graph_engine: HashMap<String, GraphEngine>,
}

impl <'a> GraphRequestHandler<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GraphRequestHandler{conf: ctx, map_session_graph_engine: HashMap::new()}
    }

    pub fn handle_graph_request(&self, steps: &Vec<QueryStep>) -> Result<Vec<PropertyGraph>, DatabaseError> {
        let mut graph_engine = GraphEngine::new(&self.conf);
        let matched_graphs = handle_query_steps(steps, &mut graph_engine);
        graph_engine.sync();
        Ok(matched_graphs)
    }

    
    pub fn handle_graph_request_tx(&mut self, steps: &Vec<QueryStep>, tx_context: &TxContext) -> Result<Vec<PropertyGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or(DatabaseError::TxError)?;
        let matched_graphs = handle_query_steps(steps, graph_engine);
        Ok(matched_graphs)
    }

    pub fn commit_tx(&mut self, tx_context: & TxContext) -> Result<Vec<PropertyGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or(DatabaseError::TxError)?;
        graph_engine.sync();
        self.map_session_graph_engine.remove(&tx_context.session_id);
        Ok(Vec::new())
    }

    pub fn open_graph_tx(&mut self, tx_context: &TxContext) {
        self.map_session_graph_engine.insert(tx_context.session_id.clone(), GraphEngine::new(&self.conf));
    }
}

