use crate::bracket::match_ref::MatchRef;
use crate::match_::{Id, Match};
use std::cell::RefCell;
use std::collections::hash_map::Values;
use std::rc::Rc;

pub struct BracketIterator<'a> {
    map_iterator: Values<'a, Id, Rc<RefCell<Match>>>,
}

impl<'a> From<Values<'a, Id, Rc<RefCell<Match>>>> for BracketIterator<'a> {
    fn from(value: Values<'a, Id, Rc<RefCell<Match>>>) -> Self {
        Self {
            map_iterator: value,
        }
    }
}

impl<'a> Iterator for BracketIterator<'a> {
    type Item = MatchRef;

    fn next(&mut self) -> Option<Self::Item> {
        let match_ = self.map_iterator.next()?;
        Some(MatchRef::new(match_.clone()))
    }
}
