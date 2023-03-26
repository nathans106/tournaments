use crate::match_::Match;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

/// An immutable reference to a [Match].
pub struct MatchRef {
    match_: Rc<RefCell<Match>>,
}

impl MatchRef {
    pub fn new(match_: Rc<RefCell<Match>>) -> Self {
        Self { match_ }
    }

    pub fn borrow(&self) -> Ref<Match> {
        self.match_.borrow()
    }
}
