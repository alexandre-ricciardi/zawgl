use std::collections::HashMap;

use std::sync::{Arc, RwLock};

use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::{PropertyGraph, Status};
use one_graph_core::model::init::InitContext;
use one_graph_query_planner::{QueryStep, handle_query_steps};

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

    pub fn handle_graph_request(&self, steps: &Vec<QueryStep>) -> Result<Vec<PropertyGraph>, DatabaseError> {
        let mut graph_engine = GraphEngine::new(&self.conf);
        let matched_graphs = handle_query_steps(steps, &mut graph_engine);
        graph_engine.sync();
        Ok(matched_graphs)
    }

    
    pub fn handle_graph_request_tx(&mut self, steps: &Vec<QueryStep>, tx_context: &TxContext) -> Result<Vec<PropertyGraph>, DatabaseError> {
        let mut graph_engine = self.map_session_graph_engine.get_mut(&tx_context.session_id).ok_or_else(|| DatabaseError::TxError)?;
        let matched_graphs = handle_query_steps(steps, &mut graph_engine);
        Ok(matched_graphs)
    }

    pub fn commit_tx(&mut self, tx_context: & TxContext) -> Result<Vec<PropertyGraph>, DatabaseError> {
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