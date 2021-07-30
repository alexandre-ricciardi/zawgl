use super::gremlin::*;
use serde_json::Map;
use serde_json::Value;

pub fn build_gremlin_request_from_json(value: &Value) -> Option<GremlinRequest> {
    let args = &value["args"];
    let req_id = value["requestId"].as_str()?;
    let op = value["op"].as_str()?;
    let processor = value["processor"].as_str()?;
    
    if op == "bytecode" && processor == "traversal" {
      let gtype = args["@type"].as_str()?;
      if gtype == "g:Map" {
        let gmap_value = &args["@value"];
        let gremlin_tag = gmap_value[0].as_str()?;
        if gremlin_tag == "gremlin" {
          let bytecode = &gmap_value[1];
          let gremlin_steps = build_gremlin_bytecode(bytecode)?;
          return Some(GremlinRequest{request_id: String::from(req_id), steps: gremlin_steps})
        }
      }
    }
    None
}

fn build_gremlin_bytecode(bytecode: &Value) -> Option<Vec<GStep>> {
  let mut gremlin_steps = Vec::new();
  let bytecode_type = bytecode["@type"].as_str()?;
  if bytecode_type == "g:Bytecode" {
    let bytecode_value = &bytecode["@value"];
    let steps = bytecode_value["step"].as_array()?;
    for step in steps {
        let gremlin_step = build_gremlin_step(step)?;
        gremlin_steps.push(gremlin_step);
    }
  }
  Some(gremlin_steps)
}

fn build_gremlin_step(step: &Value) -> Option<Vec<GStep>> {
  let elts = step.as_array()?;
  let first = &elts[0];
  let gremlin_step = match first.as_str()? {
      "V" => {
        vec![match_v(elts)?]
      },
      "inV" => {
        vec![match_in_v()?]
      },
      "addV" => {
        vec![add_v(elts)?]
      },
      "has" => {
        vec![has_property(elts)?]
      },
      "addE" => {
        vec![add_e(elts)?]
      },
      "E" => {
        vec![match_e(elts)?]
      },
      "outE" => {
        vec![match_out_e(elts)?]
      },
      "out" => {
          match_out(elts)?
      },
      "as" => {
        vec![as_step(elts)?]
      },
      "from" => {
        vec![from_step(elts)?]
      }
      "match" => {
        vec![match_step(elts)?]
      }
      "property" => {
        vec![set_property_step(elts)?]
      }
      _ => {
        vec![GStep::Empty]
      }
  };
  Some(gremlin_step)
}

fn set_property_step(json_step: &Vec<Value>) -> Option<GStep> {
  let name = json_step.get(1)?.as_str()?;
  let value = &json_step[2];
  if value.is_object() && value["@type"] == "g:Bytecode" {
    Some(GStep::SetDynProperty(String::from(name), build_gremlin_bytecode(value)?))
  } else {
    Some(GStep::SetProperty(String::from(name), build_gremlin_value(value)?))
  }  
}

fn match_step(json_step: &Vec<Value>) -> Option<GStep> {
  let mut bytecodes = Vec::new();
  for bc in &json_step[1..] {
    bytecodes.push(build_gremlin_bytecode(bc)?)
  } 
  Some(GStep::Match(bytecodes))
}

fn from_step(json_step: &Vec<Value>) -> Option<GStep> {
    let var = json_step.get(1)?;
    Some(GStep::From(build_value_or_vertex(var)?))
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
  let vid = json_step.get(1);
  vid.and_then(|v| {
    Some(GStep::V(build_value_or_vertex(v)))
  }).or(Some(GStep::V(None)))
}

fn build_value_or_vertex(elt: &Value) -> Option<GValueOrVertex> {
  match elt {
    Value::String(svalue) => {
      Some(GValueOrVertex::Value(GValue::String(String::from(svalue))))
    }
    Value::Object(obj_map) => {
      if "g:Vertex" == obj_map["@type"] {
        let vertex = build_vertex(obj_map)?;
        Some(GValueOrVertex::Vertex(vertex))
      } else {
        None
      }
    }
    _ => {
      None
    }
  }
}

