pub mod error;
pub mod parser_utils;
pub mod parser;
pub mod pattern_parser_delegate;
pub mod cypher_parser;

use super::lexer::*;


#[cfg(test)]
mod test_parser {
    use super::*;
    #[test]
    fn test_create() {
        let qry = "CREATE (n:Person)";
        let mut lexer = Lexer::new(&qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                parser_utils::print_node(&root.unwrap(), parser.get_tokens(), 0);
            },
            Err(value) => assert!(false)
        }
    }
    #[test]
    fn test_create_labels() {
        let qry = "CREATE (n:Person:Friend:Etc)";
        let mut lexer = Lexer::new(&qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                parser_utils::print_node(&root.unwrap(), parser.get_tokens(), 0);
            },
            Err(value) => assert!(false)
        }
    }

    #[test]
    fn test_create_graph() {
        let qry = "CREATE (n:Person)-[r:FRIEND_OF]->(m:Person)";
        let mut lexer = Lexer::new(&qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = parser::Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                parser_utils::print_node(&root.unwrap(), parser.get_tokens(), 0);
            },
            Err(value) => {
                println!("{}", value);
                assert!(false)
            }
        }
    }

}

