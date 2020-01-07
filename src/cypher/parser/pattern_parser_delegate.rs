use super::parser::*;
use super::error::*;
use super::super::lexer::{TokenType};
use super::properties_parser_delegate::*;

fn enter_identifier(parser: &mut Parser, parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        let id_node = Box::new(AstNode::new(parser.index - 1));
        parent_node.childs.push(id_node);
        Ok(parser.index)
    } else {
        Err(ParserError::SyntaxError)
    }
    
}

fn enter_labels(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.check(TokenType::Identifier) {
        enter_identifier(parser, parent_node)?;
        if parser.current_token_type_advance(TokenType::Colon) {
            return enter_labels(parser, &mut parent_node);
        } else {
            return Ok(parser.index);
        }
    }
    Ok(parser.index)
}

fn enter_node_def(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    parser.require(TokenType::OpenParenthesis)?;
    enter_identifier(parser,&mut parent_node)?;
    parser.require(TokenType::Colon)?;
    enter_labels(parser, &mut parent_node)?;

    enter_properties(parser, parent_node)?;

    parser.require(TokenType::CloseParenthesis)?;

    
    enter_rel_def(parser, &mut parent_node)?;
    

    Ok(parser.index)
}

fn enter_rel_tags(parser: &mut Parser, parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.check(TokenType::Identifier) {
        enter_identifier(parser, parent_node)?;
        if parser.current_token_type_advance(TokenType::Pipe) {
            return enter_rel_tags(parser, parent_node);
        } else {
            return Ok(parser.index);
        }
    }
    Ok(parser.index)
}

fn enter_rel_id(parser: &mut Parser, mut parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.check(TokenType::Identifier) {
        enter_identifier(parser, parent_node)?;
        if parser.current_token_type_advance(TokenType::Colon) {
            Ok(enter_rel_tags(parser, parent_node)?)
        } else {
            Err(ParserError::SyntaxError)
        }
    } else {
        if parser.current_token_type_advance(TokenType::Colon) {
            Ok(enter_rel_tags(parser, parent_node)?)
        } else {
            Err(ParserError::SyntaxError)
        }
    }
}

fn enter_rel_def(parser: &mut Parser, parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.has_next() {
        match parser.get_current_token_type() {
            TokenType::LeftSourceRel |
            TokenType::RightSourceRel |
            TokenType::LeftTargetRel |
            TokenType::RightTargetRel => {
                parser.advance();
                let mut rel = Box::new(AstNode::new_tag(AstTag::Relationship));
                enter_rel_id(parser, &mut rel)?;
                exit_rel_def(parser, &mut rel)?;
                parent_node.childs.push(rel);
                Ok(parser.index)
            },
            _ => {
                Ok(parser.index)
            }
        }
    } else {
        Ok(parser.index)
    }
    
}

pub fn exit_rel_def(parser: &mut Parser, parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    if parser.has_next() {
        match parser.get_current_token_type() {
            TokenType::LeftSourceRel |
            TokenType::RightSourceRel |
            TokenType::LeftTargetRel |
            TokenType::RightTargetRel => {
                parser.advance();
                Ok(parse_pattern(parser, parent_node)?)
            },
            _ => {
                Ok(parser.index)
            }
        }
    } else {
        Ok(parser.index)
    }
}

pub fn parse_pattern(parser: &mut Parser, parent_node: &mut Box<AstNode>) -> ParserResult<usize> {
    let mut node = Box::new(AstNode::new_tag(AstTag::Node));
    enter_node_def(parser, &mut node)?;
    parent_node.childs.push(node);
    Ok(parser.index)
}
