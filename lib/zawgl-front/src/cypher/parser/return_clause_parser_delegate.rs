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

use zawgl_cypher_query_model::ast::{AstTagNode, AstTag, Ast};
use zawgl_cypher_query_model::token::{TokenType};
use super::common_parser_delegate::*;

pub fn parse_return(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
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
            let func = parse_function_definition(parser)?;
            parent_node.append(func);
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