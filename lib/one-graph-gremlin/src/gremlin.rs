use std::collections::HashMap;

use serde_json::Value;
use serde_json::json;

pub trait ToJson {
    fn to_json(&self) -> serde_json::Value;
}

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
    I64(GInt64),
    I32(GI32),
}

#[derive(Debug, PartialEq)]
pub struct GInt64(i64);

impl GInt64 {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "@type": "g:Int64",
            "@value": self.0,
        })
    }
}
#[derive(Debug, PartialEq)]
pub struct GI32(i64);



#[derive(Debug, PartialEq)]
pub struct GList<T: ToJson> {
    pub values: Vec<T>,
}

impl <T: ToJson> ToJson for GList<T> {

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
    result: GResult,
}

impl ToJson for GremlinResponse {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "requestId": self.request_id,
            "status": self.status.to_json(),
            "result": self.result.to_json(),
        })
    }
}

pub struct Status {
    pub message: String,
    pub code: i32,
    attributes: GMap,
}

impl Status {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "message": self.message,
            "code": self.code,
            "attributes": self.attributes.to_json(),
        })
    }
}

pub struct GTraverser {
    bulk: GInt64,
    value: GItem,
}

impl ToJson for GTraverser {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "bulk": self.bulk.to_json(),
            "value": self.value.to_json(),
        })
    }
}

pub struct GVertex {
    id: GInt64,
    label: String,
}

pub enum GItem {
    Vertex(GVertex),
    Edge(GEdge),
}

impl ToJson for GItem {
    fn to_json(&self) -> serde_json::Value {
        match self {
            GItem::Vertex(v) => {
                v.to_json()
            },
            GItem::Edge(e) => {
                e.to_json()
            }
        }
    }
}

impl GVertex {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "@type": "g:Vertex",
            "@value": {
                "id": self.id.to_json(),
                "label": self.label,
            }
        })
    }
}

pub struct GMap {
    map: HashMap<String, String>,
}

impl GMap {
    fn to_json(&self) -> serde_json::Value {
        let mut res = Vec::new();
        for e in self.map {
            res.push(e.0);
            res.push(e.1);
        }
        json!(res)
    }
}

pub struct GResult {
    data: GList<GTraverser>,
    meta: GMap,
}

impl GResult {
    fn to_json(&self) -> serde_json::Value {
    
    }
}