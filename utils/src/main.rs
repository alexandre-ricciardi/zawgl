use one_graph_core::graph::traits::GrowableGraphTrait;
use one_graph_core::test_utils::*;
use one_graph_core::model::init::InitContext;
use one_graph_core::graph_engine::GraphEngine;

fn main() {
    let main_dir = get_tmp_dir_path("test_main");
    let conf = InitContext::new(&main_dir);
    let mut graph_engine = GraphEngine::new(&conf);
    let full_graph = graph_engine.retrieve_graph().unwrap();
    print!("{:?}", full_graph.get_nodes_ids());
}