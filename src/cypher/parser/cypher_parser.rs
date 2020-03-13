use super::error::*;
use super::super::lexer::{Token, TokenType};
use super::*;
use super::pattern_parser_delegate::*;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub fn parse(parser: &mut Parser) -> ParserResult<Box<AstNode>> {
    if parser.get_tokens().len() > 0  {
        let tok = &parser.get_tokens()[0];
        match tok.token_type {
            TokenType::Create =>  {
                parser.advance();
                let mut create_node = make_ast_token(&parser);
                let res = parse_pattern(parser, &mut create_node);
                if res.is_err() {
                    Err(res.err().unwrap())
                } else {
                    Ok(create_node)
                }
                
            },
            _ => Err(ParserError::SyntaxError)
        }
    } else {
        Err(ParserError::SyntaxError)
    }
}
