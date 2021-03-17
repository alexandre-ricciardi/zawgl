pub enum Step {
    AddV(String),
    V(Option<GValue>),
    Has(String, Predicate),
    AddE(String),
    As(String),
    From(String),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GValue {
    Integer(GInteger),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GInteger {
    I64(i64),
    I32(i32),
}

#[derive(Debug, PartialEq)]
pub struct GList {
    pub values: Vec<GValue>,
}

pub struct GEdge {
    pub id: GValue,
    pub label: String,
    pub in_v_label: String,
    pub out_v_abel: String,
    pub in_v: GValue,
    pub out_v: GValue,
}

#[derive(Debug, PartialEq)]
pub enum Predicate {
    Value(GValue),
    Within(GList),
}

pub struct GremlinRequest {
    pub request_id: String,
    pub steps: Vec<Step>,
}

pub struct GremlinResponse {
    pub request_id: String,
    pub status: Status,
}

pub struct Status {
    pub message: String,
    pub code: i32,
}