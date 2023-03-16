pub mod current_match;
mod iterator;
pub mod match_ref;

use crate::bracket::current_match::CurrentMatch;
use crate::bracket::iterator::BracketIterator;
use crate::bracket::match_ref::MatchRef;
use crate::contestant::Contestant;
use std::cell::RefCell;
use std::rc::Rc;

use crate::match_::{Match, MatchState, SetWinnerInvalid};

pub type MatchId = usize;

#[derive(Default)]
pub struct Bracket {
    matches: Vec<Rc<RefCell<Match>>>,
}

impl Bracket {
    pub fn len(&self) -> usize {
        self.matches.len()
    }

    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }

    pub fn match_(&self, id: &MatchId) -> Option<MatchRef> {
        self.matches.get(*id).map(|m| MatchRef::new(m.clone()))
    }

    pub fn current_matches(&self) -> Vec<CurrentMatch> {
        self.iter()
            .enumerate()
            .filter(|(_i, m)| {
                let state = m.borrow().state();
                matches!(state, MatchState::Ready)
            })
            .map(|(i, m)| CurrentMatch::new(i, &m))
            .collect()
    }

    pub fn iter(&self) -> BracketIterator {
        BracketIterator::from(&self.matches)
    }

    pub fn insert(&mut self, match_: Match) -> MatchId {
        self.matches.push(Rc::new(RefCell::new(match_)));
        self.matches.len() - 1
    }

    pub fn set_winner(
        &mut self,
        match_id: &MatchId,
        winner: &Contestant,
    ) -> Result<(), SetWinnerInvalid> {
        let maybe_match = self.matches.get_mut(*match_id);

        match maybe_match {
            None => Err(SetWinnerInvalid::MatchId),
            Some(match_) => match_.borrow_mut().set_winner(winner),
        }
    }
}
