use one_graph_gremlin::gremlin_engine::DatabaseError;

#[derive(Debug)]
pub enum ServerError {
    HeaderError,
    ParsingError(String),
    GremlinError,
    WebsocketError(tungstenite::Error),
    DatabaseError(DatabaseError)
}