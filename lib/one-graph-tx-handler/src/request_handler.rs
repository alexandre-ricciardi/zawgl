use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::{PropertyGraph, Status};
use one_graph_core::model::init::InitContext;
use one_graph_query_planner::QueryStep;

use crate::tx_context::TxContext;
use crate::tx_handler::Scenario;
use crate::{DatabaseError, ResultGraph};


pub type RequestHandler<'a> = Arc<RwLock<GraphRequestHandler<'a>>>;

pub struct GraphRequestHandler <'a> {
    conf: InitContext<'a>,
    map_session_graph_engine: HashMap<String, GraphEngine>,
}

impl <'a> GraphRequestHandler<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GraphRequestHandler{conf: ctx, map_session_graph_engine: HashMap::new()}
    }

    pub fn handle_graph_request(&self, steps: &Vec<QueryStep>) -> Result<Vec<ResultGraph>, DatabaseError> {
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

    
    pub fn handle_graph_request_tx(&mut self, steps: &Vec<QueryStep>, tx_context: &TxContext) -> Result<Vec<ResultGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or_else(|| DatabaseError::TxError)?;
        let mut matched_graphs = Vec::new();
        for pattern in patterns {
            let result_graphs = match get_request_scenario(&steps) {
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

    pub fn commit_tx(&mut self, tx_context: & TxContext) -> Result<Vec<ResultGraph>, DatabaseError> {
        let graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or_else(|| DatabaseError::TxError)?;
        graph_engine.sync();
        self.map_session_graph_engine.remove(&tx_context.session_id);
        Ok(Vec::new())
    }

    pub fn open_graph_tx(&mut self, tx_context: &TxContext) {
        self.map_session_graph_engine.insert(tx_context.session_id.clone(), GraphEngine::new(&self.conf));
    }
}


pub fn get_request_scenario(pattern: &PropertyGraph) -> Scenario {
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