use serde_json::Value;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;

fn get_json_array(value: &Value) -> Option<&Vec<Value>> {
    match &value {
        Value::Array(array) => {
            Some(array)
        },
        _ => None
    }
}

pub fn handle_gremlin_json_request(value: &Value) -> Option<()> {
    let ctx = InitContext::new(dir);
    let engine = GraphEngine::new(&ctx);
    let args = &value["args"];
    let gremlin = &args["@value"];
    let steps = &gremlin[1];
    let step_bytecode = get_json_array(&steps["@value"]["step"])?;
    for expr in step_bytecode {
        let a_expr = get_json_array(&expr)?;
        let first = &a_expr[0];
        match first.as_str()? {
            "V" => {

            },
            "addV" => {
                
            },
            _ => {

            }
        }
    }
    Some(())
}