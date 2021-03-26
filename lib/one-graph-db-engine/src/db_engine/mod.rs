use one_graph_gremlin::gremlin::*;
use one_graph_core::model::*;

pub struct GraphDatabaseEngine {
    
}

impl GraphDatabaseEngine {
    fn handle_gremlin_request(gremlin: &GremlinRequest) -> Option<GremlinResponse> {
        let mut pattern = PropertyGraph::new();
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