use crate::contestant::Contestant;
use crate::match_::{Match, MatchState};
use crate::match_contender::MatchContender;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Winner {
    match_: Rc<RefCell<Match>>,
}

impl Winner {
    #[allow(dead_code)]
    pub fn new(match_: &Rc<RefCell<Match>>) -> Self {
        Self {
            match_: match_.clone(),
        }
    }
}

impl MatchContender for Winner {
    fn contestant(&self) -> Option<Rc<Contestant>> {
        match self.match_.borrow().state() {
            MatchState::Won(winner) => Some(winner),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contestant;
    use crate::match_::Contenders;
    use crate::match_contender::new_contestant::NewContestant;

    #[test]
    fn no_winner() {
        let mut factory = contestant::Factory::default();

        let contestants: Contenders = [
            Box::new(NewContestant::new(
                factory.create_contestant("Nathan".to_string()),
            )),
            Box::new(NewContestant::new(
                factory.create_contestant("Not Nathan".to_string()),
            )),
        ];

        let match_ = Rc::new(RefCell::new(Match::new(0, contestants)));
        let winner = Winner::new(&match_);

        assert!(winner.contestant().is_none());
    }

    #[test]
    fn winner() {
        let contestants: Contenders = [
            Box::new(NewContestant::new(Contestant::new(0, "Nathan".to_string()))),
            Box::new(NewContestant::new(Contestant::new(
                1,
                "Not Nathan".to_string(),
            ))),
        ];

        let match_ = Rc::new(RefCell::new(Match::new(0, contestants)));
        let winner = Winner::new(&match_);

        match_.borrow_mut().set_winner(&0).unwrap();

        assert_eq!(winner.contestant().unwrap().id(), &0);
    }
}
