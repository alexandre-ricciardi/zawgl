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

use std::{collections::HashSet};

use bson::{Bson, Document, doc};
use cypher::query_engine::{process_cypher_query, CypherError};
use zawgl_core::model::{Property, PropertyValue, PropertyGraph};
use zawgl_cypher_query_model::{parameters::build_parameters, model::Request};
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
            let matched_graphs = handle_graph_request(tx_handler, graph_request_handler.clone(), r.steps.to_vec(), None);
            match matched_graphs {
                Ok(mg) => {
                    build_response(request_id, mg, &r)
                },
                Err(e) => build_error(request_id, e),
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
fn build_response(request_id: &str, matched_graphs: Vec<PropertyGraph>, request: &Request) -> Result<Document, CypherError> {
    let mut result_doc = Document::new();
    let mut graph_list = Vec::new();
    let mut return_set = HashSet::new();
    if let Some(ret) = &request.return_clause {
        for ret_exp in &ret.expressions {
            match ret_exp {
                zawgl_cypher_query_model::model::ReturnExpression::FunctionCall(_) => todo!(),
                zawgl_cypher_query_model::model::ReturnExpression::Item(item) => {return_set.insert(item);},
            }
        }
    }
    for pattern in &matched_graphs {
        let mut graph_doc = Document::new();  
        let mut nodes_doc = Vec::new();
        for node in pattern.get_nodes() {
            if let Some(var) = node.get_var() {
                if return_set.contains(&"*".to_string()) || return_set.contains(var) {
                    let mut node_doc = doc!{
                        "name": node.get_var().as_ref().ok_or(CypherError::ResponseError)?.to_string(),
                        "id": node.get_id().ok_or(CypherError::ResponseError)? as i64,
                        "properties": build_properties(node.get_properties_ref()),
                        "labels": Bson::from(node.get_labels_ref()),
                    };
                    node_doc.insert("name", var.to_string());
                    nodes_doc.push(node_doc);
                }
            }
        }
        graph_doc.insert("nodes", nodes_doc);

        let mut rels_doc = Vec::new();
        for rel in pattern.get_relationships_and_edges() {
            if let Some(var) = rel.relationship.get_var() {
                if return_set.contains(&"*".to_string()) || return_set.contains(var) {
                    let mut rel_doc = doc!{
                        "id": rel.relationship.get_id().ok_or(CypherError::ResponseError)? as i64,
                        "source_id": pattern.get_node_ref(&rel.get_source()).get_id().ok_or(CypherError::ResponseError)? as i64,
                        "target_id": pattern.get_node_ref(&rel.get_target()).get_id().ok_or(CypherError::ResponseError)? as i64,
                        "properties": build_properties(rel.relationship.get_properties_ref()),
                        "labels": Bson::from(rel.relationship.get_labels_ref()),
                    };
                    rel_doc.insert("name", var.to_string());
                    rels_doc.push(rel_doc);
                }
            }
        }
        graph_doc.insert("relationships", rels_doc);
        graph_list.push(graph_doc);
    }
    result_doc.insert("graphs", graph_list);

    let mut response_doc = Document::new();
    response_doc.insert("request_id", request_id);
    response_doc.insert("result", result_doc);
    Ok(response_doc)
}

fn build_properties(item_properties: &Vec<Property>) -> Vec<Document> {
    let mut props = Vec::new();
    for p in item_properties {
        
        let name = p.get_name();
        let value = p.get_value();
        let mut bprop = Document::new();
        match value {
            PropertyValue::PBool(v) => bprop.insert(name, v),
            PropertyValue::PFloat(f) => bprop.insert(name, f),
            PropertyValue::PInteger(i) => bprop.insert(name, i),
            PropertyValue::PUInteger(u) => bprop.insert(name, *u as i64),
            PropertyValue::PString(s) => bprop.insert(name, s),
        };
        props.push(bprop);
    }
    props
}