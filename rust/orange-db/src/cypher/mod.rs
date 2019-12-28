pub mod lexer;
pub mod parser;

#[cfg(test)]
mod parser_tests {
use super::*;

    #[test]
    fn test_create() {
        let qry = "CREATE (n:Person { name: 'Andy', title: 'Developer' })";
        let mut lexer = lexer::Lexer::new(&qry);
        let vtoks = lexer.get_tokens();
        match vtoks {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let root = parser.parse();
            },
            Err(value) => panic!("{}", value)
        }
    }
}