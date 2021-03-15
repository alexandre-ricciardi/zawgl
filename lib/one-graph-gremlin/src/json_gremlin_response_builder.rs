use super::gremlin::*;
use serde_json::Value;
use serde_json::json;

pub fn build_json_gremlin_response(gremlin: &GremlinResponse) -> Option<Value> {
    Some(json!({
        "request_id": gremlin.request_id,
        "status": {
            "messages": "",
            "code": 200
        }
    }))
}