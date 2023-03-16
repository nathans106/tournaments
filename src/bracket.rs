use crate::{contestant::Contestant, match_};
use std::array;
use std::cell::{Ref, RefCell};
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::rc::Rc;

use crate::match_::{Match, MatchState, SetWinnerInvalid};

#[allow(dead_code)]
#[derive(Default)]
pub struct Bracket {
    matches: HashMap<match_::Id, Rc<RefCell<Match>>>,
}

#[allow(dead_code)]
impl Bracket {
    pub fn len(&self) -> usize {
        self.matches.len()
    }

    pub fn match_(&self, id: &match_::Id) -> Option<MatchRef> {
        self.matches.get(id).map(|match_| MatchRef {
            match_: match_.clone(),
        })
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
        BracketIterator {
            map_iterator: self.matches.values(),
        }
    }

    pub fn insert(&mut self, match_: Match) -> match_::Id {
        let id = *match_.id();
        self.matches.insert(id, Rc::new(RefCell::new(match_)));
        id
    }

    pub fn set_winner(
        &mut self,
        match_id: &match_::Id,
        winner: &Contestant,
    ) -> Result<(), SetWinnerInvalid> {
        let maybe_match = self.matches.get_mut(match_id);

        match maybe_match {
            None => Err(SetWinnerInvalid::MatchId),
            Some(match_) => match_.borrow_mut().set_winner(winner),
        }
    }
}

pub struct MatchRef {
    match_: Rc<RefCell<Match>>,
}

impl MatchRef {
    pub fn borrow(&self) -> Ref<Match> {
        self.match_.borrow()
    }
}

pub struct BracketIterator<'a> {
    map_iterator: Values<'a, match_::Id, Rc<RefCell<Match>>>,
}

impl<'a> Iterator for BracketIterator<'a> {
    type Item = MatchRef;

    fn next(&mut self) -> Option<Self::Item> {
        let match_ = self.map_iterator.next()?;
        Some(MatchRef {
            match_: match_.clone(),
        })
    }
}

#[allow(dead_code)]
pub struct CurrentMatch {
    id: match_::Id,
    contestants: [Contestant; 2],
}

impl From<MatchRef> for CurrentMatch {
    fn from(match_ref: MatchRef) -> Self {
        let match_ = match_ref.borrow();
        assert!(matches!(match_.state(), MatchState::Ready));
        let mut contestants_iter = match_.contestants().iter().map(|c| c.contestant().unwrap());

        Self {
            id: *match_.id(),
            contestants: array::from_fn(|_| contestants_iter.next().unwrap()),
        }
    }
}
