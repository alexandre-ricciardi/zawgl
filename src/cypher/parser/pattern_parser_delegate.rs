use super::*;
use super::error::*;
use super::super::lexer::{TokenType};
use super::properties_parser_delegate::*;

enum RelationshipState {
    Initial,
    LeftSourceRel,
    LeftTargetRel,
    RightSourceRel,
    RightTargetRel,
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
    fn has_accepting_state(&self) -> bool {
        match self.state {
            RelationshipState::LeftDirectedRel |
            RelationshipState::RightDirectedRel |
            RelationshipState::UndirectedRel => true,
            _ => false
        }
    }
    fn has_invalid_state(&self) -> bool {
        match self.state {
            RelationshipState::Invalid => true,
            _ => false
        }
    }
    fn get_state(&self) -> &RelationshipState {
        &self.state
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
        let mut label_tag = Box::new(AstNode::new_tag(AstTag::Label));
        enter_identifier(parser, &mut label_tag)?;
        if parser.current_token_type_advance(TokenType::Colon) {
            return enter_labels(parser, &mut label_tag);
        } else {
            parent_node.childs.push(label_tag);
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
    let mut rel_fsm = RelationshipFsm::new();
    if parser.has_next() {
        match parser.get_current_token_type() {
            TokenType::LeftSourceRel |
            TokenType::LeftTargetRel => {
                rel_fsm.run(parser.get_current_token_type());
                if rel_fsm.has_invalid_state() {
                    Err(ParserError::SyntaxError)
                } else {
                    let mut rel = Box::new(AstNode::new_empty());
                    parser.advance();
                    enter_rel_id(parser, &mut rel)?;
                    exit_rel_def(parser, &mut rel, &mut rel_fsm)?;
                    parent_node.childs.push(rel);
                    Ok(parser.index)
                }
            },
            _ => {
                Ok(parser.index)
            }
        }
    } else {
        Ok(parser.index)
    }
    
}

fn exit_rel_def(parser: &mut Parser, parent_node: &mut Box<AstNode>, rel_fsm: &mut RelationshipFsm) -> ParserResult<usize> {
    if parser.has_next() {
        match parser.get_current_token_type() {
            TokenType::RightSourceRel |
            TokenType::RightTargetRel => {
                rel_fsm.run(parser.get_current_token_type());
                match rel_fsm.convert_to_ast_tag() {
                    Some(tag) => {
                        parent_node.ast_tag = Some(tag);
                        parser.advance();
                        Ok(parse_pattern(parser, parent_node)?)
                    },
                    None => Err(ParserError::SyntaxError)
                }
                
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
