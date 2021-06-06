use one_graph_core::{graph::{EdgeIndex, NodeIndex, traits::{GraphContainerTrait, GraphTrait}}, model::{Node, PropertyGraph, PropertyValue, Status}};

use super::gremlin::gremlin_state::StateContext;

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

fn build_vertex_from_node(n: &Node) -> Option<GVertex> {
    let label = n.get_labels_ref().join(":");
    let id = GValue::Integer(GInteger::I64(GInt64(n.get_id()? as i64)));
    Some(GVertex{id: id, label: label})
}

pub fn convert_graph_to_gremlin_response(graphs: &Vec<PropertyGraph>, request_id: &str) -> Option<GremlinResponse> {
    let mut res = GResult::new();
    for graph in graphs {
        for n in graph.get_nodes() {
            let vertex = build_vertex_from_node(n)?;
            let traverser = GTraverser{bulk: GInt64(1), value: GItem::Vertex(vertex)};
            res.data.values.push(traverser);
        }

        let mut r_index = 0;
        for r in graph.get_relationships() {
            let edge_index = EdgeIndex::new(r_index);
            let s_index = graph.get_source_index(&edge_index);
            let t_index = graph.get_target_index(&edge_index);
            let label = r.get_labels_ref().join(":");
            let id = GInt64(r.get_id()? as i64);
            let source = graph.get_node_ref(&s_index);
            let target = graph.get_node_ref(&t_index);
            let edge = GEdge{id: id, label: label, 
                out_v_abel: target.get_labels_ref().join(":"),
                in_v_label: source.get_labels_ref().join(":"),
                in_v: GInt64(source.get_id()? as i64),
                out_v: GInt64(target.get_id()? as i64),
            };
            let traverser = GTraverser{bulk: GInt64(1), value: GItem::Edge(edge)};
            res.data.values.push(traverser);
            r_index += 1;
        }
    }
    
    let attrs = GMap::new();
    Some(GremlinResponse{request_id: String::from(request_id), status: GStatus{message: String::from(""), code: 200, attributes: attrs}, result: res})
}