use crate::bracket::match_ref::MatchRef;
use crate::contestant::Contestant;
use crate::match_;
use crate::match_::MatchState;
use std::array;

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
