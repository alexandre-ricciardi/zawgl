use serde_json::Value;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;
use super::gremlin::*;
use serde_json::Map;

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
        let gremlin_step = match first.as_str()? {
            "V" => {
                match_v(elts)?
            },
            "addV" => {
                add_v(elts)?
            },
            "has" => {
                has_property(elts)?
            },
            _ => {
                Step::Empty
            }
        };
    }
    Some(())
}

fn match_v(json_step: &Vec<Value>) -> Option<Step> {
    let vid = json_step.get(1)?.as_str()?;
    Some(Step::V(MatchVStep{vid: String::from(vid)}))
}

fn add_v(json_step: &Vec<Value>) -> Option<Step> {
    let label = json_step.get(1)?.as_str()?;
    Some(Step::AddV(AddVStep{label: String::from(label)}))
}

fn has_property(json_step: &Vec<Value>) -> Option<Step> {
    let name = json_step.get(1)?.as_str()?;
    Some(Step::Has(HasPropertyStep{
        property_name: String::from(name),
        predicate: build_predicate(json_step.get(2)?)?}))
}

fn build_predicate(json_predicate: &Value) -> Option<Predicate> {
    match json_predicate {
        Value::String(sval) => {
            Some(Predicate::Value(String::from(sval)))
        },
        Value::Object(pobj) => {
            let p = pobj.get("@value")?.as_object()?;
            match p.get("predicate")?.as_str()? {
                "within" => {
                    let pvalue = p.get("value")?;
                    match pvalue {
                        Value::Object(plist) => {
                            None
                        },
                        _ => None
                    }
                },
                _ => None
            }
            None
        },
        _ => {
            None
        }
    }
}

fn build_gremlin_list(json: &Value) -> Option<GList> {
    let obj_type = json.get("@type")?.as_str()?;
    if obj_type == "g:List" {
        let mut list = GList{values: Vec::new()};
        let array = json.get("@value")?.as_array()?;
        for elt in array {
            match elt {
                Value::Object(item) => {
                    list.values.push(build_gremlin_value(item)?);
                },
                _ => {}
            }
        }
        Some(list)
    } else {
        None
    }
}

fn build_gremlin_value(obj: &Map<String, Value>) -> Option<GValue> {
    let val = obj.get("@value")?;
    match obj.get("@type")?.as_str()? {
        "g:Int32" => Some(GValue::Long(val.as_i64()?)),
        _ => None
    }
}