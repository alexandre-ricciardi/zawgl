// MIT License
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

use super::properties_parser_delegate::*;

enum RelationshipState {
    Initial,
    LeftSourceRel,
    LeftTargetRel,
    UndirectedRel,
    LeftDirectedRel,
    RightDirectedRel,
    Invalid,
}

struct RelationshipFsm {
    state: RelationshipState,
}

impl RelationshipFsm {
    fn new() -> Self {
        RelationshipFsm {state: RelationshipState::Initial}
    }
    fn run(&mut self, token_type: TokenType) {
        match token_type {
            TokenType::LeftSourceRel => {
                match self.state {
                    RelationshipState::Initial => self.state = RelationshipState::LeftSourceRel,
                    _ => self.state = RelationshipState::Invalid
                }
            },
            TokenType::LeftTargetRel => {
                match self.state {
                    RelationshipState::Initial => self.state = RelationshipState::LeftTargetRel,
                    _ => self.state = RelationshipState::Invalid
                }
            },
            TokenType::RightSourceRel => {
                match self.state {
                    RelationshipState::LeftSourceRel => self.state = RelationshipState::UndirectedRel,
                    RelationshipState::LeftTargetRel => self.state = RelationshipState::LeftDirectedRel,
                    _ => self.state = RelationshipState::Invalid
                }
            },
            TokenType::RightTargetRel => {
                match self.state {
                    RelationshipState::LeftSourceRel => self.state = RelationshipState::RightDirectedRel,
                    _ => self.state = RelationshipState::Invalid
                }
            }
            _ => self.state = RelationshipState::Invalid
        }
    }
    fn has_invalid_state(&self) -> bool {
        matches!(self.state, RelationshipState::Invalid)
    }
    fn convert_to_ast_tag(&self) -> Option<AstTag> {
        match self.state {
            RelationshipState::RightDirectedRel => Some(AstTag::RelDirectedLR),
            RelationshipState::LeftDirectedRel => Some(AstTag::RelDirectedRL),
            RelationshipState::UndirectedRel => Some(AstTag::RelUndirected),
            _ => None
        }
    }
}


fn enter_identifier(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        let id_node = make_ast_token(&parser);
        parent_node.append(id_node);
        Ok(parser.index)
    } else {
        Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
    }
    
}

fn enter_labels(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.check(TokenType::Identifier) {
        let mut label_tag = Box::new(AstTagNode::new_tag(AstTag::Label));
        enter_identifier(parser, &mut label_tag)?;
        if parser.current_token_type_advance(TokenType::Colon) {
            parent_node.append(label_tag);
            return enter_labels(parser, parent_node);
        } else {
            parent_node.append(label_tag);
            return Ok(());
        }
    }
    Ok(())
}

fn enter_node_def(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut node = Box::new(AstTagNode::new_tag(AstTag::Node));
    
    parser.require(TokenType::OpenParenthesis)?;
    let mut var_node = Box::new(AstTagNode::new_tag(AstTag::Variable));
    enter_identifier(parser, &mut var_node)?;
    node.append(var_node);

    if parser.current_token_type_advance(TokenType::Colon) {
        enter_labels(parser, &mut node)?;
    }

    enter_properties(parser, &mut node)?;

    parser.require(TokenType::CloseParenthesis)?;
    parent_node.append(node);


    enter_rel_def(parser, parent_node)?;
    
    Ok(())
}

fn enter_rel_tags(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.check(TokenType::Identifier) {
        let mut label_tag = Box::new(AstTagNode::new_tag(AstTag::Label));
        enter_identifier(parser, &mut label_tag)?;
        if parser.current_token_type_advance(TokenType::Pipe) {
            parent_node.append(label_tag);
            return enter_rel_tags(parser, parent_node);
        } else {
            parent_node.append(label_tag);
            return Ok(());
        }
    }
    Ok(())
}

fn enter_rel_id(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.check(TokenType::Identifier) {
        let mut var_node = Box::new(AstTagNode::new_tag(AstTag::Variable));        
        enter_identifier(parser, &mut var_node)?;
        parent_node.append(var_node);
        if parser.current_token_type_advance(TokenType::Colon) {
            Ok(enter_rel_tags(parser, parent_node)?)
        } else {
            Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
        }
    } else if parser.current_token_type_advance(TokenType::Colon) {
        Ok(enter_rel_tags(parser, parent_node)?)
    } else {
        Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
    }
}

fn enter_rel_def(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut rel_fsm = RelationshipFsm::new();
    if parser.has_next() {
        match parser.get_current_token_type() {
            TokenType::LeftSourceRel |
            TokenType::LeftTargetRel |
            TokenType::RightSourceRel |
            TokenType::RightTargetRel => {
                rel_fsm.run(parser.get_current_token_type());
                if rel_fsm.has_invalid_state() {
                    Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
                } else {
                    let mut rel = Box::new(AstTagNode::new_empty());
                    parser.advance();
                    enter_rel_id(parser, &mut rel)?;
                    exit_rel_def(parser, rel, &mut rel_fsm, parent_node)?;
                    Ok(())
                }
            },
            TokenType::Return | TokenType::Where | TokenType::With | TokenType::Create | TokenType::Match | TokenType::Comma => Ok(()),
            _ => {
                Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
            }
        }
    } else {
        Ok(())
    }
    
}

fn exit_rel_def(parser: &mut Parser, mut rel_node: Box<AstTagNode>, rel_fsm: &mut RelationshipFsm, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.has_next() {
        match parser.get_current_token_type() {
            TokenType::RightSourceRel |
            TokenType::RightTargetRel => {
                rel_fsm.run(parser.get_current_token_type());
                match rel_fsm.convert_to_ast_tag() {
                    Some(tag) => {
                        rel_node.ast_tag = Some(tag);
                        parser.advance();
                        parent_node.append(rel_node);
                        Ok(enter_node_def(parser, parent_node)?)
                    },
                    None => Err(ParserError::SyntaxError(parser.index, parser.get_current_token_value()))
                }
            },
            _ => {
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

pub fn parse_path(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut path = Box::new(AstTagNode::new_tag(AstTag::Path));
    
    
    enter_node_def(parser, &mut path)?;
    
    if parser.current_token_type_advance(TokenType::Comma) {
        parse_path(parser, parent_node)?;
    }

    parent_node.append(path);
    Ok(())
}
