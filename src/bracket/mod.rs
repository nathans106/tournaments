pub mod current_match;
mod iterator;
pub mod match_ref;

use crate::bracket::current_match::CurrentMatch;
use crate::bracket::iterator::BracketIterator;
use crate::bracket::match_ref::MatchRef;
use crate::{contestant::Contestant, match_};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::match_::{Match, MatchState, SetWinnerInvalid};

#[allow(dead_code)]
#[derive(Default)]
pub struct Bracket {
    matches: HashMap<match_::MatchId, Rc<RefCell<Match>>>,
}

#[allow(dead_code)]
impl Bracket {
    pub fn len(&self) -> usize {
        self.matches.len()
    }

    pub fn match_(&self, id: &match_::MatchId) -> Option<MatchRef> {
        self.matches
            .get(id)
            .map(|match_| MatchRef::new(match_.clone()))
    }

    pub fn current_matches(&self) -> Vec<CurrentMatch> {
        self.iter()
            .filter(|m| {
                let state = m.borrow().state();
                matches!(state, MatchState::Ready)
            })
            .map(CurrentMatch::from)
            .collect()
    }

    pub fn iter(&self) -> BracketIterator {
        BracketIterator::from(self.matches.values())
    }

    pub fn insert(&mut self, match_: Match) -> match_::MatchId {
        let id = *match_.id();
        self.matches.insert(id, Rc::new(RefCell::new(match_)));
        id
    }

    pub fn set_winner(
        &mut self,
        match_id: &match_::MatchId,
        winner: &Contestant,
    ) -> Result<(), SetWinnerInvalid> {
        let maybe_match = self.matches.get_mut(match_id);

        match maybe_match {
            None => Err(SetWinnerInvalid::MatchId),
            Some(match_) => match_.borrow_mut().set_winner(winner),
        }
    }
}
