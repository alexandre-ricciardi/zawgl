use super::fsm::{FSM, RunnableFSM};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum NumberState {
        Initial,
        Integer,
        BeginNumberWithFractionalPart,
        NumberWithFractionalPart,
        BeginNumberWithExponent,
        BeginNumberWithSignedExponent,
        NumberWithExponent,
        NoNextState
}

pub fn make_number_fsm() -> Box<dyn RunnableFSM<NumberState>>  {
    let next_state = |s, c: char| {
            let mut res = None;
            match s {
                NumberState::Initial => {
                    if c.is_numeric() {
                        res = Some(NumberState::Integer);
                    }
                },
                NumberState::Integer => {
                    if c.is_numeric() {
                        res = Some(NumberState::Integer);
                    } else if c == '.' {
                        res = Some(NumberState::BeginNumberWithFractionalPart);
                    } else if c.to_lowercase().to_string() == "e" {
                        res = Some(NumberState::BeginNumberWithExponent);
                    }
                },
                NumberState::BeginNumberWithFractionalPart => {
                    if c.is_numeric() {
                        res = Some(NumberState::NumberWithFractionalPart);
                    }
                },
                NumberState::NumberWithFractionalPart => {
                    if c.is_numeric() {
                       res =  Some(NumberState::NumberWithFractionalPart);
                    } else if c.to_lowercase().to_string() == "e" {
                       res =  Some(NumberState::BeginNumberWithExponent);
                    }
                },
                NumberState::BeginNumberWithExponent => {
                    if c == '+' || c == '-' {
                        res = Some(NumberState::BeginNumberWithSignedExponent);
                    } else if c.is_numeric() {
                        res = Some(NumberState::NumberWithExponent);
                    }
                },
                NumberState::BeginNumberWithSignedExponent => {
                    if c.is_numeric() {
                        res = Some(NumberState::NumberWithExponent);
                    }
                },
                NumberState::NumberWithExponent => {
                    if c.is_numeric() {
                        res = Some(NumberState::NumberWithExponent);
                    }
                }
                _ => {},
            }
            res
         };

        Box::new(FSM::new(NumberState::Initial,
        |s| {
            match s {
                NumberState::Integer => true,
                NumberState::NumberWithFractionalPart => true,
                NumberState::NumberWithExponent => true,
                _ => false,
            }
        },
        next_state))
}


#[cfg(test)]
mod test_fsm {
    use super::*;
    #[test]
    fn test_numbers() {
        let mut fsm = make_number_fsm();
        assert_eq!(fsm.run("12.03"), Some((5, NumberState::NumberWithFractionalPart)));
        assert_eq!(fsm.run("12.4e-03:"), Some((8, NumberState::NumberWithExponent)));
        assert_eq!(fsm.run("121111.02223"), Some((12, NumberState::NumberWithFractionalPart)));
        assert_eq!(fsm.run("12.4333E03"), Some((10, NumberState::NumberWithExponent)));
        assert_eq!(fsm.run("12.4333E03XXXX"), Some((10, NumberState::NumberWithExponent)));
    }
}