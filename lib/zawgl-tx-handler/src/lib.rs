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

pub mod tx_context;
pub mod tx_handler;
pub mod request_handler;
use request_handler::RequestHandler;
use tx_handler::{Scenario, TxHandler, TxStatus, needs_write_lock};

use zawgl_core::model::PropertyGraph;
use zawgl_cypher_query_model::QueryStep;
use self::tx_context::TxContext;

pub struct ResultGraph {
    pub scenario: Scenario,
    pub patterns: Vec<PropertyGraph>,
}

#[derive(Debug)]
pub enum DatabaseError {
    EngineError,
    TxError,
}

pub fn handle_graph_request<'a>(tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, steps: &Vec<QueryStep>, tx_context: Option<TxContext>) -> Result<Vec<PropertyGraph>, DatabaseError> {
    
    let tx_lock = tx_handler.lock();
    let tx_status = tx_lock.borrow_mut().get_session_status(&tx_context);
    match tx_status {
        TxStatus::OpenNewTx(ctx) => {
            tx_lock.borrow_mut().acquire_session_lock();
            graph_request_handler.write().unwrap().open_graph_tx(ctx);
            graph_request_handler.write().unwrap().handle_graph_request_tx(steps, ctx)
        },
        TxStatus::ContinueCurrentTx(ctx) => graph_request_handler.write().unwrap().handle_graph_request_tx(steps, ctx),
        TxStatus::CommitCurrentTx(ctx) => { 
            let res = graph_request_handler.write().unwrap().commit_tx(ctx);
            tx_lock.borrow_mut().release_session_lock();
            res
        },
        TxStatus::WaitForCurrentTx => {
            tx_lock.borrow_mut().acquire_session_lock();
            handle_graph_request(tx_handler.clone(), graph_request_handler, steps, tx_context)
        },
        TxStatus::NoTx => {
            if needs_write_lock(steps) {
                graph_request_handler.write().unwrap().handle_graph_request(steps)
            } else {
                graph_request_handler.read().unwrap().handle_graph_request(steps)
            }
        },
    }
}