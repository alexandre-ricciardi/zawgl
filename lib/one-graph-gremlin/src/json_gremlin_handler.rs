use serde_json::Value;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;
use super::gremlin::*;
use serde_json::Map;

pub fn handle_gremlin_json_request(value: &Value) -> Option<()> {
    let args = &value["args"];
    let gremlin = &args["@value"];
    let steps = &gremlin[1];
    let bytecode = steps["@value"]["step"].as_array()?;
    let mut gremlin = Gremlin{steps: Vec::new()};
    for step in bytecode {
        let elts = step.as_array()?;
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
            "addE" => {
                add_e(elts)?
            },
            _ => {
                Step::Empty
            }
        };
        gremlin.steps.push(gremlin_step);
    }
    Some(())
}

fn add_e(json: &Vec<Value>) -> Option<Step> {
    let label = json_step.get(1)?.as_str()?;
    Some(Step::AddE(String::from(label)))
}

fn match_v(json_step: &Vec<Value>) -> Option<Step> {
    json_step.get(1).map(|v| Step::V(v.as_str().map(|sval| GValue::String(String::from(sval))))).or(Some(Step::V(None)))
}

fn add_v(json_step: &Vec<Value>) -> Option<Step> {
    let label = json_step.get(1)?.as_str()?;
    Some(Step::AddV(String::from(label)))
}

fn has_property(json_step: &Vec<Value>) -> Option<Step> {
    let name = json_step.get(1)?.as_str()?;
    Some(Step::Has(String::from(name), build_predicate(json_step.get(2)?)?))
}

fn build_predicate(json_predicate: &Value) -> Option<Predicate> {
    match json_predicate {
        Value::String(sval) => {
            Some(Predicate::Value(GValue::String(String::from(sval))))
        },
        Value::Object(pobj) => {
            let p = pobj.get("@value")?.as_object()?;
            match p.get("predicate")?.as_str()? {
                "within" => {
                    return build_within_predicate(p)
                },
                _ => return None
            }
        },
        _ => {
            None
        }
    }
}

fn build_within_predicate(json: &Map<String, Value>) -> Option<Predicate> {
    Some(Predicate::Within(build_gremlin_list(json.get("value")?)?))
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
                Value::String(sval) => {
                    list.values.push(GValue::String(String::from(sval)));
                }
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
        "g:Int32" => Some(GValue::Integer(val.as_i64()?)),
        _ => None
    }
}

#[cfg(test)]
mod test_gremlin_json {
    use serde_json::Value;
    use super::*;

    #[test]
    fn test_build_list_int() {
        let json = r#"
        {
            "@type": "g:List",
            "@value": [
              {
                "@type": "g:Int32",
                "@value": 1
              },
              {
                "@type": "g:Int32",
                "@value": 2
              },
              {
                "@type": "g:Int32",
                "@value": 3
              }
            ]
        }
        "#;
        let value: Value = serde_json::from_str(json).expect("json g list");
        let glist = build_gremlin_list(&value).expect("glist");
        assert_eq!(GValue::Integer(1), glist.values[0]);
    }

    #[test]
    fn test_build_list_string() {
        let json = r#"
        {
            "@type": "g:List",
            "@value": [
              "lop",
              "ripple"
            ]
        }
        "#;
        let value: Value = serde_json::from_str(json).expect("json g list");
        let glist = build_gremlin_list(&value).expect("glist");
        assert_eq!(GValue::String(String::from("lop")), glist.values[0]);
    }

    #[test]
    fn test_build_predicate() {
        let json = r#"
        {
            "@type": "g:P",
            "@value": {
              "predicate": "within",
              "value": {
                "@type": "g:List",
                "@value": [
                  {
                    "@type": "g:Int32",
                    "@value": 1
                  },
                  {
                    "@type": "g:Int32",
                    "@value": 2
                  },
                  {
                    "@type": "g:Int32",
                    "@value": 3
                  }
                ]
              }
            }
          }
        "#;
        let value: Value = serde_json::from_str(json).expect("json predicate");
        let predicate = build_predicate(&value).expect("predicate");
        match &predicate {
            Predicate::Within(l) => {
                assert_eq!(GValue::Integer(2), l.values[1]);
            },
            _ => {
                assert!(false)
            }
        }
    }
}