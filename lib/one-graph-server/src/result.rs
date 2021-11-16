use one_graph_cypher::CypherError;
use one_graph_gremlin::handler::GremlinError;
use tokio_tungstenite::tungstenite::Error;

#[derive(Debug)]
pub enum ServerError {
    HeaderError,
    ParsingError(String),
    WebsocketError(Error),
    GremlinTxError(GremlinError),
    CypherTxError(CypherError),
}