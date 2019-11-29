use super::fsm::{FSM, RunnableFSM};

#[derive(PartialEq, Copy, Clone, Debug)]
enum KeywordState {
        Initial,
        MatchChar(usize),
        MatchKeyword,
}

fn check_equals_ignorecase(c: char, keyword: &'static str, i: usize) -> bool {
    !c.to_lowercase().find(|lc|keyword.chars().nth(i + 1) == Some(*lc)).is_none()
}

pub fn make_keyword_ignorecase_fsm(keyword: &'static str) -> Box<dyn RunnableFSM>  {
    let next_state = move|s, c: char| {
        let mut res = None;
        match s {
               KeywordState::Initial => {
                   if !c.to_lowercase().find(|lc|keyword.chars().nth(0) == Some(*lc)).is_none() {
                       res = Some(KeywordState::MatchChar(0));
                   }
               },
               KeywordState::MatchChar(i) => {
                   if check_equals_ignorecase(c, keyword, i) {
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
        assert_eq!(fsm.run("true"), Some(4));
        assert_eq!(fsm.run("true or false"), Some(4));
        assert_eq!(fsm.run("true or anything"), Some(4));
        assert_eq!(fsm.run("false"), None);
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
        assert_eq!(fsm.run("TRUE"), Some(4));
        assert_eq!(fsm.run("TruA"), None);
    }
}