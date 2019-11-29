use super::fsm::{FSM, RunnableFSM};

#[derive(PartialEq, Copy, Clone, Debug)]
enum IdentifierState {
        Initial,
        MatchIdentifier(usize),
}

pub fn make_identifier_fsm() -> Box<dyn RunnableFSM>  {
    let next_state = move|s, c: char| {
        let mut res = None;
        match s {
               IdentifierState::Initial => {
                   if c.is_alphabetic() {
                        res = Some(IdentifierState::MatchIdentifier(0));
                   }
               },
               IdentifierState::MatchIdentifier(i) => {
                   if c.is_alphabetic() {
                        res = Some(IdentifierState::MatchIdentifier(i + 1));
                   }
               },
           };
        
        res
    };

    let accepting_states = |s| -> bool {
           let res = match s {
                IdentifierState::MatchIdentifier(_i) => true,
                _ => false,
            };
            res
        };

    Box::new(FSM::new(IdentifierState::Initial, accepting_states, next_state))
}

#[cfg(test)]
mod test_identifier_fsm {
    use super::*;
    #[test]
    fn test_identifier() {
        let mut fsm = make_identifier_fsm();
        assert_eq!(fsm.run("blabla:"), Some(6));
    }
}