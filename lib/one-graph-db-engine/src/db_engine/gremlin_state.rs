use one_graph_gremlin::gremlin::*;

pub trait State {
    fn handle_match_vertex(vid: Option<u64>);
    fn handle_add_vertex(label: &str);
    fn handle_add_edge(label: &str);
    fn handle_alias(name: &str);
}

pub struct StateContext {
    
}