pub mod lexer;
pub mod parser;

#[cfg(test)]
mod parser_tests {
use super::*;
    #[test]
    fn test_create() {
        let qry = "CREATE (n:Person { name: 'Andy', title: 'Developer' })";
        let mut lexer = lexer::Lexer::new(&qry);
        let mut parser = parser::Parser::new();
        loop {
            let res = lexer.next_token();
            match res {
                Ok(token) => parser.consume(token),
                Err(_) => break,
            }           
        }
        
        


    }
}