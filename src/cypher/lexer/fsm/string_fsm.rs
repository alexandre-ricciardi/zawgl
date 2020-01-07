use super::fsm::{FSM, RunnableFSM};

#[derive(PartialEq, Copy, Clone, Debug)]
enum StringState {
        Initial,
        MatchBeginSimpleQuote(usize),
        MatchString(usize),
        MatchEndSimpleQuote(usize),
}

fn is_valid_string_char(c: char) -> bool {
    c.is_alphanumeric() || c.is_whitespace()
}

pub fn make_string_fsm() -> Box<dyn RunnableFSM>  {
    let next_state = move|s, c: char| {
        let mut res = None;
        match s {
                StringState::Initial => {
                   if c == '\'' {
                        res = Some(StringState::MatchBeginSimpleQuote(0));
                   }
                },
                StringState::MatchBeginSimpleQuote(i) => {
                    if is_valid_string_char(c) {
                        res = Some(StringState::MatchString(i + 1));
                    }
                },
                StringState::MatchString(i) => {
                   if is_valid_string_char(c) {
                        res = Some(StringState::MatchString(i + 1));
                   } else if c == '\'' {
                       res = Some(StringState::MatchEndSimpleQuote(i + 1));
                   }
                },
                _ => {},
           };
        
        res
    };

    let accepting_states = |s| -> bool {
           let res = match s {
                StringState::MatchEndSimpleQuote(_i) => true,
                _ => false,
            };
            res
        };

    Box::new(FSM::new(StringState::Initial, accepting_states, next_state))
}

#[cfg(test)]
mod test_identifier_fsm {
    use super::*;
    #[test]
    fn test_string_fsm() {
        let mut fsm = make_string_fsm();
        assert_eq!(fsm.run("'blabla' test"), Some(8));
    }
    #[test]
    fn test_string_ws_fsm() {
        let mut fsm = make_string_fsm();
        assert_eq!(fsm.run("'blab la' test"), Some(9));
    }
}