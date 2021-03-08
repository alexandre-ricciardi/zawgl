use serde_json::Value;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;
use super::gremlin::*;

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
    let bytecode = get_json_array(&steps["@value"]["step"])?;
    let mut gremlin = Gremlin{steps: Vec::new()};
    for step in bytecode {
        let elts = get_json_array(&step)?;
        let first = &elts[0];
        match first.as_str()? {
            "V" => {
                let vid = 
                gremlin.steps.push(Steps::MatchVStep())
                //match_v_step(elts.get(1)?);
            },
            "addV" => {
                //add_v_step(elts);
            },
            "property" => {
                //property_step(elts);
            },
            _ => {

            }
        }
    }
    Some(())
}