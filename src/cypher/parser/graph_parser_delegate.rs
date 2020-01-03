use super::parser::*;
use super::error::*;
use super::super::lexer::{TokenType};

fn enter_identifier(parser: &mut Parser, mut parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    if parser.current_token_type(TokenType::Identifier) {
        let id_node = Box::new(AstNode::new(parser.index - 1));
        parent_node.childs.push(id_node);
        Ok(parent_node)
    } else {
        Err(ParserError::SyntaxError)
    }
    
}

fn enter_labels(parser: &mut Parser, parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    if parser.check(TokenType::Identifier) {
        let id_res = enter_identifier(parser, parent_node)?;
        if parser.current_token_type(TokenType::Colon) {
            return enter_labels(parser, id_res);
        } else {
            return Ok(id_res);
        }
    }
    Ok(parent_node)
}

fn enter_node_def(parser: &mut Parser, parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    let req_open_par = parser.require(parent_node, TokenType::OpenParenthesis)?;
    let id_res = enter_identifier(parser, req_open_par)?;
    let req_sc = parser.require(id_res, TokenType::Colon)?;
    let labels = enter_labels(parser, req_sc)?;
    let req_close_par = parser.require(labels, TokenType::CloseParenthesis)?;
    Ok(req_close_par)
}

fn enter_rel(parser: &mut Parser, parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    if parser.current_token_type(TokenType::Identifier) {
        let id_res = enter_identifier(parser, parent_node)?;
        if parser.current_token_type(TokenType::Identifier) {

        }
    }
}

fn enter_rel_def(parser: &mut Parser, parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    match parser.get_current_token_type() {
        TokenType::LeftSourceRel |
        TokenType::RightSourceRel |
        TokenType::LeftTargetRel |
        TokenType::RightTargetRel => {
            parser.advance();
            let id_tags = enter_rel_tags(parser, parent_node)?;
            Ok(id_tags)
        },
        _ => {
            Err(ParserError::SyntaxError)
        }
    }
}

pub fn parse_graph(parser: &mut Parser, parent_node: Box<AstNode>) -> ParserResult<Box<AstNode>> {
    let graph = enter_node_def(parser, parent_node)?;
    let rel = enter_rel_def(parser, graph)?;
    Ok(rel)
}
