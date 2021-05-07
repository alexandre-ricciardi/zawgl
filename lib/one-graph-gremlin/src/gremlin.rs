use std::collections::HashMap;
use std::convert::TryFrom;
use serde_json::Value;
use serde_json::json;

pub trait ToJson {
    fn to_json(&self) -> serde_json::Value;
}

pub enum GStep {
    AddV(String),
    V(Option<GValue>),
    Has(String, Predicate),
    AddE(String),
    E(Option<GValue>),
    OutE(Vec<String>),
    As(String),
    From(String),
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GValue {
    Integer(GInteger),
    String(String),
    Bool(bool),
}

impl ToJson for GValue {
    fn to_json(&self) -> serde_json::Value {
        match self {
            GValue::Integer(v) => {
                v.to_json()
            },
            GValue::String(v) => {
                json!(v)
            },
            GValue::Bool(v) => {
                json!(v)
            }
        }
    }
}

impl TryFrom<GValue> for u64 {
    type Error = &'static str;

    fn try_from(value: GValue) -> Result<Self, Self::Error> {
        match &value {
            GValue::Integer(v) => {
                match v {
                    GInteger::I32(ivalue) => {
                        Ok(ivalue.0 as u64)
                    },
                    GInteger::I64(ivalue) => {
                        u64::try_from(ivalue.0).map_err(|_| "Error casting to unsigned integer")
                    }
                }
            },
            GValue::String(sv) => {
                sv.parse().map_err(|_| "Error parsing integer")
            }
            _ => {
                Err("Cannot parse GValue as Integer")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GInteger {
    I64(GInt64),
    I32(GInt32),
}

impl ToJson for GInteger {
    fn to_json(&self) -> serde_json::Value {
        match self {
            GInteger::I64(v) => {
                v.to_json()
            },
            GInteger::I32(v) => {
                v.to_json()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GInt64(pub i64);

impl GInt64 {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "@type": "g:Int64",
            "@value": self.0,
        })
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GInt32(pub i32);

impl GInt32 {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "@type": "g:Int32",
            "@value": self.0,
        })
    }
}


#[derive(Debug, PartialEq)]
pub struct GList<T: ToJson> {
    pub values: Vec<T>,
}

impl <T: ToJson> ToJson for GList<T> {
    fn to_json(&self) -> serde_json::Value {
        let mut array = Vec::new();
        for e in &self.values {
            array.push(e.to_json());
        }
        json!(array)
    }
}

pub struct GEdge {
    pub id: GInt64,
    pub label: String,
    pub in_v_label: String,
    pub out_v_abel: String,
    pub in_v: GInt64,
    pub out_v: GInt64,
}

impl ToJson for GEdge {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "id": self.id.to_json(),
            "label": self.label,
            "inVLabel": self.in_v_label,
            "outVLabel": self.out_v_abel,
            "inV": self.in_v.to_json(),
            "outV": self.out_v.to_json()
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Predicate {
    Value(GValue),
    Within(GList<GValue>),
}

pub struct GremlinRequest {
    pub request_id: String,
    pub steps: Vec<GStep>,
}

pub struct GremlinResponse {
    pub request_id: String,
    pub status: GStatus,
    pub result: GResult,
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

pub struct GStatus {
    pub message: String,
    pub code: i32,
    pub attributes: GMap,
}

impl GStatus {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "message": self.message,
            "code": self.code,
            "attributes": self.attributes.to_json(),
        })
    }
}

pub struct GTraverser {
    pub bulk: GInt64,
    pub value: GItem,
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
    pub id: GInt64,
    pub label: String,
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
    pub map: HashMap<String, String>,
}

impl GMap {
    fn to_json(&self) -> serde_json::Value {
        let mut res = Vec::new();
        for e in &self.map {
            res.push(e.0);
            res.push(e.1);
        }
        json!(res)
    }
}

pub struct GResult {
    pub data: GList<GTraverser>,
    pub meta: GMap,
}

impl GResult {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "data": self.data.to_json(),
            "meta": self.meta.to_json()
        })
    }
}