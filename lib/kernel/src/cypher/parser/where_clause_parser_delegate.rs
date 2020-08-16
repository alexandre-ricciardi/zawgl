use super::*;
use super::error::*;
use super::super::lexer::TokenType;
use super::common_parser_delegate::*;

pub fn parse_where_clause(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.check(TokenType::Where) {
        parser.require(TokenType::Where)?;
        let mut where_node = Box::new(AstTagNode::new_tag(AstTag::Where));
        where_node.append(parse_boolean_expression(parser)?);
        parent_node.append(where_node);
    }
    Ok(())
}

fn parse_boolean_operator(parser: &mut Parser, prev_expr: Box<AstTagNode>) -> ParserResult<Box<AstTagNode>> {
    if parser.check(TokenType::And) {
        parser.advance();
        let mut operator = make_ast_tag(AstTag::AndOperator);
        let mut expr = parse_boolean_expression(parser)?;
        if expr.ast_tag == Some(AstTag::OrOperator) {
            operator.append(prev_expr);
            operator.append(expr.childs.remove(0));
            expr.append(operator);
            Ok(expr)
        } else {
            operator.append(prev_expr);
            operator.append(expr);
            Ok(operator)
        }
        
    } else if parser.check(TokenType::Or) {
        parser.advance();
        let mut operator = make_ast_tag(AstTag::OrOperator);
        let expr = parse_boolean_expression(parser)?;
        operator.append(prev_expr);
        operator.append(expr);
        Ok(operator)
    } else {
        Ok(prev_expr)
    }
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
            let func = parse_function_definition(parser)?;
            parent_node.append(func);
            Ok(())
        },
        TokenType::OpenParenthesis => {
            parser.advance();
            parse_boolean_expression(parser)?;
            parser.require(TokenType::CloseParenthesis)?;
            Ok(())
        },
        TokenType::StringType => {
            parser.advance();
            parent_node.append(make_ast_token(parser));
            Ok(())
        },
        _ => {
            Err(ParserError::SyntaxError(parser.index))
        }
    }
}

fn parse_boolean_expression(parser: &mut Parser) -> ParserResult<Box<AstTagNode>> {
    match parser.get_current_token_type() {
        TokenType::Integer => {
            parser.advance();
            parser.require(TokenType::Equals)?;
            let mut eqop = make_ast_tag(AstTag::EqualityOperator);
            parse_boolean_expression_terminal(parser, &mut eqop)?;
            parse_boolean_operator(parser, eqop)
        },
        TokenType::Float => {
            parser.advance();
            parser.require(TokenType::Equals)?;
            let mut eqop = make_ast_tag(AstTag::EqualityOperator);
            parse_boolean_expression_terminal(parser, &mut eqop)?;
            parse_boolean_operator(parser, eqop)
        },
        TokenType::True | TokenType::False => {
            parser.advance();
            parser.require(TokenType::Equals)?;
            let mut eqop = make_ast_tag(AstTag::EqualityOperator);
            parse_boolean_expression_terminal(parser, &mut eqop)?;
            parse_boolean_operator(parser, eqop)
        },
        TokenType::Identifier => {
            parser.advance();
            
            if parser.check(TokenType::OpenParenthesis) {
                let func = parse_function_definition(parser)?;
                if parser.check(TokenType::Equals) {
                    parser.advance();
                    let mut eqop = make_ast_tag(AstTag::EqualityOperator);
                    eqop.append(func);
                    parse_boolean_expression_terminal(parser, &mut eqop)?;
                    return parse_boolean_operator(parser, eqop)
                } else {
                    return parse_boolean_operator(parser, func)
                }
            } else if parser.check(TokenType::Dot) {
                let mut item_prop = make_ast_tag(AstTag::ItemPropertyIdentifier);
                item_prop.append(make_ast_token(parser));
                parser.advance();
                if parser.check(TokenType::Identifier) {
                    parser.advance();
                    item_prop.append(make_ast_token(parser));
                    parser.require(TokenType::Equals)?;
                    let mut eqop = make_ast_tag(AstTag::EqualityOperator);
                    eqop.append(item_prop);
                    parse_boolean_expression_terminal(parser, &mut eqop)?;
                    return parse_boolean_operator(parser, eqop)
                } else {
                    return Err(ParserError::SyntaxError(parser.index))
                }
            } else {
                return Err(ParserError::SyntaxError(parser.index))
            }
        },
        TokenType::OpenParenthesis => {
            parser.advance();
            let expr = parse_boolean_expression(parser)?;
            parser.require(TokenType::CloseParenthesis)?;
            return parse_boolean_operator(parser, expr)
        },
        _ => {
            Err(ParserError::SyntaxError(parser.index))
        }
    }
}
