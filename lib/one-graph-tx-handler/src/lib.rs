pub mod tx_context;

use std::sync::{Arc, RwLock};

use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::{PropertyGraph, Status};
use one_graph_core::model::init::InitContext;
use self::tx_context::TxContext;

pub struct ResultGraph {
    pub scenario: Scenario,
    pub patterns: Vec<PropertyGraph>,
}

#[derive(Debug)]
pub enum DatabaseError {
    EngineError,
}

pub struct GraphTransactionHandler <'a> {
    conf: InitContext<'a>,
}

impl <'a> GraphTransactionHandler<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GraphTransactionHandler{conf: ctx}
    }

    pub fn handle_graph_request(&self, patterns: &Vec<PropertyGraph>) -> Result<Vec<ResultGraph>, DatabaseError> {
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

fn needs_write_lock(patterns: &Vec<PropertyGraph>) -> bool {
    for pattern in patterns {
        if get_request_scenario(pattern) != Scenario::MatchOnly {
            return true;
        } 
    }
    return false;
}

pub fn handle_graph_request<'a>(tx_handler: Arc<RwLock<GraphTransactionHandler<'a>>>, patterns: &Vec<PropertyGraph>) -> Result<Vec<ResultGraph>, DatabaseError> {
    if needs_write_lock(patterns) {
        tx_handler.write().unwrap().handle_graph_request(patterns)
    } else {
        tx_handler.read().unwrap().handle_graph_request(patterns)
    }
}