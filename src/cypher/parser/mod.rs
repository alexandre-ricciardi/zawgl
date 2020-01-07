pub mod error;
pub mod parser_utils;
pub mod parser;
pub mod pattern_parser_delegate;
pub mod properties_parser_delegate;
pub mod cypher_parser;

use super::lexer::*;


#[cfg(test)]
mod test_parser {
    use super::*;

    fn run(qry: &str) {
        let mut lexer = Lexer::new(qry);
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
    fn test_create() {
        run("CREATE (n:Person)");
    }
    #[test]
    fn test_create_labels() {
        run("CREATE (n:Person:Friend:Etc)");
    }

    #[test]
    fn test_create_graph() {
        run("CREATE (n:Person)-[r:FRIEND_OF]->(m:Person)");
    }

    #[test]
    fn test_properties_node() {
        run("CREATE (n:Person { name: 'hello', value: 'world' })");
    }

}

