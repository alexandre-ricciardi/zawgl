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

use bson::{Bson, Document, doc};
use cypher::query_engine::{process_cypher_query, CypherError};
use zawgl_core::{model::{Property, PropertyValue, PropertyGraph, Relationship, Node}, graph::{EdgeData, NodeIndex, EdgeIndex}};
use zawgl_cypher_query_model::{model::{EvalResultItem, EvalScopeClause, EvalScopeExpression, NodeResult, RelationshipResult, Request, ValueItem}, parameters::build_parameters, QueryResult, StepType};
use tx_handler::{handle_graph_request, request_handler::RequestHandler, handler::TxHandler, DatabaseError};

extern crate zawgl_core;

extern crate bson;

pub mod cypher;

pub fn handle_open_cypher_request(tx_handler: TxHandler, graph_request_handler: RequestHandler, cypher_request: &Document) -> Result<Document, CypherError> {
    let query = cypher_request.get_str("query").map_err(|_err| CypherError::RequestError)?;
    let request_id = cypher_request.get_str("request_id").map_err(|_err| CypherError::RequestError)?;
    let parameters = cypher_request.get_document("parameters");
    let params = parameters.ok().map(build_parameters);
    let request = process_cypher_query(query, params);
    match request {
        Ok(r) => {
            let oqr = handle_graph_request(tx_handler, graph_request_handler.clone(), r.steps.to_vec(), None);
            match oqr {
                Ok(qr) => {
                    build_response(request_id, qr, &r)
                },
                Err(e) => {
                    graph_request_handler.lock().unwrap().cancel();
                    build_error(request_id, e)
                },
            }
        },
        Err(ce) => build_cypher_error(request_id, ce),
    }
}

fn build_error(request_id: &str, err: DatabaseError) -> Result<Document, CypherError> {
    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("error", format!("database error {}", err));
    Ok(response_doc)
}

fn build_cypher_error(request_id: &str, err: CypherError) -> Result<Document, CypherError> {
    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("error", format!("database error {}", err));
    Ok(response_doc)
}

fn get_return_clause(request: &Request) -> Option<&EvalScopeClause> {
    request.steps.last().and_then(|ret| {
        match &ret.step_type {
            StepType::RETURN(ret_clause) => Some(ret_clause),
            _ => None
        }
    })
}

fn eval_item_to_bson(eval_item: &EvalResultItem) -> Result<Document, CypherError> {
    match eval_item {
        EvalResultItem::Node(n) => make_node_doc(&n),
        EvalResultItem::Relationship(rel) => make_relationship_doc(&rel),
        EvalResultItem::Scalar(value) => Ok(doc! {
            value.name.to_string(): value.value
        }),
        EvalResultItem::Bool(value) =>  Ok(doc! {
            value.name.to_string(): value.value
        }),
        EvalResultItem::String(value) =>  Ok(doc! {
            value.name.to_string(): value.value.to_string()
        })
    }
}

