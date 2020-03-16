use super::cypher::*;
use super::model::*;
use super::cypher::parser::*;

pub fn process_query(query: &str) -> Option<Request> {
    let mut lexer = lexer::Lexer::new(query);
    match lexer.get_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            parser::cypher_parser::parse(&mut parser).ok();
            Some(Request::new(Directive::CREATE))
        }
        Err(value) => None
    }
}