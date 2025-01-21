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

pub mod planner;
pub mod tx_handler;

use cypher::query_engine::{process_cypher_query, CypherError};
use serde_json::{json, Map, Value};
use zawgl_core::{model::{Property, PropertyValue, PropertyGraph, Relationship, Node}, graph::{EdgeData, NodeIndex, EdgeIndex}};
use zawgl_cypher_query_model::{model::{EvalResultItem, EvalScopeClause, EvalScopeExpression, NodeResult, RelationshipResult, Request, ValueItem}, QueryResult, StepType};
use tx_handler::{handle_graph_request, request_handler::RequestHandler, handler::TxHandler, DatabaseError};

extern crate zawgl_core;
pub mod cypher;

pub fn handle_open_cypher_request(tx_handler: TxHandler, graph_request_handler: RequestHandler, cypher_request: &Value) -> Result<Value, CypherError> {
    let query = cypher_request["query"].as_str().ok_or(CypherError::RequestError)?;
    let request_id = cypher_request["request_id"].as_str().ok_or(CypherError::RequestError)?;
    let parameters = cypher_request.get("parameters").map(|v| v.clone());
    let request = process_cypher_query(&query, parameters);
    match request {
        Ok(r) => {
            let oqr = handle_graph_request(tx_handler, graph_request_handler.clone(), r.steps.to_vec(), None);
            match oqr {
                Ok(qr) => {
                    build_response(&request_id, qr, &r)
                },
                Err(e) => {
                    graph_request_handler.lock().unwrap().cancel();
                    build_error(&request_id, e)
                },
            }
        },
        Err(ce) => build_cypher_error(&request_id, ce),
    }
}

fn build_error(request_id: &str, err: DatabaseError) -> Result<Value, CypherError> {
    Ok(json!({
        "request_id": request_id, 
        "error": format!("database error {}", err)
    }))
}

fn build_cypher_error(request_id: &str, err: CypherError) -> Result<Value, CypherError> {
    Ok(json!({
        "request_id": request_id, 
        "error": format!("database error {}", err)
    }))
}

fn get_return_clause(request: &Request) -> Option<&EvalScopeClause> {
    request.steps.last().and_then(|ret| {
        match &ret.step_type {
            StepType::RETURN(ret_clause) => Some(ret_clause),
            _ => None
        }
    })
}

fn eval_item_to_json(eval_item: &EvalResultItem) -> Result<Value, CypherError> {
    match eval_item {
        EvalResultItem::Node(n) => make_node_doc(&n),
        EvalResultItem::Relationship(rel) => make_relationship_doc(&rel),
        EvalResultItem::Scalar(value) => Ok(json!({
            value.name.to_string(): value.value
        })),
        EvalResultItem::Bool(value) =>  Ok(json!({
            value.name.to_string(): value.value
        })),
        EvalResultItem::String(value) =>  Ok(json!({
            value.name.to_string(): value.value.to_string()
        })),
        EvalResultItem::List(list) => {
            let mut res = vec![];
            for item in &list.values {
                res.push(eval_item_to_json(item)?)
            }
            Ok(json!({list.name.to_string(): res}))
        }
    }
}

fn build_response(request_id: &str, qr: QueryResult, request: &Request) -> Result<Value, CypherError> {
    let mut result_doc = Map::new();
    let mut graph_list = Vec::new();
    let return_wildcard = &request.steps.last().map(|ret| {
        match &ret.step_type {
            StepType::RETURN(ret_clause) => ret_clause.has_wildcard(),
            _ => false
        }
    });
    let wildcard = return_wildcard == &Some(true);
    for graph in &qr.matched_graphs {
        let graph_doc = build_graph_doc(&request, graph, wildcard)?;
        graph_list.push(graph_doc);
    }

    let mut values_doc = vec![];
    for res in &qr.return_eval {
        let mut row = vec![];
        for eval_item in res {
            row.push(eval_item_to_json(eval_item)?);
        }
        values_doc.push(Value::Array(row));
    }
    

    if !values_doc.is_empty() {
        result_doc.insert("values".to_string(), Value::Array(values_doc));
    }
    
    if !graph_list.is_empty() {
        result_doc.insert("graphs".to_string(), Value::Array(graph_list));
    }

    result_doc.insert("merged_graphs".to_string(), build_graph_doc(&request, &qr.merged_graphs, wildcard)?);

    Ok(json!({
        "request_id": request_id,
        "result": result_doc
    }))
}

