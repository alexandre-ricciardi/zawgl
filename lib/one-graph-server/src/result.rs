#[derive(Debug)]
pub enum ServerError {
    HeaderError,
    ParsingError(String),
    GremlinError,
    WebsocketError(tungstenite::Error),
}