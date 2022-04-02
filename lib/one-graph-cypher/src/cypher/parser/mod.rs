pub mod error;
pub mod parser_utils;
mod pattern_parser_delegate;
mod properties_parser_delegate;
mod common_parser_delegate;
mod return_clause_parser_delegate;
mod where_clause_parser_delegate;
pub mod cypher_parser;

use super::lexer::*;
use self::error::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstTag  {
    Create,
    Match,
    Node,
    Pattern,
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
}

pub trait AstVisitor {
    fn enter_create(&mut self, node: &AstTagNode) -> AstVisitorResult<bool>;
    fn enter_match(&mut self, node: &AstTagNode) -> AstVisitorResult<bool>;
    fn enter_pattern(&mut self, node: &AstTagNode) -> AstVisitorResult<bool>;
    fn enter_node(&mut self, node: &AstTagNode) -> AstVisitorResult<bool>;
    fn enter_relationship(&mut self, node: &AstTagNode) -> AstVisitorResult<bool>;
    fn enter_property(&mut self, node: &AstTagNode) -> AstVisitorResult<bool>;
    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult<bool>;
    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult<bool>;
    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult<bool>;
    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult<bool>;
    fn enter_identifier(&mut self, key: &str) -> AstVisitorResult<bool>;
    fn enter_variable(&mut self) -> AstVisitorResult<bool>;
    fn enter_label(&mut self) -> AstVisitorResult<bool>;
    fn enter_query(&mut self) -> AstVisitorResult<bool>;
    fn enter_return(&mut self) -> AstVisitorResult<bool>;
    fn enter_function(&mut self) -> AstVisitorResult<bool>;
    fn enter_function_arg(&mut self) -> AstVisitorResult<bool>;
    fn enter_item(&mut self) -> AstVisitorResult<bool>;
    fn enter_where(&mut self, node: &AstTagNode) -> AstVisitorResult<bool>;
}

#[derive(Debug, Clone)]
pub enum AstVisitorError {
    SyntaxError,
}

pub type AstVisitorResult<T> = std::result::Result<T, AstVisitorError>;

pub trait Ast : fmt::Display {
    fn append(&mut self, ast: Box<dyn Ast>);
    fn accept(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult<bool>;
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
    fn accept(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult<bool> {
        match self.ast_tag.as_ref() {
            Some(ast_tag) => {
                match ast_tag {
                    AstTag::Create => {
                        visitor.enter_create(self)
                    },
                    AstTag::Match => {
                        visitor.enter_match(self)
                    },
                    AstTag::RelDirectedLR |
                    AstTag::RelDirectedRL |
                    AstTag::RelUndirected => {
                        visitor.enter_relationship(self)
                    },
                    AstTag::Node => {
                        visitor.enter_node(self)
                    },
                    AstTag::Pattern => {
                        visitor.enter_pattern(self)
                    },
                    AstTag::Property => {
                        visitor.enter_property(self)
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
                    _ => {
                        Ok(true)
                    }
                }
            },
            None => {
                Ok(true)
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
    fn accept(&self, visitor: &mut dyn AstVisitor) -> AstVisitorResult<bool> {
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
            _ => {
                Ok(true)
            }
        }
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

pub fn walk_ast(visitor: &mut dyn AstVisitor, ast: &Box<dyn Ast>) -> AstVisitorResult<()>  {
    let res = ast.accept(visitor)?;
    if res {
        for child in ast.get_childs() {
            walk_ast(visitor, &child)?;
        }
    }
    Ok(())
}

pub struct Parser<'a>  {
    tokens: Vec<Token<'a>>,
    pub index: usize,
}

impl <'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {tokens : tokens, index: 0}
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    } 

    pub fn require(&mut self, token_type: TokenType) -> ParserResult<usize> {
        if !self.check(token_type) {
            return Err(ParserError::SyntaxError(self.index));
        }
        self.advance();
        Ok(self.index)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn has_next(&self) -> bool {
        self.index + 1 < self.tokens.len()
    }

    pub fn current_token_type_advance(&mut self, token_type: TokenType) -> bool {
        if self.tokens.len() > self.index && self.tokens[self.index].token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn get_current_token_type(&self) -> TokenType {
        self.tokens[self.index].token_type
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index && self.tokens[self.index].token_type == token_type
    }

    pub fn next_token_type(& self, token_type: TokenType) -> bool {
        self.tokens.len() > self.index + 1 && self.tokens[self.index + 1].token_type == token_type
    }

}


fn make_ast_token(parser: &Parser) -> Box<AstTokenNode> {
    let token_id = parser.index - 1;
    let token = &parser.get_tokens()[token_id];
    Box::new(AstTokenNode::new_token(token_id, token.content.to_owned(), token.token_type ))
}

fn make_ast_tag(tag: AstTag) -> Box<AstTagNode> {
    Box::new(AstTagNode::new_tag(tag))
}

#[cfg(test)]
mod test_parser {
    use super::*;

    fn run(qry: &str) {
        let mut lexer = Lexer::new(qry);
        match lexer.get_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let root = cypher_parser::parse(&mut parser);
                parser_utils::print_node(&root.unwrap(), parser.get_tokens(), 0);
            },
            Err(value) => assert!(false, "lexer error: {}", value)
        }
    }

    #[test]
    fn test_create() {
        run("CREATE (n:Person) RETURN id(n, r, z)");
    }
    #[test]
    fn test_create_labels() {
        run("CREATE (n:Person:Friend:Etc)");
    }

    #[test]
    fn test_create_graph() {
        run("CREATE (n:Person)-[r:FRIEND_OF]->(m:Person) RETURN n, r, m");
    }

    #[test]
    fn test_properties_node() {
        run("CREATE (n:Person { name: 'hello', value: 'world' })");
    }
    #[test]
    fn test_properties_node_1() {
        run("CREATE (n:Person:Parent {test: 'Hello', case: 4.99})");
    }
    

    #[test]
    fn test_where_clause_1() {
        run("CREATE (n:Person:Parent {test: 'Hello', case: 4.99}) WHERE id(n) = 112 AND n.test = 'hello' OR n.case = 123.9 RETURN n, id(n)");
    }

    #[test]
    fn test_match_then_create() {
        run("match (p:Person), (m:Movie) create (m)<-[r:Played]-(p) return m, r, p");
    }
}

