use super::gremlin::*;
use serde_json::Map;
use serde_json::Value;

pub fn build_gremlin_request_from_json(value: &Value) -> Option<GremlinRequest> {
    let args = &value["args"];
    let req_id = value.get("requestId")?.as_str()?;
    let gremlin = &args["@value"];
    let steps = &gremlin[1];
    let bytecode = steps["@value"]["step"].as_array()?;
    let mut gremlin = GremlinRequest{request_id: String::from(req_id), steps: Vec::new()};
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
            "E" => {
                match_e(elts)?
            },
            "outE" => {
                match_out_e(elts)?
            },
            "as" => {
                as_step(elts)?
            },
            "from" => {
                from_step(elts)?
            }
            _ => {
                GStep::Empty
            }
        };
        gremlin.steps.push(gremlin_step);
    }
    Some(gremlin)
}

fn from_step(json_step: &Vec<Value>) -> Option<GStep> {
    let var = json_step.get(1)?.as_str()?;
    Some(GStep::From(String::from(var)))
}

fn as_step(json_step: &Vec<Value>) -> Option<GStep> {
    let var = json_step.get(1)?.as_str()?;
    Some(GStep::As(String::from(var)))
}

fn add_e(json_step: &Vec<Value>) -> Option<GStep> {
    let label = json_step.get(1)?.as_str()?;
    Some(GStep::AddE(String::from(label)))
}

fn match_v(json_step: &Vec<Value>) -> Option<GStep> {
    json_step.get(1).map(|v| GStep::V(v.as_str().map(|sval| GValue::String(String::from(sval))))).or(Some(GStep::V(None)))
}

fn match_e(json_step: &Vec<Value>) -> Option<GStep> {
  json_step.get(1).map(|v| GStep::E(v.as_str().map(|sval| GValue::String(String::from(sval))))).or(Some(GStep::V(None)))
}


fn match_out_e(json_step: &Vec<Value>) -> Option<GStep> {
  let mut labels = Vec::new();
  for value in &json_step[1..] {
    labels.push(String::from(value.as_str()?));
  }
  Some(GStep::OutE(labels))
}


fn add_v(json_step: &Vec<Value>) -> Option<GStep> {
    let label = json_step.get(1)?.as_str()?;
    Some(GStep::AddV(String::from(label)))
}

fn has_property(json_step: &Vec<Value>) -> Option<GStep> {
    let name = json_step.get(1)?.as_str()?;
    Some(GStep::Has(String::from(name), build_predicate(json_step.get(2)?)?))
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

fn build_gremlin_list(json: &Value) -> Option<GList<GValue>> {
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
      "g:Int32" => Some(GValue::Integer(GInteger::I32(GInt32(val.as_i64()? as i32)))),
      "g:Int64" => Some(GValue::Integer(GInteger::I64(GInt64(val.as_i64()?)))),
      _ => None
    }
}

fn build_gremlin_integer(obj: &Map<String, Value>) -> Option<GInteger> {
  let val = obj.get("@value")?;
  match obj.get("@type")?.as_str()? {
    "g:Int32" => Some(GInteger::I32(GInt32(val.as_i64()? as i32))),
    "g:Int64" => Some(GInteger::I64(GInt64(val.as_i64()?))),
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
        assert_eq!(GValue::Integer(GInteger::I32(GInt32(1))), glist.values[0]);
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
                assert_eq!(GValue::Integer(GInteger::I32(GInt32(2))), l.values[1]);
            },
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_build_has_str_predicate() {
        let json = r#"
            [
                "has",
                "name",
                {
                    "@type": "g:P",
                    "@value": {
                        "predicate": "within",
                        "value": {
                            "@type": "g:List",
                            "@value": [
                            "lop",
                            "ripple"
                            ]
                        }
                    }
                }
            ]"#;
        let value: Value = serde_json::from_str(json).expect("json has predicate");
        let has = has_property(value.as_array().expect("step list")).expect("has prop");
        match &has {
            GStep::Has(prop_name, predicate) => {
                assert_eq!("name", prop_name);
                match predicate {
                    Predicate::Within(list) => {
                        assert_eq!(GValue::String(String::from("ripple")), list.values[1]);
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parse_gremlin_request() {
        let json = r#"
        {
            "requestId": "9bacba37-9dea-4be3-8fa4-9db886a7de0e",
            "op": "bytecode",
            "processor": "traversal",
            "args": {
              "@type": "g:Map",
              "@value": [
                "gremlin",
                {
                  "@type": "g:Bytecode",
                  "@value": {
                    "step": [
                      [
                        "V"
                      ],
                      [
                        "has",
                        "name",
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
                      ],
                      [
                        "as",
                        "person"
                      ],
                      [
                        "V"
                      ],
                      [
                        "has",
                        "name",
                        {
                          "@type": "g:P",
                          "@value": {
                            "predicate": "within",
                            "value": {
                              "@type": "g:List",
                              "@value": [
                                "lop",
                                "ripple"
                              ]
                            }
                          }
                        }
                      ],
                      [
                        "addE",
                        "uses"
                      ],
                      [
                        "from",
                        "person"
                      ]
                    ]
                  }
                },
                "aliases",
                {
                  "@type": "g:Map",
                  "@value": [
                    "g",
                    "g"
                  ]
                }
              ]
            }
        }
        "#;
        let value: Value = serde_json::from_str(json).expect("json gremlin request");
        let g = build_gremlin_request_from_json(&value).expect("gremlin request");
        assert_eq!("9bacba37-9dea-4be3-8fa4-9db886a7de0e", g.request_id);
    }
}