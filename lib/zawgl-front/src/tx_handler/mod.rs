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

pub mod handler;
pub mod tx_context;
pub mod request_handler;
use std::{fmt, future::Future, sync::Arc};
use log::*;

use request_handler::RequestHandler;
use handler::{Scenario, TxHandler, TxStatus, needs_write_lock};

use zawgl_core::model::PropertyGraph;
use zawgl_cypher_query_model::QueryStep;
use self::tx_context::TxContext;

pub struct ResultGraph {
    pub scenario: Scenario,
    pub patterns: Vec<PropertyGraph>,
}

#[derive(Debug, Clone)]
pub enum DatabaseError {
    EngineError,
    TxError,
}


impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DatabaseError::EngineError => f.write_str("graph engine error"),
            DatabaseError::TxError => f.write_str("tx error"),
        }
    }
}

pub fn handle_graph_request(tx_handler: TxHandler, graph_request_handler: RequestHandler, steps: Vec<QueryStep>, tx_context: Option<TxContext>) -> Result<Vec<PropertyGraph>, DatabaseError> {
    let tx_lock = tx_handler.lock();
    let tx_status = tx_lock.borrow_mut().get_session_status(tx_context.clone());
    match tx_status {
        TxStatus::OpenNewTx(ctx) => {
            trace!("Open new TX {}", ctx.session_id);
            tx_lock.borrow_mut().acquire_session_lock();
            graph_request_handler.lock().unwrap().open_graph_tx(ctx.clone());
            graph_request_handler.lock().unwrap().handle_graph_request_tx(steps, ctx.clone())
        },
        TxStatus::ContinueCurrentTx(ctx) => {
            trace!("Continue current TX {}", ctx.session_id);
            graph_request_handler.lock().unwrap().handle_graph_request_tx(steps, ctx)
        },
        TxStatus::CommitCurrentTx(ctx) => { 
            trace!("Commit current TX {}", ctx.session_id);
            tx_lock.borrow_mut().release_session_lock();
            graph_request_handler.lock().unwrap().commit_tx(ctx)
        },
        TxStatus::WaitForCurrentTx => {
            trace!("Wait for current TX {:?}", tx_context);
            tx_lock.borrow_mut().acquire_session_lock();
            handle_graph_request(Arc::clone(&tx_handler), Arc::clone(&graph_request_handler), steps, tx_context)
        },
        TxStatus::NoTx => {
            trace!("No TX {:?}", tx_context);
            graph_request_handler.lock().unwrap().handle_graph_request(steps)
        },
    }
}