use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
    SyntaxError(usize),
    EndOfFile
}

pub type ParserResult<T> = std::result::Result<T, ParserError>;


impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserError::SyntaxError(index) => f.write_str(&format!("Syntax Error Around {}", index)),
            ParserError::EndOfFile => f.write_str("InternalServerError"),
        }
    }
}
