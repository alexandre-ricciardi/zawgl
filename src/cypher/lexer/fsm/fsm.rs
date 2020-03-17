pub struct FSM<S, NS, AC> where S: PartialEq + Copy + Clone, NS: Fn(S, char) -> Option<S>, AC: Fn(S) -> bool {
    initial_state: S,
    accepting_states: AC,
    next_state: NS,
}

pub trait RunnableFSM<S>  {
    fn run(&mut self, input: & str) -> Option<(usize, S)>;
}

impl <S, NS, AC> FSM<S, NS, AC> where S: PartialEq + Copy + Clone, NS: Fn(S, char) -> Option<S>, AC: Fn(S) -> bool {

    pub fn new(initial_state: S, accepting_states: AC, next_state: NS) -> FSM<S, NS, AC> {
        FSM {initial_state: initial_state, accepting_states: accepting_states, next_state: next_state}
    }
}

impl <S, NS, AC> RunnableFSM<S> for  FSM<S, NS, AC> where S: PartialEq + Copy + Clone, NS: Fn(S, char) -> Option<S>, AC: Fn(S) -> bool {

    fn run(&mut self, input: & str) -> Option<(usize, S)> {
        let mut current_state = self.initial_state;
        let mut position = 0;
        for (i, c) in input.chars().enumerate() {
            position = i;
            match (self.next_state)(current_state, c) {
                Some(next_state) => {
                    current_state = next_state;
                },
                None => {
                    if (self.accepting_states)(current_state) {
                        return Some((position, current_state));
                    } else {
                        return None;
                    }
                },
            };
        }
        if (self.accepting_states)(current_state) {
            return Some((position + 1, current_state));
        }
        return None;
    }
}

