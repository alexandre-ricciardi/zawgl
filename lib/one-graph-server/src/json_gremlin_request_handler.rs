use one_graph_gremlin::json_gremlin_request_builder::*;
use one_graph_gremlin::gremlin::*;
use serde_json::Value;

pub fn handle_gremlin_json_request(value: &Value) -> Option<Value> {
    let gremlin_request = build_gremlin_request_from_json(value)?;
    None
}