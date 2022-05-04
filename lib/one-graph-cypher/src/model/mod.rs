use super::cypher::parser::Ast;
use one_graph_core::model::PropertyGraph;
use one_graph_query_planner::QueryStep;


pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<String>,
}

impl FunctionCall {
    pub fn new(name: &str) -> Self {
        FunctionCall{name: String::from(name), args: Vec::new()}
    }
}

pub enum ReturnExpression {
    FunctionCall(FunctionCall),
    Item(String),
}

pub struct ReturnClause {
    pub expressions: Vec<ReturnExpression>,
}

impl ReturnClause {
    pub fn new() -> Self {
        ReturnClause{expressions: Vec::new()}
    }
}

pub struct WhereClause {
    pub expressions: Box<dyn Ast>,
}

impl WhereClause {
    pub fn new(ast: Box<dyn Ast>) -> Self {
        WhereClause{expressions: ast}
    }
}

pub enum Operator {
    Equal,
    Inferior,
    Superior,
    InferiorOrEqual,
    SuperiorOrEqual,
}
pub struct BoolCondition {
    pub first_member: Box<dyn Ast>,
    pub second_member: Box<dyn Ast>,
    pub operator: Operator,
}

pub struct Request {
    pub steps: Vec<QueryStep>,
    pub return_clause: Option<ReturnClause>,
    pub where_clause: Option<WhereClause>,
}

impl Request {
    pub fn new() -> Self {
        Request {steps: Vec::new(), return_clause: None, where_clause: None}
    }
}