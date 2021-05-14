use one_graph_gremlin::gremlin::*;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;

use self::gremlin_state::*;


pub mod gremlin_state;
mod match_out_edge_state;
mod match_vertex_state;



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
            gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, step)?;
        }
        gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, &GStep::Empty)?;
        let ctx = gremlin_state.get_context();
        let graph_engine = GraphEngine::new(&self.conf);
        
        None
    }
}