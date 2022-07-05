use zawgl_cypher::CypherError;
use zawgl_gremlin::handler::GremlinError;
use tokio_tungstenite::tungstenite::Error;

#[derive(Debug)]
pub enum ServerError {
    HeaderError,
    ParsingError(String),
    WebsocketError(Error),
    GremlinTxError(GremlinError),
    CypherTxError(CypherError),
}