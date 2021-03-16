use super::gremlin::*;
use serde_json::Value;
use serde_json::json;

pub fn build_json_gremlin_response(gremlin: &GremlinResponse) -> Option<Value> {
    let mut json_res = json!({
        "request_id": gremlin.request_id,
        "status": {
            "messages": "",
            "code": 200
        },
        "result": {
            "data": null,
            "meta": null
        }
    });

    
    //json_res["result"]["data"] = 
    Some(json_res)
}