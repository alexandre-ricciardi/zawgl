// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::*;
use super::error::*;
use super::super::lexer::{TokenType};

fn enter_string_expr(parser: &mut Parser, parent_node: &mut Box<dyn Ast>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::StringType) {
        let str_node = make_ast_token(&parser);
        parent_node.append(str_node);
        if parser.current_token_type_advance(TokenType::Plus) {
            enter_string_expr(parser, parent_node)
        } else {
            Ok(parser.index)
        }
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
}

fn enter_float_expr(parser: &mut Parser, parent_node: &mut Box<dyn Ast>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Float) {
        let float_node = make_ast_token(&parser);
        parent_node.append(float_node);
        Ok(parser.index)
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
}

fn enter_integer_expr(parser: &mut Parser, parent_node: &mut Box<dyn Ast>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Integer) {
        let int_node = make_ast_token(&parser);
        parent_node.append(int_node);
        Ok(parser.index)
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
}

fn enter_bool_expr(parser: &mut Parser, parent_node: &mut Box<dyn Ast>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::True) {
        let bool_node = make_ast_token(&parser);
        parent_node.append(bool_node);
        Ok(parser.index)
    } else if parser.current_token_type_advance(TokenType::False) {
        let bool_node = make_ast_token(&parser);
        parent_node.append(bool_node);
        Ok(parser.index)
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
}

fn enter_prop_value(parser: &mut Parser, parent_node: &mut Box<dyn Ast>) -> ParserResult<usize> {
    match parser.get_current_token_type() {
        TokenType::StringType => {
            enter_string_expr(parser, parent_node)
        },
        TokenType::True |
        TokenType::False => {
            enter_bool_expr(parser, parent_node)
        }
        TokenType::Float => {
            enter_float_expr(parser, parent_node)
        },
        TokenType::Integer => {
            enter_integer_expr(parser, parent_node)
        },
        _ => {
            Err(ParserError::SyntaxError(parser.index))
        }
    }
}

fn enter_prop_name(parser: &mut Parser, parent_node: &mut Box<dyn Ast>) -> ParserResult<usize> {
    let id_node = make_ast_token(&parser);
    parent_node.append(id_node);
    Ok(parser.index)
}


fn enter_property(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        let mut prop_node: Box<dyn Ast> = Box::new(AstTagNode::new_tag(AstTag::Property));
        enter_prop_name(parser, &mut prop_node)?;
        parser.require(TokenType::Colon)?;
        enter_prop_value(parser, &mut prop_node)?;
        parent_node.append(prop_node);
        if parser.current_token_type_advance(TokenType::Comma) {
            enter_property(parser, parent_node)
        } else {
            Ok(parser.index)
        }
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
}

pub fn enter_properties(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::OpenBrace) {
        enter_property(parser, parent_node)?;
        parser.require(TokenType::CloseBrace)?;
        Ok(parser.index)
    } else {
        Ok(parser.index)
    }
}