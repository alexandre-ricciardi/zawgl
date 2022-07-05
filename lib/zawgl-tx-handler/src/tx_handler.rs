use std::sync::Arc;
use std::{cell::RefCell, mem};
use std::time::Instant;
use zawgl_core::model::PropertyGraph;
use zawgl_query_planner::QueryStep;
use parking_lot::{Mutex, ReentrantMutex};

use crate::request_handler::get_request_scenario;
use crate::tx_context::TxContext;

pub type TxHandler = Arc<ReentrantMutex<RefCell<GraphTxHandler>>>;

pub enum TxStatus<'a> {
    OpenNewTx(&'a TxContext),
    ContinueCurrentTx(&'a TxContext),
    CommitCurrentTx(&'a TxContext),
    WaitForCurrentTx,
    NoTx,
}

pub struct GraphTxHandler {
    current_session_id: Option<String>,
    session_lock: Mutex<()>,
    is_session_locked: bool,
    tx_start_date: Option<Instant>,
}

impl GraphTxHandler {
    pub fn new() -> Self {
        GraphTxHandler{current_session_id: None, session_lock: Mutex::new(()), is_session_locked: false, tx_start_date: None}
    }

    pub fn get_session_status<'a>(&mut self, tx_context: &'a Option<TxContext>) -> TxStatus<'a> {
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


pub fn needs_write_lock<'a>(steps: &Vec<QueryStep>) -> bool {
    return true;
}
