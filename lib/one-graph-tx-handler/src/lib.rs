pub mod tx_context;

use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex, RwLock};

use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::{PropertyGraph, Status};
use one_graph_core::model::init::InitContext;
use self::tx_context::TxContext;

pub type TxHandler = Arc<Mutex<GraphTxHandler>>;
pub type RequestHandler<'a> = Arc<RwLock<GraphRequestHandler<'a>>>;

enum TxStatus<'a> {
    OpenNewTx(&'a TxContext),
    ContinueCurrentTx(&'a TxContext),
    CommitCurrentTx(&'a TxContext),
    WaitForCurrentTx,
    NoTx,
}
pub struct ResultGraph {
    pub scenario: Scenario,
    pub patterns: Vec<PropertyGraph>,
}

#[derive(Debug)]
pub enum DatabaseError {
    EngineError,
    TxError,
}

pub struct GraphTxHandler {
    current_session_id: Option<String>,
    session_lock: Mutex<()>
}

impl GraphTxHandler {
    pub fn new() -> Self {
        GraphTxHandler{current_session_id: None, session_lock: Mutex::new(())}
    }

    fn get_session_status<'a>(&mut self, tx_context: &'a Option<TxContext>) -> TxStatus<'a> {
        if let Some(s_id) = &self.current_session_id {
            if let Some(ctx) = tx_context {
                if &ctx.session_id == s_id {
                    if ctx.commit {
                        TxStatus::CommitCurrentTx(ctx)
                    } else {
                        TxStatus::ContinueCurrentTx(ctx)
                    }
                } else {
                    self.current_session_id = Some(ctx.session_id.clone());
                    TxStatus::OpenNewTx(ctx)
                }
            } else {
                TxStatus::WaitForCurrentTx
            }
        } else {
            TxStatus::NoTx
        }
    }

    fn acquire_session_lock(&mut self) {
        mem::forget(self.session_lock.lock().unwrap());
    }
}

pub struct GraphRequestHandler <'a> {
    conf: InitContext<'a>,
    map_session_graph_engine: HashMap<String, GraphEngine>,
}

impl <'a> GraphRequestHandler<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GraphRequestHandler{conf: ctx, map_session_graph_engine: HashMap::new()}
    }

    fn handle_graph_request(&self, patterns: &Vec<PropertyGraph>) -> Result<Vec<ResultGraph>, DatabaseError> {
        let mut graph_engine = GraphEngine::new(&self.conf);
        let mut matched_graphs = Vec::new();
        for pattern in patterns {
            let result_graphs = match get_request_scenario(&pattern) {
                Scenario::CreateOnly => {
                    let created = graph_engine.create_graph(&pattern).ok_or_else(|| DatabaseError::EngineError)?;
                    graph_engine.sync();
                    ResultGraph{ scenario: Scenario::CreateOnly, patterns: vec![created] }
                }
                Scenario::MatchAndCreate => {
                    let matched = graph_engine.match_pattern_and_create(&pattern).ok_or_else(|| DatabaseError::EngineError)?;
                    graph_engine.sync();
                    ResultGraph{ scenario: Scenario::MatchAndCreate, patterns: matched }
                }
                Scenario::MatchOnly => {
                    let matched = graph_engine.match_pattern(&pattern).ok_or_else(|| DatabaseError::EngineError)?;
                    ResultGraph{ scenario: Scenario::MatchOnly, patterns: matched }
                }
                Scenario::Unknown => {ResultGraph{ scenario: Scenario::Unknown, patterns: vec![] }}
            };
            matched_graphs.push(result_graphs);
        }
        Ok(matched_graphs)
    }

    
    fn handle_graph_request_tx(&mut self, patterns: &Vec<PropertyGraph>, tx_context: &TxContext) -> Result<Vec<ResultGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or_else(|| DatabaseError::TxError)?;
        let mut matched_graphs = Vec::new();
        for pattern in patterns {
            let result_graphs = match get_request_scenario(&pattern) {
                Scenario::CreateOnly => {
                    let created = graph_engine.create_graph(&pattern).ok_or_else(|| DatabaseError::EngineError)?;
                    ResultGraph{ scenario: Scenario::CreateOnly, patterns: vec![created] }
                }
                Scenario::MatchAndCreate => {
                    let matched = graph_engine.match_pattern_and_create(&pattern).ok_or_else(|| DatabaseError::EngineError)?;
                    ResultGraph{ scenario: Scenario::MatchAndCreate, patterns: matched }
                }
                Scenario::MatchOnly => {
                    let matched = graph_engine.match_pattern(&pattern).ok_or_else(|| DatabaseError::EngineError)?;
                    ResultGraph{ scenario: Scenario::MatchOnly, patterns: matched }
                }
                Scenario::Unknown => {ResultGraph{ scenario: Scenario::Unknown, patterns: vec![] }}
            };
            matched_graphs.push(result_graphs);
        }
        Ok(matched_graphs)
    }

    fn commit_tx(&mut self, tx_context: & TxContext) -> Result<Vec<ResultGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or_else(|| DatabaseError::TxError)?;
        graph_engine.sync();
        Ok(Vec::new())
    }

    fn open_graph_tx(&mut self, tx_context: &TxContext) {
        self.map_session_graph_engine.insert(tx_context.session_id.clone(), GraphEngine::new(&self.conf));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    CreateOnly,
    MatchAndCreate,
    MatchOnly,
    Unknown,
}

