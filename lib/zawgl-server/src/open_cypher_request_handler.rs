use bson::Document;
use zawgl_cypher::CypherError;
use zawgl_tx_handler::{request_handler::RequestHandler, tx_handler::TxHandler};

pub fn handle_open_cypher_request<'a>(tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, cypher_request: &Document) -> Result<Document, CypherError> {
    zawgl_cypher::handle_open_cypher_request(tx_handler, graph_request_handler, cypher_request)
}