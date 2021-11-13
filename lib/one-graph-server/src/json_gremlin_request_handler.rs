use one_graph_gremlin::gremlin::ToJson;
use one_graph_gremlin::handler::GremlinError;
use one_graph_gremlin::handler::handle_gremlin_request;
use one_graph_gremlin::json_gremlin_request_builder::*;
use one_graph_tx_handler::request_handler::RequestHandler;
use one_graph_tx_handler::tx_handler::TxHandler;
use serde_json::Value;

pub fn handle_gremlin_json_request<'a>(tx_handler: TxHandler, graph_request_handler: RequestHandler<'a>, value: &Value) -> Result<Value, GremlinError> {
    let gremlin_request = build_gremlin_request_from_json(value)?;
    let res = handle_gremlin_request(tx_handler, graph_request_handler, &gremlin_request)?;
    Ok(res.to_json())
}