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

use std::cell::RefCell;
use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use std::sync::Mutex;
use zawgl_core::graph_engine::GraphEngine;
use zawgl_core::model::{PropertyGraph};
use zawgl_core::model::init::InitContext;
use zawgl_cypher_query_model::QueryStep;
use crate::planner::handle_query_steps;

use crate::tx_handler::tx_context::TxContext;
use super::{DatabaseError};


pub type RequestHandler = Arc<Mutex<GraphRequestHandler>>;

pub struct GraphRequestHandler {
    conf: InitContext,
    map_session_graph_engine: HashMap<String, GraphEngine>,
    graph_engine: GraphEngine,
}

impl <'a> GraphRequestHandler {
    pub fn new(ctx: InitContext) -> Self {
        let graph_engine = GraphEngine::new(ctx.clone());
        GraphRequestHandler{conf: ctx, map_session_graph_engine: HashMap::new(), graph_engine }
    }

    pub fn handle_graph_request(&mut self, steps: Vec<QueryStep>) -> Result<Vec<PropertyGraph>, DatabaseError> {
        
        let matched_graphs = handle_query_steps(steps, &mut self.graph_engine).ok_or(DatabaseError::EngineError)?;
        self.commit_graph_request();
        Ok(matched_graphs)
    }

    pub fn commit_graph_request(&mut self) {
        self.graph_engine.sync();
    }

    
    pub fn handle_graph_request_tx(&mut self, steps: Vec<QueryStep>, tx_context: TxContext) -> Result<Vec<PropertyGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or(DatabaseError::TxError)?;
        let matched_graphs = handle_query_steps(steps, graph_engine).ok_or(DatabaseError::EngineError)?;
        Ok(matched_graphs)
    }

    pub fn commit_tx(&mut self, tx_context: TxContext) -> Result<Vec<PropertyGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or(DatabaseError::TxError)?;
        graph_engine.sync();
        self.map_session_graph_engine.remove(&tx_context.session_id);
        Ok(Vec::new())
    }

    pub fn open_graph_tx(&mut self, tx_context: TxContext) {
        self.map_session_graph_engine.insert(tx_context.session_id.clone(), GraphEngine::new(self.conf.clone()));
    }
}

