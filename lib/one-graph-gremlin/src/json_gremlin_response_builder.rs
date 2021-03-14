use super::gremlin::*;
use serde_json::Value;

pub fn build_json_gremlin_response(gremlin: &GremlinResponse) -> Option<Value> {
    let mut response = GremlinResponse {
        request_id: gremlin.request_id,
        
    }
    None
}