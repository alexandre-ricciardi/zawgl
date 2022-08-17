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