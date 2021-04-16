use one_graph_gremlin::gremlin::*;
use one_graph_core::model::*;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum StateError {
    Invalid
}

pub trait State {
    fn handle_match_vertex(&mut self, vid: Option<u64>);
    fn handle_add_vertex(&mut self, label: &str);
    fn handle_add_edge(&mut self, label: &str);
    fn handle_alias(&mut self, name: &str);
}

struct MatchVertexState {
    vid: Option<u64>,
}

impl MatchVertexState {
    fn new(vid: Option<u64>) -> Self {
        MatchVertexState{vid: vid}
    }
}
impl State for MatchVertexState {
    
    fn handle_match_vertex(&mut self, vid: Option<u64>) {

    }
    fn handle_add_vertex(&mut self, label: &str) {

    }
    fn handle_add_edge(&mut self, label: &str) {

    }
    fn handle_alias(&mut self, name: &str) {

    }
}

pub struct StateContext {
    state: Option<Box<dyn State>>,
}

impl StateContext {
    pub fn new() -> Self {
        StateContext{state: None}
    }

    pub fn match_vertex(&mut self, vid: Option<GValue>) {
        let id = vid.and_then(|value| u64::try_from(value).ok());
        if let Some(s) = &mut self.state {
            s.handle_match_vertex(id);
        } else {
            self.state = Some(Box::new(MatchVertexState::new(id)));
        }
    }
}