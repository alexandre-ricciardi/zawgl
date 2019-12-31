use super::error::*;
use super::super::lexer::{Token, TokenType};
use super::parser::*;
use super::graph_parser_delegate::*;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub fn parse(parser: &mut Parser) -> ParserResult<Box<AstNode>> {
    if parser.get_tokens().len() > 0  {
        let tok = &parser.get_tokens()[0];
        match tok.token_type {
            TokenType::Create =>  {
                parse_graph(parser)
            },
            _ => Err(ParserError::SyntaxError)
        }
    } else {
        Err(ParserError::SyntaxError)
    }
}
