use crate::contestant::Contestant;
use crate::match_::{MatchState, RcMatch};
use crate::match_contender::MatchContender;
use std::rc::Rc;

pub struct Winner {
    match_: RcMatch,
}

impl Winner {
    #[allow(dead_code)]
    pub fn new(match_: &RcMatch) -> Self {
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
    use crate::match_::Match;
    use crate::match_contender::tests::dummy_contenders;
    use std::cell::RefCell;

    #[test]
    fn no_winner() {
        let contestants = dummy_contenders();
        let match_ = Rc::new(RefCell::new(Match::new(0, contestants)));
        let winner = Winner::new(&match_);

        assert!(winner.contestant().is_none());
    }

    #[test]
    fn winner() {
        let contestants = dummy_contenders();
        let match_ = Rc::new(RefCell::new(Match::new(0, contestants)));
        let winner = Winner::new(&match_);

        match_.borrow_mut().set_winner(&0).unwrap();

        assert_eq!(winner.contestant().unwrap().id(), &0);
    }
}
