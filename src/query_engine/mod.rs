use super::cypher::*;
use super::model::*;

pub fn process_query(query: &str) -> Option<Request> {
    let mut lexer = lexer::Lexer::new(query);
    match lexer.get_tokens() {
        Ok(tokens) => {
            let mut parser = parser::parser::Parser::new(tokens);
            parser::cypher_parser::parse(&mut parser).map(|root| build_request(root)).ok()
        },
        Err(value) => None
    }
}

pub fn build_request(root: Box<parser::parser::AstNode>) -> Request {
    Request::new()
}