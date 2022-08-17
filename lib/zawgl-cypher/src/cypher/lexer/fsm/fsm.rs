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

