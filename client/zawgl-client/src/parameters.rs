use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Parameters(Parameters),
}

pub type Parameters = HashMap<String, Value>;

