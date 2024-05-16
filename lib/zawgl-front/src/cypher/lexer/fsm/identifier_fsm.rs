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

use super::fsm_run::{FSM, RunnableFSM};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum IdentifierState {
        Initial,
        MatchIdentifier(usize),
}

fn is_valid_id_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_valid_id_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub fn make_identifier_fsm() -> Box<dyn RunnableFSM<IdentifierState>>  {
    let next_state = move|s, c: char| {
        let mut res = None;
        match s {
               IdentifierState::Initial => {
                   if is_valid_id_start(c) {
                        res = Some(IdentifierState::MatchIdentifier(0));
                   }
               },
               IdentifierState::MatchIdentifier(i) => {
                   if is_valid_id_char(c) {
                        res = Some(IdentifierState::MatchIdentifier(i + 1));
                   }
               },
           };
        
        res
    };

    let accepting_states = |s| -> bool {
            match s {
                IdentifierState::MatchIdentifier(_i) => true,
                _ => false,
            }
        };

    Box::new(FSM::new(IdentifierState::Initial, accepting_states, next_state))
}

#[cfg(test)]
mod test_identifier_fsm {
    use super::*;
    #[test]
    fn test_identifier() {
        let mut fsm = make_identifier_fsm();
        assert_eq!(fsm.run("blabla:"), Some((6, IdentifierState::MatchIdentifier(5))));
    }
}