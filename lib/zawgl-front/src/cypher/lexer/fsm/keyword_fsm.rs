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
pub enum KeywordState {
        Initial,
        MatchChar(usize),
        MatchKeyword,
}

fn check_equals_ignorecase(c: char, keyword: &'static str, i: usize) -> bool {
    keyword.chars().nth(i) == Some(c) ||
    !c.to_lowercase().find(|lc|keyword.chars().nth(i) == Some(*lc)).is_none()
}

pub fn make_keyword_ignorecase_fsm(keyword: &'static str) -> Box<dyn RunnableFSM<KeywordState>>  {
    let next_state = move|s, c: char| {
        let mut res = None;
        match s {
                KeywordState::Initial => {
                    if check_equals_ignorecase(c, keyword, 0) {
                        if keyword.len() == 1 {
                            res = Some(KeywordState::MatchKeyword)
                        } else {
                            res = Some(KeywordState::MatchChar(0));
                        }
                       
                   }
               },
               KeywordState::MatchChar(i) => {
                   if check_equals_ignorecase(c, keyword, i + 1) {
                        if i + 2 == keyword.len() {
                            res = Some(KeywordState::MatchKeyword);
                        } else {
                            res = Some(KeywordState::MatchChar(i + 1));
                        }
                   }
               },
               KeywordState::MatchKeyword => {
               },
           };
        
        res
    };

    let accepting_states = |s: KeywordState| -> bool {
        let mut res = false;
            match s {
                KeywordState::MatchKeyword => {res = true;},
                _ => {},
            }
            res
        };

    Box::new(FSM::new(KeywordState::Initial, accepting_states, next_state))
}

#[cfg(test)]
mod test_keywords_fsm {
    use super::*;
    #[test]
    fn test_keywords() {
        let mut fsm = make_keyword_ignorecase_fsm("true");
        assert_eq!(fsm.run("true"), Some((4, KeywordState::MatchKeyword)));
        assert_eq!(fsm.run("true or false"), Some((4, KeywordState::MatchKeyword)));
        assert_eq!(fsm.run("true or anything"), Some((4, KeywordState::MatchKeyword)));
        assert_eq!(fsm.run("false"), None);
        
    }
    #[test]
    fn test_par() {
        assert_eq!(make_keyword_ignorecase_fsm("(").run("("), Some((1, KeywordState::MatchKeyword)));
    }
    #[test]
    fn test_error_1() {
        let mut fsm = make_keyword_ignorecase_fsm("false");
        assert_eq!(fsm.run("or false"), None);
    }
    #[test]
    fn test_errors() {
        let mut fsm = make_keyword_ignorecase_fsm("trues");
        assert_eq!(fsm.run("true"), None);
        assert_eq!(fsm.run("false"), None);
    }
    #[test]
    fn test_ignore_case() {
        let mut fsm = make_keyword_ignorecase_fsm("true");
        assert_eq!(fsm.run("TRUE"), Some((4, KeywordState::MatchKeyword)));
        assert_eq!(fsm.run("TruA"), None);
    }
}