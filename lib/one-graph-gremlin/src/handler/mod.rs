use std::sync::Arc;
use std::sync::RwLock;

use super::gremlin::*;
use one_graph_tx_handler::DatabaseError;
use one_graph_tx_handler::GraphTransactionHandler;
use one_graph_tx_handler::handle_graph_request;

use self::steps::gremlin_state::*;
use self::utils::convert_graph_to_gremlin_response;

pub mod steps;
mod utils;

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

pub fn handle_gremlin_request<'a>(tx_handler: Arc<RwLock<GraphTransactionHandler<'a>>>, gremlin: &GremlinRequest) -> Result<GremlinResponse, GremlinError> {
    let mut gremlin_state = GremlinStateMachine::new();
    gremlin_state = iterate_gremlin_steps(&gremlin.steps, gremlin_state).or_else(|err| Err(GremlinError::StateError(err)))?;
    let ctx = gremlin_state.context;
    let matched_graphs = handle_graph_request(tx_handler, &ctx.patterns).map_err(|err| GremlinError::TxError(err))?;
    convert_graph_to_gremlin_response(&matched_graphs, &gremlin.request_id)
}

#[derive(Debug)]
pub enum GremlinError {
    RequestError,
    ResponseError,
    StateError(GremlinStateError),
    TxError(DatabaseError)
}
  