fn get_request_scenario(pattern: &PropertyGraph) -> Scenario {
    let mut contains_match = false;
    let mut contains_create = false;
    for n in pattern.get_nodes() {
        match n.get_status() {
            Status::Create => {contains_create = true;}
            Status::Match => {contains_match = true;}
            _ => {}
        } 
    }
    for r in pattern.get_relationships() {
        match r.get_status() {
            Status::Create => {contains_create = true;}
            Status::Match => {contains_match = true;}
            _ => {}
        } 
    }
    if contains_match {
        if contains_create {
            Scenario::MatchAndCreate
        } else {
            Scenario::MatchOnly
        }
    } else {
        if contains_create {
            Scenario::CreateOnly
        } else {
            Scenario::Unknown
        }
    }
}

fn needs_write_lock<'a>(patterns: &Vec<PropertyGraph>) -> bool {
    for pattern in patterns {
        if get_request_scenario(pattern) != Scenario::MatchOnly {
            return true;
        } 
    }
    return false;
}

pub fn handle_graph_request<'a>(tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, patterns: &Vec<PropertyGraph>, tx_context: Option<TxContext>) -> Result<Vec<ResultGraph>, DatabaseError> {
    
    let mut tx_lock = tx_handler.lock().unwrap();
    let tx_status = tx_lock.get_session_status(&tx_context);
    match tx_status {
        TxStatus::OpenNewTx(ctx) => {
            graph_request_handler.write().unwrap().open_graph_tx(ctx);
            graph_request_handler.write().unwrap().handle_graph_request_tx(patterns, ctx)
        },
        TxStatus::ContinueCurrentTx(ctx) => graph_request_handler.write().unwrap().handle_graph_request_tx(patterns, ctx),
        TxStatus::CommitCurrentTx(ctx) => graph_request_handler.write().unwrap().commit_tx(ctx),
        TxStatus::WaitForCurrentTx => {
            tx_lock.acquire_session_lock();
            handle_graph_request(tx_handler.clone(), graph_request_handler, patterns, tx_context)
        },
        TxStatus::NoTx => {
            if needs_write_lock(patterns) {
                graph_request_handler.write().unwrap().handle_graph_request(patterns)
            } else {
                graph_request_handler.read().unwrap().handle_graph_request(patterns)
            }
        },
    }
}