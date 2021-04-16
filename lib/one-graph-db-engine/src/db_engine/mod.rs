use one_graph_gremlin::gremlin::*;
use one_graph_core::model::*;

use self::gremlin_state::StateContext;

pub mod gremlin_state;

pub struct GraphDatabaseEngine {
    
}

impl GraphDatabaseEngine {
    fn handle_gremlin_request(gremlin: &GremlinRequest) -> Option<GremlinResponse> {
        let mut pattern = PropertyGraph::new();
        let mut gremlin_state = StateContext::new();
        for step in &gremlin.steps {
            match step {
                GStep::V(id) => {
                    match id {
                        Some(val) => {
                            let n = Node::new();
                            
                        },
                        None => {

                        }
                    }
                },
                _ => {}
            }
        }
        None
    }
}