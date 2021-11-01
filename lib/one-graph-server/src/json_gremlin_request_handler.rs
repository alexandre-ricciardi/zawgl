use one_graph_gremlin::gremlin::ToJson;
use one_graph_gremlin::handler::GremlinError;
use one_graph_gremlin::handler::handle_gremlin_request;
use one_graph_gremlin::json_gremlin_request_builder::*;
use one_graph_tx_handler::GraphTransactionHandler;
use std::sync::RwLock;
use std::sync::Arc;
use serde_json::Value;

pub fn handle_gremlin_json_request<'a>(tx_handler: Arc<RwLock<GraphTransactionHandler<'a>>>, value: &Value) -> Result<Value, GremlinError> {
    let gremlin_request = build_gremlin_request_from_json(value)?;
    let res = handle_gremlin_request(tx_handler, &gremlin_request)?;
    Ok(res.to_json())
}