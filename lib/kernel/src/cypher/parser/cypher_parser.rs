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
        if parser.check(TokenType::OpenParenthesis) {
            parse_function_definition(parser, parent_node)?;
        } else {
            let item_id = make_ast_token(&parser);
            let mut item_node = make_ast_tag(AstTag::Item);
            item_node.append(item_id);
            parent_node.append(item_node);
        }
        if parser.current_token_type_advance(TokenType::Comma) { 
            parse_return_expression(parser, parent_node)?;
        }
    }
    Ok(())
}

fn parse_function_definition(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut item_id = make_ast_token(&parser);
    parser.require(TokenType::OpenParenthesis)?;
    let mut func_node = make_ast_tag(AstTag::Function);
    parse_func_args(parser, &mut item_id)?;
    func_node.append(item_id);
    parent_node.append(func_node);
    parser.require(TokenType::CloseParenthesis)?;
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
        parse_boolean_expression(parser, &mut ret_node)?;
        parent_node.append(ret_node);
    }
    Ok(())
}

fn parse_boolean_operator(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.check(TokenType::And) {
        parser.advance();
        let mut operator = make_ast_tag(AstTag::AndOperator);
        parse_boolean_expression(parser, &mut operator)?;
        parent_node.append(operator);
    } else if parser.check(TokenType::Or) {
        parser.advance();
        let mut operator = make_ast_tag(AstTag::OrOperator);
        parse_boolean_expression(parser, &mut operator)?;
        parent_node.append(operator);
    }
    
    Ok(())
}

fn parse_boolean_expression_terminal(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    match parser.get_current_token_type() {
        TokenType::Integer => {
            parser.advance();
            parent_node.append(make_ast_token(parser));
            Ok(())
        },
        TokenType::Float => {
            parser.advance();
            parent_node.append(make_ast_token(parser));
            Ok(())
        },
        TokenType::True | TokenType::False => {
            parser.advance();
            parent_node.append(make_ast_token(parser));
            Ok(())
        },
        TokenType::Identifier => {
            parser.advance();
            parse_function_definition(parser, parent_node)?;
            Ok(())
        },
        TokenType::OpenParenthesis => {
            parser.advance();
            parse_boolean_expression(parser, parent_node)?;
            parser.require(TokenType::CloseParenthesis)?;
            Ok(())
        },
        _ => {
            Err(ParserError::SyntaxError(parser.index))
        }
    }
}

fn parse_boolean_expression(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    match parser.get_current_token_type() {
        TokenType::Integer => {
            parser.advance();
            parser.require(TokenType::Equals)?;
            let mut eqop = make_ast_tag(AstTag::EqualityOperator);
            parse_boolean_expression_terminal(parser, &mut eqop)?;
            parse_boolean_operator(parser, &mut eqop)
        },
        TokenType::Float => {
            parser.advance();
            parser.require(TokenType::Equals)?;
            let mut eqop = make_ast_tag(AstTag::EqualityOperator);
            parse_boolean_expression_terminal(parser, &mut eqop)?;
            parse_boolean_operator(parser, &mut eqop)
        },
        TokenType::True | TokenType::False => {
            parser.advance();
            parser.require(TokenType::Equals)?;
            let mut eqop = make_ast_tag(AstTag::EqualityOperator);
            parse_boolean_expression_terminal(parser, &mut eqop)?;
            parse_boolean_operator(parser, &mut eqop)
        },
        TokenType::Identifier => {
            parser.advance();
            parse_function_definition(parser, parent_node)?;
            parser.require(TokenType::Equals)?;
            let mut eqop = make_ast_tag(AstTag::EqualityOperator);
            parse_boolean_expression_terminal(parser, &mut eqop)?;
            parse_boolean_operator(parser, &mut eqop)?;
            parent_node.append(eqop);
            Ok(())
        },
        TokenType::OpenParenthesis => {
            parser.advance();
            parse_boolean_expression(parser, parent_node)?;
            parser.require(TokenType::CloseParenthesis)?;
            parse_boolean_operator(parser, parent_node)
        },
        _ => {
            Err(ParserError::SyntaxError(parser.index))
        }
    }
}