fn build_response(request_id: &str, qr: QueryResult, request: &Request) -> Result<Document, CypherError> {
    let mut result_doc = Document::new();
    let mut graph_list = Vec::new();
    let return_wildcard = &request.steps.last().map(|ret| {
        match &ret.step_type {
            StepType::RETURN(ret_clause) => ret_clause.has_wildcard(),
            _ => false
        }
    });
    let wildcard = return_wildcard == &Some(true);
    for pattern in &qr.matched_graphs {
        let mut graph_doc = Document::new();  
        let mut nodes_doc = Vec::new();
        let mut rels_doc = Vec::new();
        if let Some(ret_clause) = get_return_clause(request) {
            for ret_exp in &ret_clause.expressions {
                match ret_exp {
                    EvalScopeExpression::Item(item) => {
                        match &item.item {
                            ValueItem::NamedItem(named_item) => {
                                nodes_doc.extend(get_nodes_named(wildcard, item.alias.as_ref(), named_item, pattern)?);
                                rels_doc.extend(get_relationships_named(wildcard, item.alias.as_ref(), named_item, pattern)?);
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
        
        if !nodes_doc.is_empty() {
            graph_doc.insert("nodes", nodes_doc);
        }
        if !rels_doc.is_empty() {
            graph_doc.insert("relationships", rels_doc);
        }
        if !graph_doc.is_empty() {
            graph_list.push(graph_doc);
        }
    }

    let mut values_doc = vec![];
    for res in &qr.return_eval {
        let mut row = vec![];
        for eval_item in res {
            row.push(eval_item_to_bson(eval_item)?);
        }
        values_doc.push(row);
    }
    

    if !values_doc.is_empty() {
        result_doc.insert("values", values_doc);
    }
    
    if !graph_list.is_empty() {
        result_doc.insert("graphs", graph_list);
    }

    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("result", result_doc);
    Ok(response_doc)
}

fn make_node_doc(node: &NodeResult) -> Result<Document, CypherError> {
    let node_doc = doc!{
        "name": node.name.to_string(),
        "id": node.value.get_id().ok_or(CypherError::ResponseError)? as i64,
        "properties": build_properties(node.value.get_properties_ref()),
        "labels": Bson::from(node.value.get_labels_ref()),
    };
    Ok(node_doc)
}

fn make_relationship_doc(rel: &RelationshipResult) -> Result<Document, CypherError> {
    let rel_doc = doc!{
        "name": rel.name.to_string(),
        "id": rel.value.relationship.get_id().ok_or(CypherError::ResponseError)? as i64,
        "source_id": rel.source_nid,
        "target_id": rel.target_nid,
        "properties": build_properties(rel.value.relationship.get_properties_ref()),
        "labels": Bson::from(rel.value.relationship.get_labels_ref()),
    };
    Ok(rel_doc)
}

fn get_nodes_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph) -> Result<Vec<Document>, CypherError> {
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
fn get_relationships_named(ret_all: bool, alias: Option<&String>, name: &str, graph: &PropertyGraph) -> Result<Vec<Document>, CypherError> {
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

fn make_node(alias: Option<&String>, name: &str, node: &Node) -> Result<Document, CypherError> {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let node_doc = doc!{
        "name": ret_name,
        "id": node.get_id().ok_or(CypherError::ResponseError)? as i64,
        "properties": build_properties(node.get_properties_ref()),
        "labels": Bson::from(node.get_labels_ref()),
    };
    Ok(node_doc)
}

fn make_relationship(alias: Option<&String>, name: &str, rel: &EdgeData<NodeIndex, EdgeIndex, Relationship>, graph: &PropertyGraph) -> Result<Document, CypherError> {
    let ret_name = if let Some(a) = alias {
        a.to_string()
    } else {
        name.to_string()
    };
    let rel_doc = doc!{
        "name": ret_name,
        "id": rel.relationship.get_id().ok_or(CypherError::ResponseError)? as i64,
        "source_id": graph.get_node_ref(&rel.get_source()).get_id().ok_or(CypherError::ResponseError)? as i64,
        "target_id": graph.get_node_ref(&rel.get_target()).get_id().ok_or(CypherError::ResponseError)? as i64,
        "properties": build_properties(rel.relationship.get_properties_ref()),
        "labels": Bson::from(rel.relationship.get_labels_ref()),
    };
    Ok(rel_doc)
}
fn build_property_value(name: &str, value: &PropertyValue) -> Document {
    match value {
        PropertyValue::PBool(v) => doc! {
            name: v
        },
        PropertyValue::PFloat(f) => doc! {
            name: f
        },
        PropertyValue::PInteger(i) => doc! {
            name: i
        },
        PropertyValue::PUInteger(u) => doc! {
            name: *u as i64
        },
        PropertyValue::PString(s) => doc! {
            name: s
        }
    }
}

fn build_properties(item_properties: &Vec<Property>) -> Vec<Document> {
    let mut props = Vec::new();
    for p in item_properties {
        let name = p.get_name();
        let value = p.get_value();
        props.push(build_property_value(name, value));
    }
    props
}