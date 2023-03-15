use crate::{contestant::Contestant, match_};
use std::cell::{Ref, RefCell};
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::rc::Rc;

use crate::match_::{Match, SetWinnerInvalid};

#[allow(dead_code)]
#[derive(Default)]
pub struct Matches {
    matches: HashMap<match_::Id, Rc<RefCell<Match>>>,
}

#[allow(dead_code)]
impl Matches {
    pub fn len(&self) -> usize {
        self.matches.len()
    }

    pub fn at(&self, id: &match_::Id) -> Option<MatchRef> {
        self.matches.get(id).map(|match_| MatchRef {
            match_: match_.clone(),
        })
    }

    pub fn iter(&self) -> MatchesIterator {
        MatchesIterator {
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

pub struct MatchesIterator<'a> {
    map_iterator: Values<'a, match_::Id, Rc<RefCell<Match>>>,
}

impl<'a> Iterator for MatchesIterator<'a> {
    type Item = MatchRef;

    fn next(&mut self) -> Option<Self::Item> {
        let match_ = self.map_iterator.next()?;
        Some(MatchRef {
            match_: match_.clone(),
        })
    }
}
