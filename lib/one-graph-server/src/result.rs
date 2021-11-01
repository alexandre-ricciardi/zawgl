use one_graph_gremlin::handler::GremlinError;

#[derive(Debug)]
pub enum ServerError {
    HeaderError,
    ParsingError(String),
    WebsocketError(tungstenite::Error),
    GremlinTxError(GremlinError)
}