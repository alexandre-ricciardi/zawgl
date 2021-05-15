use super::{State, StateContext, match_vertex_state::MatchVertexState};
use one_graph_core::model::*;
use one_graph_gremlin::gremlin::*;
use super::gremlin_state::*;

pub struct MatchState {
    bytecodes: Vec<Vec<GStep>>,
}

impl MatchState {
    pub fn new(bytecodes: &Vec<Vec<GStep>>) -> Self {
        MatchState{bytecodes: bytecodes.clone()}
    }
}
impl State for MatchState {
    
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        let mut gremlin_state = GremlinStateMachine::new();
        for bytecode in &self.bytecodes {
            for step in bytecode {
                gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, step).ok_or(StateError::Invalid)?;
            }
            gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, &GStep::Empty).ok_or(StateError::Invalid)?;
        }
        context.match_states_contexts.push(gremlin_state.context);

        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}
