// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
//
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

use std::fmt;

use crate::token::TokenType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstTag  {
    Create,
    Match,
    Node,
    Path,
    Property,
    RelDirectedLR,
    RelDirectedRL,
    RelUndirected,
    Variable,
    Label,
    Query,
    Return,
    Where,
    Function,
    FunctionArg,
    Item,
    AndOperator,
    OrOperator,
    EqualityOperator,
    ItemPropertyIdentifier,
    Parameter,
}

pub trait AstVisitor {
    fn enter_create(&mut self,) -> AstVisitorResult;
    fn enter_match(&mut self) -> AstVisitorResult;
    fn enter_path(&mut self) -> AstVisitorResult;
    fn enter_node(&mut self) -> AstVisitorResult;
    fn enter_relationship(&mut self, node: &AstTagNode) -> AstVisitorResult;
    fn enter_property(&mut self) -> AstVisitorResult;
    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult;
    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult;
    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult;
    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult;
    fn enter_identifier(&mut self, key: &str) -> AstVisitorResult;
    fn enter_variable(&mut self) -> AstVisitorResult;
    fn enter_label(&mut self) -> AstVisitorResult;
    fn enter_query(&mut self) -> AstVisitorResult;
    fn enter_return(&mut self) -> AstVisitorResult;
    fn enter_function(&mut self) -> AstVisitorResult;
    fn enter_function_arg(&mut self) -> AstVisitorResult;
    fn enter_item(&mut self) -> AstVisitorResult;
    fn enter_where(&mut self, node: &AstTagNode) -> AstVisitorResult;
    fn enter_parameter(&mut self, name: &str) -> AstVisitorResult;
    fn enter_equality_operator(&mut self) -> AstVisitorResult;
    fn enter_and_operator(&mut self) -> AstVisitorResult;
    fn exit_create(&mut self) -> AstVisitorResult;
    fn exit_match(&mut self) -> AstVisitorResult;
    fn exit_path(&mut self) -> AstVisitorResult;
    fn exit_node(&mut self) -> AstVisitorResult;
    fn exit_relationship(&mut self) -> AstVisitorResult;
    fn exit_property(&mut self) -> AstVisitorResult;
    fn exit_integer_value(&mut self) -> AstVisitorResult;
    fn exit_float_value(&mut self) -> AstVisitorResult;
    fn exit_string_value(&mut self) -> AstVisitorResult;
    fn exit_bool_value(&mut self) -> AstVisitorResult;
    fn exit_identifier(&mut self) -> AstVisitorResult;
    fn exit_variable(&mut self) -> AstVisitorResult;
    fn exit_label(&mut self) -> AstVisitorResult;
    fn exit_query(&mut self) -> AstVisitorResult;
    fn exit_return(&mut self) -> AstVisitorResult;
    fn exit_function(&mut self) -> AstVisitorResult;
    fn exit_function_arg(&mut self) -> AstVisitorResult;
    fn exit_item(&mut self) -> AstVisitorResult;
    fn exit_where(&mut self) -> AstVisitorResult;
    fn exit_parameter(&mut self) -> AstVisitorResult;
    fn exit_equality_operator(&mut self) -> AstVisitorResult;
    fn exit_and_operator(&mut self) -> AstVisitorResult;
}

#[derive(Debug, Clone)]
pub enum AstVisitorError {
    SyntaxError,
}

pub type AstVisitorResult = std::result::Result<(), AstVisitorError>;