fn build_vertex(obj: &Map<String, Value>) -> Option<GVertex> {
  let v_value = &obj["@value"];
  let id = build_gremlin_value(&v_value["id"])?;
  let label = v_value["label"].as_str()?;
  let vertex = GVertex{id: id, label: String::from(label)};
  Some(vertex)
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

fn match_out(json_step: &Vec<Value>) -> Option<Vec<GStep>> {
  Some(vec![match_out_e(json_step)?, match_in_v(json_step)?]
}

fn match_in_v() -> Option<GStep> {
  Some(GStep::InV)
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
          list.values.push(build_gremlin_value(elt)?);
        }
        Some(list)
    } else {
        None
    }
}

fn build_gremlin_value(elt: &Value) -> Option<GValue> {
    match elt {
      Value::Object(obj) => {
        let val = obj.get("@value")?;
        match obj.get("@type")?.as_str()? {
          "g:Int32" => Some(GValue::Integer(GInteger::I32(GInt32(val.as_i64()? as i32)))),
          "g:Int64" => Some(GValue::Integer(GInteger::I64(GInt64(val.as_i64()?)))),
          "g:Double" => Some(GValue::Double(GDouble(val.as_f64()?))),
          _ => None
        }
      },
      Value::String(sval) => {
          Some(GValue::String(String::from(sval)))
      }
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

    #[test]
    fn test_parse_match_request() {
      let json = r#"
      {
        "requestId": "e9ec71b5-7c44-4d9e-b1c9-f1268d64e2d4",
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
                    "match",
                    {
                      "@type": "g:Bytecode",
                      "@value": {
                        "step": [
                          [
                            "as",
                            "a"
                          ],
                          [
                            "out",
                            "knows"
                          ],
                          [
                            "as",
                            "b"
                          ]
                        ]
                      }
                    },
                    {
                      "@type": "g:Bytecode",
                      "@value": {
                        "step": [
                          [
                            "as",
                            "a"
                          ],
                          [
                            "out",
                            "created"
                          ],
                          [
                            "as",
                            "c"
                          ]
                        ]
                      }
                    },
                    {
                      "@type": "g:Bytecode",
                      "@value": {
                        "step": [
                          [
                            "as",
                            "b"
                          ],
                          [
                            "out",
                            "created"
                          ],
                          [
                            "as",
                            "c"
                          ]
                        ]
                      }
                    }
                  ],
                  [
                    "addE",
                    "friendlyCollaborator"
                  ],
                  [
                    "from",
                    "a"
                  ],
                  [
                    "to",
                    "b"
                  ],
                  [
                    "property",
                    "id",
                    {
                      "@type": "g:Int32",
                      "@value": 23
                    }
                  ],
                  [
                    "property",
                    "project",
                    {
                      "@type": "g:Bytecode",
                      "@value": {
                        "step": [
                          [
                            "select",
                            "c"
                          ],
                          [
                            "values",
                            "name"
                          ]
                        ]
                      }
                    }
                  ],
                  [
                    "none"
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
      assert_eq!("e9ec71b5-7c44-4d9e-b1c9-f1268d64e2d4", g.request_id);
    }

    #[test]
    fn test_add_vertex() {
      let json = r#"{"requestId":"b3a2c6a8-0982-4414-b07f-41ec49009861","op":"bytecode","processor":"traversal","args":{"@type":"g:Map","@value":["gremlin",{"@type":"g:Bytecode","@value":{"step":[["addV","person"],["property","name","marko"],["none"]]}},"aliases",{"@type":"g:Map","@value":["g","g"]}]}}"#;
      let value: Value = serde_json::from_str(json).expect("json gremlin request");
      let g = build_gremlin_request_from_json(&value).expect("gremlin request");
      assert_eq!("b3a2c6a8-0982-4414-b07f-41ec49009861", g.request_id);
    }
}