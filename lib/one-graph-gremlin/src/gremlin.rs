pub enum Step {
    AddV(String),
    V(Option<GValue>),
    Has(String, Predicate),
    AddE(String),
    As(String),
    From(String),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum GValue {
    Integer(GInteger),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum GInteger {
    I64(i64),
    I32(i64),
}

#[derive(Debug, PartialEq)]
pub struct GList<T> {
    pub values: Vec<T>,
}

pub struct GEdge {
    pub id: i64,
    pub label: String,
    pub in_v_label: String,
    pub out_v_abel: String,
    pub in_v: i64,
    pub out_v: i64,
}

#[derive(Debug, PartialEq)]
pub enum Predicate {
    Value(GValue),
    Within(GList<GValue>),
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

pub struct GTraverser {
    bulk: i64,
    value: GItem,
}

pub struct GVertex {
    id: i64,
    label: String,
}

pub enum GItem {
    Vertex(GVertex),
    Edge(GEdge),
}