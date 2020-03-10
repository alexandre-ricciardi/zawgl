use super::*;
use super::error::*;
use super::super::lexer::{TokenType};

fn enter_string_expr(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::StringType) {
        let mut str_tag = Box::new(AstNode::new_token_type(TokenType::StringType));
        let str_node = Box::new(AstNode::new(parser.index - 1));
        str_tag.childs.push(str_node);
        parent_node.childs.push(str_tag);
        if parser.current_token_type_advance(TokenType::Plus) {
            enter_string_expr(parser, parent_node)
        } else {
            Ok(parser.index)
        }
    } else {
        Err(ParserError::SyntaxError)
    }
}

fn enter_prop_value(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    match parser.get_current_token_type() {
        TokenType::StringType => {
            enter_string_expr(parser, parent_node)
        },
        TokenType::True |
        TokenType::False => {
            Err(ParserError::SyntaxError)
        }
        TokenType::Number => {
            Err(ParserError::SyntaxError)
        },
        _ => {
            Err(ParserError::SyntaxError)
        }
    }
}

fn enter_prop_name(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    let id_node = Box::new(AstNode::new(parser.index - 1));
    parent_node.childs.push(id_node);
    Ok(parser.index)
}


fn enter_property(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        let mut prop_node = Box::new(AstNode::new_tag(AstTag::Property));
        enter_prop_name(parser, &mut prop_node)?;
        parser.require(TokenType::Colon)?;
        enter_prop_value(parser, &mut prop_node)?;
        parent_node.childs.push(prop_node);
        if parser.current_token_type_advance(TokenType::Comma) {
            enter_property(parser, parent_node)
        } else {
            Ok(parser.index)
        }
    } else {
        Err(ParserError::SyntaxError)
    }
}

pub fn enter_properties(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::OpenBrace) {
        enter_property(parser, parent_node)?;
        parser.require(TokenType::CloseBrace)?;
        Ok(parser.index)
    } else {
        Ok(parser.index)
    }
}