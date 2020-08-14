use super::error::*;
use super::super::lexer::{TokenType};
use super::*;
use super::pattern_parser_delegate::*;

pub fn parse(parser: &mut Parser) -> ParserResult<Box<dyn Ast>> {
    if parser.get_tokens().len() > 0  {
        let mut query_node = Box::new(AstTagNode::new_tag(AstTag::Query));

        let tok = &parser.get_tokens()[0];
        match tok.token_type {
            TokenType::Create =>  {
                parser.advance();
                let mut create_node = Box::new(AstTagNode::new_tag(AstTag::Create));
                parse_pattern(parser, &mut create_node)?;
                query_node.append(create_node);
                parse_where_clause(parser, &mut query_node)?;
                parse_return(parser, &mut query_node)?;
                
                Ok(query_node)
                
            },
            TokenType::Match => {
                parser.advance();
                let mut match_node = Box::new(AstTagNode::new_tag(AstTag::Match));
                parse_pattern(parser, &mut match_node)?;
                query_node.append(match_node);
                parse_where_clause(parser, &mut query_node)?;
                parse_return(parser, &mut query_node)?;
                
                Ok(query_node)
            },
            _ => Err(ParserError::SyntaxError(parser.index))
        }
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
}

fn parse_return(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.has_next() && parser.check(TokenType::Return) {
        parser.require(TokenType::Return)?;
        let mut ret_node = Box::new(AstTagNode::new_tag(AstTag::Return));
        parse_return_expression(parser, &mut ret_node)?;
        parent_node.append(ret_node);
    }
    Ok(())
}

fn parse_return_expression(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        let mut item_id = make_ast_token(&parser);
        if parser.current_token_type_advance(TokenType::OpenParenthesis) {
            let mut func_node = Box::new(AstTagNode::new_tag(AstTag::Function));
            parse_func_args(parser, &mut item_id)?;
            func_node.append(item_id);
            parent_node.append(func_node);
            parser.require(TokenType::CloseParenthesis)?;
        } else {
            let mut item_node = Box::new(AstTagNode::new_tag(AstTag::Item));
            item_node.append(item_id);
            parent_node.append(item_node);
        }
        if parser.current_token_type_advance(TokenType::Comma) { 
            parse_return_expression(parser, parent_node)?;
        }
    }
    Ok(())
}

fn parse_func_args(parser: &mut Parser, parent_node: &mut Box<AstTokenNode>) -> ParserResult<()> {
    
    while parser.check(TokenType::Identifier) {
        parser.advance();
        let mut func_arg = Box::new(AstTagNode::new_tag(AstTag::FunctionArg));
        func_arg.append(make_ast_token(parser));
        parent_node.append(func_arg);
        if !parser.check(TokenType::Comma) {
            break;
        } else {
            parser.advance();
        }
    }
    Ok(())
}

fn parse_where_clause(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.has_next() && parser.check(TokenType::Where) {
        parser.require(TokenType::Where)?;
        let mut ret_node = Box::new(AstTagNode::new_tag(AstTag::Where));
        parse_expression(parser, &mut ret_node)?;
        parent_node.append(ret_node);
    }
    Ok(())
}

fn parse_expression(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {

    Ok(())
}
