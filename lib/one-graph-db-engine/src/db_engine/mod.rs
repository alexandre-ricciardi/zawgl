use one_graph_core::model::PropertyGraph;
use one_graph_gremlin::gremlin::*;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;

use self::gremlin::gremlin_state::*;
use self::utils::convert_graph_to_gremlin_response;
use self::utils::get_request_scenario;
use self::utils::Scenario;

mod gremlin;
mod utils;


pub struct GraphDatabaseEngine<'a> {
    conf: InitContext<'a>,
}

fn iterate_gremlin_steps(steps: &Vec<GStep>, mut gremlin_state: GremlinStateMachine) -> Option<GremlinStateMachine> {
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
        previous_step = step.clone();
    }
    gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, &previous_step, &GStep::Empty)?;
    Some(gremlin_state)
}

impl <'a> GraphDatabaseEngine<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GraphDatabaseEngine{conf: ctx}
    }

    pub fn handle_gremlin_request(&mut self, gremlin: &GremlinRequest) -> Option<GremlinResponse> {
        let mut gremlin_state = GremlinStateMachine::new();
        gremlin_state = iterate_gremlin_steps(&gremlin.steps, gremlin_state)?;
        let ctx = gremlin_state.context;
        let mut graph_engine = GraphEngine::new(&self.conf);
        let mut matched_graphs = Vec::new();
        for pattern in ctx.patterns {
            let mut result_graphs = match get_request_scenario(&pattern) {
                Scenario::CreateOnly => {
                    let created = graph_engine.create_graph(&pattern)?;
                    graph_engine.sync();
                    vec![created]
                }
                Scenario::MatchAndCreate => {
                    let matched = graph_engine.match_pattern(&pattern)?;
                    graph_engine.sync();
                    matched
                }
                Scenario::MatchOnly => {
                    let matched = graph_engine.match_pattern(&pattern)?;
                    graph_engine.sync();
                    matched
                }
                Scenario::Unknown => {vec![PropertyGraph::new()]}
            };
            matched_graphs.append(&mut result_graphs);
        }
        return convert_graph_to_gremlin_response(&matched_graphs, &gremlin.request_id);
    }

}