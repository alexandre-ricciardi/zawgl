use super::gremlin::*;
use serde_json::Value;

pub fn build_json_gremlin_response(gremlin: &GremlinResponse) -> Option<Value> {
    Some(gremlin.to_json())
}