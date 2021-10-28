use super::gremlin::*;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;

use self::steps::gremlin_state::*;
use self::utils::ResultGraph;
use self::utils::convert_graph_to_gremlin_response;
use self::utils::get_request_scenario;
use self::utils::Scenario;

pub mod steps;
mod utils;

#[derive(Debug)]
pub enum DatabaseError {
    GremlinError(GremlinStateError),
    EngineError,
    ResponseError,
    RequestError,
}

pub struct GremlinDatabaseEngine<'a> {
    conf: InitContext<'a>,
}

fn skip_step(prev_step: &GStep, curr_step: &GStep) -> GStep {
    match curr_step {
        GStep::Has(_, _) => prev_step.clone(),
        _ => curr_step.clone(),
    }
}

fn iterate_gremlin_steps(steps: &Vec<GStep>, mut gremlin_state: GremlinStateMachine) -> Result<GremlinStateMachine, GremlinStateError> {
    let mut previous_step = GStep::Empty;
    for step in steps {
        match step {
            GStep::Match(bytecodes) => {
                for bc in bytecodes {
                    gremlin_state = iterate_gremlin_steps(bc, gremlin_state)?;
                }
            }
            _ => {
                gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, &previous_step, step)?;
            }
        }
        previous_step = skip_step(&previous_step, &step);
    }
    gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, &previous_step, &GStep::Empty)?;
    Ok(gremlin_state)
}

impl <'a> GremlinDatabaseEngine<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GremlinDatabaseEngine{conf: ctx}
    }

    pub fn handle_gremlin_request(&mut self, gremlin: &GremlinRequest) -> Result<GremlinResponse, DatabaseError> {
        let mut gremlin_state = GremlinStateMachine::new();
        gremlin_state = iterate_gremlin_steps(&gremlin.steps, gremlin_state).or_else(|err| Err(DatabaseError::GremlinError(err)))?;
        let ctx = gremlin_state.context;
        let mut graph_engine = GraphEngine::new(&self.conf);
        let mut matched_graphs = Vec::new();
        for pattern in ctx.patterns {
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
        convert_graph_to_gremlin_response(&matched_graphs, &gremlin.request_id)
    }

}
