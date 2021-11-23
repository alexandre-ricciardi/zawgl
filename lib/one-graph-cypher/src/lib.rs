use bson::{Bson, Document, doc};
use cypher::query_engine::process_cypher_query;
use one_graph_core::graph::traits::GraphContainerTrait;
use one_graph_core::model::{Property, PropertyValue};
use one_graph_tx_handler::{DatabaseError, handle_graph_request, request_handler::RequestHandler, tx_handler::TxHandler};

extern crate one_graph_core;

extern crate bson;
pub mod store;
pub mod cypher;
mod model;

#[derive(Debug)]
pub enum CypherError {
    RequestError,
    ResponseError,
    TxError(DatabaseError)
}

pub fn handle_open_cypher_request<'a>(tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, cypher_request: &Document) -> Result<Document, CypherError> {
    let query = cypher_request.get_str("query").map_err(|err| CypherError::RequestError)?;
    let request = process_cypher_query(query).ok_or(CypherError::RequestError)?;
    let matched_graphs = handle_graph_request(tx_handler.clone(), graph_request_handler.clone(), &vec![request.pattern], None).map_err(|err| CypherError::TxError(err))?;
    let mut response_doc = Document::new();  
    for res in &matched_graphs {
        for pattern in &res.patterns {
            let mut nodes_doc = Vec::new();
            for node in pattern.get_nodes() {
                nodes_doc.push(doc!{
                    "id": node.get_id().ok_or(CypherError::ResponseError)?.to_string(),
                    "properties": build_properties(node.get_properties_ref()),
                    "labels": Bson::from(node.get_labels_ref()),
                });
            }
            response_doc.insert("nodes", nodes_doc);

            let mut rels_doc = Vec::new();
            for rel in pattern.get_relationships_and_edges() {
                rels_doc.push(doc!{
                    "id": rel.0.get_id().ok_or(CypherError::ResponseError)?.to_string(),
                    "source_id": pattern.get_node_ref(&rel.1.get_source()).get_id().ok_or(CypherError::ResponseError)?.to_string(),
                    "target_id": pattern.get_node_ref(&rel.1.get_target()).get_id().ok_or(CypherError::ResponseError)?.to_string(),
                    "properties": build_properties(rel.0.get_properties_ref()),
                    "labels": Bson::from(rel.0.get_labels_ref()),
                });
            }
            response_doc.insert("relationships", rels_doc);
        }
    }
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
            PropertyValue::PString(s) => bprop.insert(name, s),
        };
        props.push(bprop);
    }
    props
}