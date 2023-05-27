pub mod planner;
pub mod tx_handler;

use bson::{Bson, Document, doc};
use cypher::query_engine::process_cypher_query;
use zawgl_core::model::{Property, PropertyValue};
use zawgl_cypher_query_model::parameters::build_parameters;
use tx_handler::{DatabaseError, handle_graph_request, request_handler::RequestHandler, handler::TxHandler};

extern crate zawgl_core;

extern crate bson;
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

pub mod cypher;

#[derive(Debug)]
pub enum CypherError {
    RequestError,
    ResponseError,
    TxError(DatabaseError)
}

pub fn handle_open_cypher_request(tx_handler: TxHandler, graph_request_handler: RequestHandler<'_>, cypher_request: &Document) -> Result<Document, CypherError> {
    let query = cypher_request.get_str("query").map_err(|_err| CypherError::RequestError)?;
    let request_id = cypher_request.get_str("request_id").map_err(|_err| CypherError::RequestError)?;
    let parameters = cypher_request.get_document("parameters");
    let params = parameters.ok().map(build_parameters);
    let request = process_cypher_query(query, params).ok_or(CypherError::RequestError)?;
    let matched_graphs = handle_graph_request(tx_handler, graph_request_handler.clone(), &request.steps, None).map_err(CypherError::TxError)?;
    let mut result_doc = Document::new();
    let mut graph_list = Vec::new();
    for pattern in &matched_graphs {
        let mut graph_doc = Document::new();  
        let mut nodes_doc = Vec::new();
        for node in pattern.get_nodes() {
            nodes_doc.push(doc!{
                "id": node.get_id().ok_or(CypherError::ResponseError)? as i64,
                "properties": build_properties(node.get_properties_ref()),
                "labels": Bson::from(node.get_labels_ref()),
            });
        }
        graph_doc.insert("nodes", nodes_doc);

        let mut rels_doc = Vec::new();
        for rel in pattern.get_relationships_and_edges() {
            rels_doc.push(doc!{
                "id": rel.relationship.get_id().ok_or(CypherError::ResponseError)? as i64,
                "source_id": pattern.get_node_ref(&rel.get_source()).get_id().ok_or(CypherError::ResponseError)? as i64,
                "target_id": pattern.get_node_ref(&rel.get_target()).get_id().ok_or(CypherError::ResponseError)? as i64,
                "properties": build_properties(rel.relationship.get_properties_ref()),
                "labels": Bson::from(rel.relationship.get_labels_ref()),
            });
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