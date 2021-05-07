use one_graph_gremlin::gremlin::*;
use one_graph_core::model::init::InitContext;

use self::gremlin_state::*;

pub mod gremlin_state;
mod match_out_edge_state;



pub struct GraphDatabaseEngine<'a> {
    conf: InitContext<'a>,
}

impl <'a> GraphDatabaseEngine<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GraphDatabaseEngine{conf: ctx}
    }

    pub fn handle_gremlin_request(&mut self, gremlin: &GremlinRequest) -> Option<GremlinResponse> {
        let mut gremlin_state = GremlinStateMachine::new();
        for step in &gremlin.steps {
            match step {
                GStep::V(id) => {
                    gremlin_state = GremlinStateMachine::new_match_vertex_state(gremlin_state, id);
                },
                GStep::OutE(labels) => {
                    gremlin_state = GremlinStateMachine::new(gremlin_state, id);
                },
                _ => {}
            }
        }
        None
    }
}