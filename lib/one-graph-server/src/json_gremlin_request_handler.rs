use serde_json::Value;

fn get_json_array(value: &Value) -> Option<&Vec<Value>> {
    match &value {
        Value::Array(array) => {
            Some(array)
        },
        _ => None
    }
}

pub fn handle_gremlin_json_request(value: &Value) -> Option<()> {
    let args = &value["args"];
    let gremlin = &args["@value"];
    let steps = &gremlin[1];
    let step_bytecode = get_json_array(&steps["@value"]["step"])?;
    for expr in step_bytecode {
        for v in get_json_array(&expr)? {
            if v.as_str() == Some("V") {
                
            }
        }
    }
    
    Some(())
}