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

use std::sync::Arc;
use std::{cell::RefCell, mem};
use std::time::Instant;
use parking_lot::{Mutex, ReentrantMutex};
use zawgl_cypher_query_model::QueryStep;
use super::tx_context::TxContext;

pub type TxHandler = Arc<ReentrantMutex<RefCell<GraphTxHandler>>>;

pub enum TxStatus {
    OpenNewTx(TxContext),
    ContinueCurrentTx(TxContext),
    CommitCurrentTx(TxContext),
    WaitForCurrentTx,
    NoTx,
}

pub struct GraphTxHandler {
    current_session_id: Option<String>,
    session_lock: Mutex<()>,
    is_session_locked: bool,
    tx_start_date: Option<Instant>,
}

impl Default for GraphTxHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphTxHandler {
    pub fn new() -> Self {
        GraphTxHandler{current_session_id: None, session_lock: Mutex::new(()), is_session_locked: false, tx_start_date: None}
    }

    pub fn get_session_status(&mut self, tx_context: Option<TxContext>) -> TxStatus {
        if let Some(ctx) = tx_context {
            if let Some(s_id) = &self.current_session_id {
                if &ctx.session_id == s_id {
                    if ctx.commit {
                        TxStatus::CommitCurrentTx(ctx)
                    } else {
                        TxStatus::ContinueCurrentTx(ctx)
                    }
                } else {
                    TxStatus::WaitForCurrentTx
                }
            } else {
                self.current_session_id = Some(ctx.session_id.clone());
                TxStatus::OpenNewTx(ctx)
            }
        } else if let Some(_sid) = &self.current_session_id {
            TxStatus::WaitForCurrentTx
        } else {
            TxStatus::NoTx
        }
    }

    pub fn acquire_session_lock(&mut self) {
        if let Some(tx_start_date) = self.tx_start_date {
            let duration = Instant::now().duration_since(tx_start_date);
            if duration.as_secs() > 10 {
                self.release_session_lock();
            }
        }
        mem::forget(self.session_lock.lock());
        self.is_session_locked = true;
        self.tx_start_date = Some(Instant::now());
    }

    pub fn release_session_lock(&mut self) {
        self.current_session_id = None;
        if self.is_session_locked {
            unsafe {
                self.session_lock.force_unlock();
            }
            self.is_session_locked = false;
            self.tx_start_date = None;
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    CreateOnly,
    MatchAndCreate,
    MatchOnly,
    Unknown,
}


pub fn needs_write_lock(_steps: &[QueryStep]) -> bool {
    true
}
