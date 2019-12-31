use super::error::*;
use super::super::lexer::{Token, TokenType};
use super::parser::*;
use super::graph_parser_delegate::GraphParserDelegate;

pub struct CypherParser {
    parser: Parser,
    graph_parser_delegate: GraphParserDelegate,
}

impl CypherParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let parser = Parser::new(tokens);
        CypherParser {parser: parser, graph_parser_delegate: GraphParserDelegate {parser: parser}}
    }

    pub fn parse(&mut self) -> ParserResult<Box<AstNode>> {
        if self.parser.get_tokens().len() > 0  {
            let tok = &self.parser.get_tokens()[0];
            match tok.token_type {
                TokenType::Create =>  {
                    self.graph_parser_delegate.enter_create(0)
                },
                _ => Err(ParserError::SyntaxError)
            }
        } else {
            Err(ParserError::SyntaxError)
        }
    }
}