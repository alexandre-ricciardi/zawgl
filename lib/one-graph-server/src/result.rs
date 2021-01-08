#[derive(Debug)]
pub enum ServerError {
    HeaderError,
    ParsingError(String),
    WebsocketError(tungstenite::Error),
}