fn build_graph_doc(request: &Request, graph: &PropertyGraph, wildcard: bool) -> Result<Value, CypherError> {
    let mut nodes_doc = Vec::new();
    let mut rels_doc = Vec::new();
    if let Some(ret_clause) = get_return_clause(request) {
        for ret_exp in &ret_clause.expressions {
            match ret_exp {
                EvalScopeExpression::Item(item) => {
                    match &item.item {
                        ValueItem::NamedItem(named_item) => {
                            nodes_doc.extend(get_nodes_named(wildcard, item.alias.as_ref(), named_item, graph)?);
                            rels_doc.extend(get_relationships_named(wildcard, item.alias.as_ref(), named_item, graph)?);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
    
    Ok(json!({
        "nodes": nodes_doc,
        "relationships": rels_doc
    }))
}


fn make_node_doc(node: &NodeResult) -> Result<Value, CypherError> {
    let node_doc = json!({
        "name": node.name.to_string(),
        "id": node.value.get_id().ok_or(CypherError::ResponseError)?,
        "properties": build_properties(node.value.get_properties_ref()),
        "labels": node.value.get_labels_ref(),
    });
    Ok(node_doc)
}

fn make_relationship_doc(rel: &RelationshipResult) -> Result<Value, CypherError> {
    let rel_doc = json!({
        "name": rel.name.to_string(),
        "id": rel.value.relationship.get_id().ok_or(CypherError::ResponseError)?,
        "source_id": rel.source_nid,
        "target_id": rel.target_nid,
        "properties": build_properties(rel.value.relationship.get_properties_ref()),
        "labels": rel.value.relationship.get_labels_ref(),
    });
    Ok(rel_doc)
}

fn get_nodes_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph) -> Result<Vec<Value>, CypherError> {
    let mut nodes_doc = vec![];
    for node in graph.get_nodes() {
        if let Some(var) = node.get_var() {
            if var == name || ret_all {
                nodes_doc.push(make_node(alias, name, node)?);
            }
        }
    }
    Ok(nodes_doc)
}
fn get_relationships_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph) -> Result<Vec<Value>, CypherError> {
    let mut rels_doc = vec![];
    for rel in graph.get_relationships_and_edges() {
        if let Some(var) = rel.relationship.get_var() {
            if var == name || ret_all {
                rels_doc.push(make_relationship(alias, name, rel, graph)?);
            }
        }
    }
    Ok(rels_doc)
}

fn make_node(alias: Option<&String>, name: &str, node: &Node) -> Result<Value, CypherError> {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let node_doc = json!({
        "name": ret_name,
        "id": node.get_id().ok_or(CypherError::ResponseError)? as i64,
        "properties": build_properties(node.get_properties_ref()),
        "labels": node.get_labels_ref(),
    });
    Ok(node_doc)
}

fn make_relationship(alias: Option<&String>, name: &str, rel: &EdgeData<NodeIndex, EdgeIndex, Relationship>, graph: &PropertyGraph) -> Result<Value, CypherError> {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let rel_doc = json!({
        "name": ret_name,
        "id": rel.relationship.get_id().ok_or(CypherError::ResponseError)? as i64,
        "source_id": graph.get_node_ref(&rel.get_source()).get_id().ok_or(CypherError::ResponseError)? as i64,
        "target_id": graph.get_node_ref(&rel.get_target()).get_id().ok_or(CypherError::ResponseError)? as i64,
        "properties": build_properties(rel.relationship.get_properties_ref()),
        "labels": rel.relationship.get_labels_ref(),
    });
    Ok(rel_doc)
}
fn build_property_value(name: &str, value: &PropertyValue) -> Value {
    match value {
        PropertyValue::PBool(v) => json!({
            name: v
        }),
        PropertyValue::PFloat(f) => json!({
            name: f
        }),
        PropertyValue::PInteger(i) => json!({
            name: i
        }),
        PropertyValue::PUInteger(u) => json!({
            name: *u as i64
        }),
        PropertyValue::PString(s) => json!({
            name: s
        }),
    }
}

fn build_properties(item_properties: &Vec<Property>) -> Vec<Value> {
    let mut props = Vec::new();
    for p in item_properties {
        let name = p.get_name();
        let value = p.get_value();
        props.push(build_property_value(name, value));
    }
    props
}