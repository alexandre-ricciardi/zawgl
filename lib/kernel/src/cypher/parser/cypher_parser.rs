use super::error::*;
use super::super::lexer::{TokenType};
use super::*;
use super::pattern_parser_delegate::*;

pub fn parse(parser: &mut Parser) -> ParserResult<Box<dyn Ast>> {
    if parser.get_tokens().len() > 0  {
        let tok = &parser.get_tokens()[0];
        match tok.token_type {
            TokenType::Create =>  {
                parser.advance();
                let mut create_node = Box::new(AstTagNode::new_tag(AstTag::Create));
                let res = parse_pattern(parser, &mut create_node);
                if res.is_err() {
                    Err(res.err().unwrap())
                } else {
                    Ok(create_node)
                }
                
            },
            TokenType::Match => {
                parser.advance();
                let mut create_node = Box::new(AstTagNode::new_tag(AstTag::Match));
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