pub trait Ast : fmt::Display {
    fn append(&mut self, ast: Box<dyn Ast>);
    fn accept(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult;
    fn accept_exit(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult;
    fn get_childs(&self) -> &Vec<Box<dyn Ast>>;
    fn clone_ast(&self) -> Box<dyn Ast>;
}

pub struct AstTokenNode {
    pub token_id: usize,
    pub token_value: String,
    pub childs: Vec<Box<dyn Ast>>,
    pub token_type: TokenType,
}

pub struct AstTagNode {
    pub ast_tag: Option<AstTag>,
    pub childs: Vec<Box<dyn Ast>>,
}

impl AstTagNode {
    pub fn new_empty() -> Self {
        AstTagNode {childs: Vec::new(), ast_tag: None}
    }
    pub fn new_tag(ast_tag: AstTag) -> Self {
        AstTagNode {childs: Vec::new(), ast_tag: Some(ast_tag)}
    }
    pub fn new_option_tag(ast_tag: Option<AstTag>) -> Self {
        AstTagNode {childs: Vec::new(), ast_tag: ast_tag}
    }
}

impl Ast for AstTagNode {
    fn append(&mut self, ast: Box<dyn Ast>) {
        self.childs.push(ast)    
    }
    fn get_childs(&self) -> &Vec<Box<dyn Ast>> {
        &self.childs
    }
    fn accept(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult {
        match self.ast_tag.as_ref() {
            Some(ast_tag) => {
                match ast_tag {
                    AstTag::Create => {
                        visitor.enter_create()
                    },
                    AstTag::Match => {
                        visitor.enter_match()
                    },
                    AstTag::RelDirectedLR |
                    AstTag::RelDirectedRL |
                    AstTag::RelUndirected => {
                        visitor.enter_relationship(self)
                    },
                    AstTag::Node => {
                        visitor.enter_node()
                    },
                    AstTag::Path => {
                        visitor.enter_path()
                    },
                    AstTag::Property => {
                        visitor.enter_property()
                    },
                    AstTag::Variable => {
                        visitor.enter_variable()
                    },
                    AstTag::Label => {
                        visitor.enter_label()
                    },
                    AstTag::Query => {
                        visitor.enter_query()
                    },
                    AstTag::Return => {
                        visitor.enter_return()
                    },
                    AstTag::Function => {
                        visitor.enter_function()
                    },
                    AstTag::FunctionArg => {
                        visitor.enter_function_arg()
                    },
                    AstTag::Item => {
                        visitor.enter_item()
                    },
                    AstTag::Where => {
                        visitor.enter_where(self)
                    },
                    AstTag::EqualityOperator => visitor.enter_equality_operator(),
                    AstTag::AndOperator => visitor.enter_and_operator(),
                    _ => {
                        Ok(())
                    }
                }
            },
            None => {
                Ok(())
            }
        }
    }
        
    fn accept_exit(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult {
        match self.ast_tag.as_ref() {
            Some(ast_tag) => {
                match ast_tag {
                    AstTag::Create => {
                        visitor.exit_create()
                    },
                    AstTag::Match => {
                        visitor.exit_match()
                    },
                    AstTag::RelDirectedLR |
                    AstTag::RelDirectedRL |
                    AstTag::RelUndirected => {
                        visitor.exit_relationship()
                    },
                    AstTag::Node => {
                        visitor.exit_node()
                    },
                    AstTag::Path => {
                        visitor.exit_path()
                    },
                    AstTag::Property => {
                        visitor.exit_property()
                    },
                    AstTag::Variable => {
                        visitor.exit_variable()
                    },
                    AstTag::Label => {
                        visitor.exit_label()
                    },
                    AstTag::Query => {
                        visitor.exit_query()
                    },
                    AstTag::Return => {
                        visitor.exit_return()
                    },
                    AstTag::Function => {
                        visitor.exit_function()
                    },
                    AstTag::FunctionArg => {
                        visitor.exit_function_arg()
                    },
                    AstTag::Item => {
                        visitor.exit_item()
                    },
                    AstTag::Where => {
                        visitor.exit_where()
                    },
                    AstTag::Parameter => {
                        visitor.exit_parameter()
                    }
                    AstTag::EqualityOperator => visitor.exit_equality_operator(),
                    AstTag::AndOperator => visitor.exit_and_operator(),
                    _ => {
                        Ok(())
                    }
                }
            },
            None => {
                Ok(())
            }
        }
    }
    
    fn clone_ast(&self) -> Box<dyn Ast> {
        let mut root = Box::new(AstTagNode::new_option_tag(self.ast_tag));
        for child in &self.childs {
            root.append(child.clone_ast());
        }
        root
    }
}

impl AstTokenNode {
    pub fn new_token(token_id: usize, token_value: String, token_type: TokenType) -> Self {
        AstTokenNode {token_id: token_id, token_value: token_value, childs: Vec::new(), token_type: token_type}
    }
}

impl Ast for AstTokenNode {
    fn append(&mut self, ast: Box<dyn Ast>) {
        self.childs.push(ast)    
    }
    fn get_childs(&self) -> &Vec<Box<dyn Ast>> {
        &self.childs
    }
    fn accept(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult {
        match self.token_type {
            TokenType::StringType => {
                let sval = self.token_value.get(1..self.token_value.len() -1);
                visitor.enter_string_value(sval)
            },
            TokenType::Float => {
                let res = self.token_value.parse::<f64>().ok();
                visitor.enter_float_value(res)
            },
            TokenType::Integer => {
                let res = self.token_value.parse::<i64>().ok();
                visitor.enter_integer_value(res)
            },
            TokenType::True |
            TokenType::False => {
                let res = self.token_value.parse::<bool>().ok();
                visitor.enter_bool_value(res)
            },
            TokenType::Identifier => visitor.enter_identifier(&self.token_value),
            TokenType::Parameter => visitor.enter_parameter(&self.token_value),
            _ => {
                Ok(())
            }
        }
    }

    fn accept_exit(&self, _visitor: &mut dyn AstVisitor) -> AstVisitorResult {
        Ok(())
    }

    fn clone_ast(&self) -> Box<dyn Ast> {
        let mut root = Box::new(AstTokenNode::new_token(self.token_id, self.token_value.clone(), self.token_type));
        for child in &self.childs {
            root.append(child.clone_ast());
        }
        root
    }
}

impl fmt::Display for AstTokenNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.token_type, self.token_value)
    }
}

impl fmt::Display for AstTagNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ast_tag.as_ref() {
            Some(tag) => {
                write!(f, "{:?}", tag)
            },
            _ => write!(f, "")
        }
        
    }
}