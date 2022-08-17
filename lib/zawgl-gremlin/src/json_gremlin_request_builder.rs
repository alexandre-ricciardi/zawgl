// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::handler::GremlinError;

use super::gremlin::*;
use serde_json::Map;
use serde_json::Value;


pub fn build_gremlin_request_from_json(value: &Value) -> Result<GremlinRequest, GremlinError> {
    let args = &value["args"];
    let req_id = value["requestId"].as_str().ok_or_else(|| GremlinError::RequestError)?;
    let op = value["op"].as_str().ok_or_else(|| GremlinError::RequestError)?;
    let processor = value["processor"].as_str().ok_or_else(|| GremlinError::RequestError)?;
    
    if op == "bytecode" {
      let gtype = args["@type"].as_str().ok_or_else(|| GremlinError::RequestError)?;
      if gtype == "g:Map" {
        let gmap_values = args["@value"].as_array().ok_or_else(|| GremlinError::RequestError)?;
        let mut req_data = None;
        let mut session = "";
        let mut manage_transaction = None;
        let mut maintain_state_after_exception = None;
        let mut commit_tx = false;
        for index in 0..gmap_values.len()/2 {
          let key = gmap_values[index * 2].as_str().ok_or_else(|| GremlinError::RequestError)?;
          let value = &gmap_values[index * 2 + 1];
          if key == "gremlin" {
            let gremlin_bytecode = build_gremlin_bytecode(value).ok_or_else(|| GremlinError::RequestError)?;
            match gremlin_bytecode {
                GBytecode::Steps(gremlin_steps) => {
                  req_data = Some(GremlinRequestData{steps: gremlin_steps});
                },
                GBytecode::Source(gremlin_source) => {
                  commit_tx = gremlin_source == GSource::TxCommit;
                },
            }
           
          } else if key == "session" {
            session = value.as_str().ok_or_else(|| GremlinError::RequestError)?;
          } else if key == "manageTransaction" {
            manage_transaction = value.as_bool();
          } else if key == "maintainStateAfterException" {
            maintain_state_after_exception = value.as_bool();
          }
        }
        
        if processor == "traversal" {
          return Ok(GremlinRequest{
            request_id: String::from(req_id), 
            data: req_data,
            session: None,
          })
        } else if processor == "session" {
          return Ok(GremlinRequest{
            request_id: String::from(req_id), 
            data: req_data,
            session: Some(GremlinSession {
              session_id: String::from(session),
              manage_transaction: manage_transaction.ok_or_else(|| GremlinError::RequestError)?,
              maintain_state_after_exception: maintain_state_after_exception.ok_or_else(|| GremlinError::RequestError)?,
              commit: commit_tx,
            })
          });
        }
      }
    }
    Err(GremlinError::RequestError)
}

fn build_gremlin_bytecode(bytecode: &Value) -> Option<GBytecode> {
  let bytecode_type = bytecode["@type"].as_str()?;
  if bytecode_type == "g:Bytecode" {
    let bytecode_value = &bytecode["@value"];
    if let Some(source) = bytecode_value.get("source") {
      let gsource = source.as_array()?;
      for source in gsource {
        let gremlin_src = build_gremlin_source(source)?;
        return Some(GBytecode::Source(gremlin_src));
      }
    } else if let Some(steps) = bytecode_value.get("step") {
      let gsteps = steps.as_array()?;
      let mut gremlin_steps = Vec::new();
      for step in gsteps {
          let mut gremlin_step = build_gremlin_step(step)?;
          gremlin_steps.append(&mut gremlin_step);
      }
      return Some(GBytecode::Steps(gremlin_steps));
    }
  }
  None
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
      "to" => {
        vec![to_step(elts)?]
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



fn build_gremlin_source(step: &Value) -> Option<GSource> {
  let elts = step.as_array()?;
  let first = &elts[0];
  let gremlin_source = match first.as_str()? {
      "tx" => {
        tx_source(elts)?
      },
      _ => {
        GSource::Empty
      }
  };
  Some(gremlin_source)
}


fn set_property_step(json_step: &Vec<Value>) -> Option<GStep> {
  let name = json_step.get(1)?.as_str()?;
  let value = &json_step[2];
  if value.is_object() && value["@type"] == "g:Bytecode" {
    let gbytecode = build_gremlin_bytecode(value)?;
    match gbytecode {
        GBytecode::Steps(steps) => Some(GStep::SetDynProperty(String::from(name), steps)),
        GBytecode::Source(_) => None,
    }
  } else {
    Some(GStep::SetProperty(String::from(name), build_gremlin_value(value)?))
  }  
}

fn match_step(json_step: &Vec<Value>) -> Option<GStep> {
  let mut bytecodes = Vec::new();
  for bc in &json_step[1..] {
    let gbytecode = build_gremlin_bytecode(bc)?;
    match gbytecode {
        GBytecode::Steps(steps) => bytecodes.push(steps),
        GBytecode::Source(_) => {},
    }
  } 
  Some(GStep::Match(bytecodes))
}

fn from_step(json_step: &Vec<Value>) -> Option<GStep> {
    let var = json_step.get(1)?;
    Some(GStep::From(build_value_or_vertex(var)?))
}


fn tx_source(json: &Vec<Value>) -> Option<GSource> {
  let var = json.get(1)?;
  let tx_value = var.as_str()?;
  if tx_value == "commit" {
    Some(GSource::TxCommit)
  } else {
    None
  }
}

fn to_step(json_step: &Vec<Value>) -> Option<GStep> {
  let var = json_step.get(1)?;
  Some(GStep::To(build_value_or_vertex(var)?))
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
  let vertex = GVertex{id: id, label: String::from(label), properties: GProperties::new()};
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
  Some(vec![match_out_e(json_step)?, match_in_v()?])
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

fn build_predicate(json_predicate: &Value) -> Option<GPredicate> {
    match json_predicate {
        Value::String(sval) => {
            Some(GPredicate::Value(GValue::String(String::from(sval))))
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

fn build_within_predicate(json: &Map<String, Value>) -> Option<GPredicate> {
    Some(GPredicate::Within(build_gremlin_list(json.get("value")?)?))
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
            GPredicate::Within(l) => {
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
                    GPredicate::Within(list) => {
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