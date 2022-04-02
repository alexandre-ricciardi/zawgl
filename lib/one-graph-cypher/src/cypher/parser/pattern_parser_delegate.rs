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


fn enter_identifier(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<usize> {
    if parser.current_token_type_advance(TokenType::Identifier) {
        let id_node = make_ast_token(&parser);
        parent_node.append(id_node);
        Ok(parser.index)
    } else {
        Err(ParserError::SyntaxError(parser.index))
    }
    
}

fn enter_labels(parser: &mut Parser, mut parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    if parser.check(TokenType::Identifier) {
        let mut label_tag = Box::new(AstTagNode::new_tag(AstTag::Label));
        enter_identifier(parser, &mut label_tag)?;
        if parser.current_token_type_advance(TokenType::Colon) {
            parent_node.append(label_tag);
            return enter_labels(parser, &mut parent_node);
        } else {
            parent_node.append(label_tag);
            return Ok(());
        }
    }
    Ok(())
}

fn enter_node_def(parser: &mut Parser, mut parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    parser.require(TokenType::OpenParenthesis)?;
    let mut var_node = Box::new(AstTagNode::new_tag(AstTag::Variable));
    enter_identifier(parser, &mut var_node)?;
    parent_node.append(var_node);

    if parser.current_token_type_advance(TokenType::Colon) {
        enter_labels(parser, &mut parent_node)?;
    }

    enter_properties(parser, parent_node)?;

    parser.require(TokenType::CloseParenthesis)?;

    enter_rel_def(parser, &mut parent_node)?;
    
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
            Err(ParserError::SyntaxError(parser.index))
        }
    } else {
        if parser.current_token_type_advance(TokenType::Colon) {
            Ok(enter_rel_tags(parser, parent_node)?)
        } else {
            Err(ParserError::SyntaxError(parser.index))
        }
    }
}

fn enter_rel_def(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut rel_fsm = RelationshipFsm::new();
    if parser.has_next() {
        match parser.get_current_token_type() {
            TokenType::LeftSourceRel |
            TokenType::LeftTargetRel => {
                rel_fsm.run(parser.get_current_token_type());
                if rel_fsm.has_invalid_state() {
                    Err(ParserError::SyntaxError(parser.index))
                } else {
                    let mut rel = Box::new(AstTagNode::new_empty());
                    parser.advance();
                    enter_rel_id(parser, &mut rel)?;
                    exit_rel_def(parser, &mut rel, &mut rel_fsm)?;
                    parent_node.append(rel);
                    Ok(())
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

fn exit_rel_def(parser: &mut Parser, parent_node: &mut Box<AstTagNode>, rel_fsm: &mut RelationshipFsm) -> ParserResult<()> {
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
                    None => Err(ParserError::SyntaxError(parser.index))
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

pub fn parse_pattern(parser: &mut Parser, parent_node: &mut Box<AstTagNode>) -> ParserResult<()> {
    let mut pattern = Box::new(AstTagNode::new_tag(AstTag::Pattern));
    let mut node = Box::new(AstTagNode::new_tag(AstTag::Node));
    
    enter_node_def(parser, &mut node)?;
    
    if parser.current_token_type_advance(TokenType::Comma) {
        parse_pattern(parser, parent_node)?;
    }
    pattern.append(node);
    parent_node.append(pattern);
    Ok(())
}
