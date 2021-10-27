use one_graph_gremlin::gremlin_engine::DatabaseError;
use one_graph_gremlin::json_gremlin_request_builder::*;
use one_graph_gremlin::gremlin::*;
use one_graph_gremlin::gremlin_engine::GremlinDatabaseEngine;
use std::sync::RwLock;
use std::sync::Arc;
use serde_json::Value;

pub fn handle_gremlin_json_request<'a>(graph_engine: Arc<RwLock<GremlinDatabaseEngine<'a>>>, value: &Value) -> Result<Value, DatabaseError> {
    let gremlin_request = build_gremlin_request_from_json(value)?;
    let res = graph_engine.write().unwrap().handle_gremlin_request(&gremlin_request)?;
    Ok(res.to_json())
}