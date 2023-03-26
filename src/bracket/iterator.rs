use crate::bracket::match_ref::MatchRef;
use crate::match_::Match;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

/// An iterator for [Bracket] which returns immutable [MatchRef] objects.
pub struct BracketIterator<'a> {
    iterator: Iter<'a, Rc<RefCell<Match>>>,
}

impl<'a> From<&'a Vec<Rc<RefCell<Match>>>> for BracketIterator<'a> {
    fn from(value: &'a Vec<Rc<RefCell<Match>>>) -> Self {
        Self {
            iterator: value.iter(),
        }
    }
}

impl<'a> Iterator for BracketIterator<'a> {
    type Item = MatchRef;

    fn next(&mut self) -> Option<Self::Item> {
        let match_ = self.iterator.next()?;
        Some(MatchRef::new(match_.clone()))
    }
}
