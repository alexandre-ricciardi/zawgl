use one_graph_core::model::{Node, PropertyGraph, PropertyValue, Status};

use super::gremlin_state::StateContext;

use one_graph_gremlin::gremlin::*;


pub fn init_pattern(context: &mut StateContext, n: Node) {
    let mut pattern = PropertyGraph::new();
    let nid = pattern.add_node(n);
    context.patterns.push(pattern);
    context.node_index = Some(nid);
}

pub fn prop_value_from_gremlin_value(gval: &GValue) -> PropertyValue {
    match gval {
        GValue::String(sval) => {
            PropertyValue::PString(sval.clone())
        }
        GValue::Bool(bval) => {
            PropertyValue::PBool(*bval)
        }
        GValue::Integer(ival) => {
            match ival {
                GInteger::I32(ivalue) => {
                    PropertyValue::PInteger(ivalue.0 as i64)
                },
                GInteger::I64(ivalue) => {
                    PropertyValue::PInteger(ivalue.0)
                }
            }
        }
        GValue::Double(dval) => {
            PropertyValue::PFloat(dval.0)
        }
    }
}

pub enum Scenario {
    CreateOnly,
    MatchAndCreate,
    MatchOnly,
    Unknown,
}

pub fn get_request_scenario(pattern: &PropertyGraph) -> Scenario {
    let mut contains_match = false;
    let mut contains_create = false;
    for n in pattern.get_nodes() {
        match n.get_status() {
            Status::Create => {contains_create = true;}
            Status::Match => {contains_match = true;}
            _ => {}
        } 
    }
    for r in pattern.get_relationships() {
        match r.get_status() {
            Status::Create => {contains_create = true;}
            Status::Match => {contains_match = true;}
            _ => {}
        } 
    }
    if contains_match {
        if contains_create {
            Scenario::MatchAndCreate
        } else {
            Scenario::MatchOnly
        }
    } else {
        if contains_create {
            Scenario::CreateOnly
        } else {
            Scenario::Unknown
        }
    }
}

pub fn convert_graph_to_gremlin_response(graphs: &Vec<PropertyGraph>, request_id: &str) -> Option<GremlinResponse> {
    let mut res = GResult::new();
    for graph in graphs {
        for n in graph.get_nodes() {
            let label = n.get_labels_ref().join(":");
            let id = GValue::Integer(GInteger::I64(GInt64(n.get_id()? as i64)));
            let vertex = GVertex{id: id, label: label};
            let traverser = GTraverser{bulk: GInt64(1), value: GItem::Vertex(vertex)};
            res.data.values.push(traverser);
        }
    }
    
    let attrs = GMap::new();
    Some(GremlinResponse{request_id: String::from(request_id), status: GStatus{message: String::from(""), code: 200, attributes: attrs}, result: res})
}