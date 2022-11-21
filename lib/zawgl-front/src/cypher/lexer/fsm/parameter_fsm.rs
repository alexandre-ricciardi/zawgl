// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::fsm::{FSM, RunnableFSM};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ParameterState {
        Initial,
        MatchParameter(usize),
}

fn is_valid_id_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}


fn is_valid_parameter_prefix(c: char) -> bool {
    c == '$'
}

pub fn make_parameter_fsm() -> Box<dyn RunnableFSM<ParameterState>>  {
    let next_state = move|s, c: char| {
        let mut res = None;
        match s {
            ParameterState::Initial => {
                if is_valid_parameter_prefix(c) {
                    res = Some(ParameterState::MatchParameter(0));
                }
            },
            ParameterState::MatchParameter(i) => {
                if is_valid_id_char(c) {
                    res = Some(ParameterState::MatchParameter(i + 1));
                }
            },
           };
        
        res
    };

    let accepting_states = |s| -> bool {
           let res = match s {
            ParameterState::MatchParameter(_i) => true,
                _ => false,
            };
            res
        };

    Box::new(FSM::new(ParameterState::Initial, accepting_states, next_state))
}

#[cfg(test)]
mod test_identifier_fsm {
    use super::*;
    #[test]
    fn test_identifier() {
        let mut fsm = make_parameter_fsm();
        assert_eq!(fsm.run("$blabla:"), Some((7, ParameterState::MatchParameter(6))));
    }
}