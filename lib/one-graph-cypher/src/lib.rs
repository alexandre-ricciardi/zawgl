use bson::Document;
use one_graph_tx_handler::{DatabaseError, request_handler::RequestHandler, tx_handler::TxHandler};

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
    Ok(Document::default())
}