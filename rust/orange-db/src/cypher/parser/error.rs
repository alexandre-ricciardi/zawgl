use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum ParserError {
    SyntaxError,
    EndOfFile
}

pub type ParserResult<T> = std::result::Result<T, ParserError>;


impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserError::SyntaxError => f.write_str("NotFound"),
            ParserError::EndOfFile => f.write_str("InternalServerError"),
        }
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::SyntaxError => "Record not found",
            ParserError::EndOfFile => "Internal server error",
        }
    }
}
