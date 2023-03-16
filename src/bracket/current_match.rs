use crate::bracket::match_ref::MatchRef;
use crate::bracket::MatchId;
use crate::contestant::Contestant;
use crate::match_::MatchState;
use std::array;

#[allow(dead_code)]
pub struct CurrentMatch {
    id: MatchId,
    contestants: [Contestant; 2],
}

impl CurrentMatch {
    pub fn new(id: MatchId, match_ref: &MatchRef) -> Self {
        let match_ = match_ref.borrow();
        assert!(matches!(match_.state(), MatchState::Ready));
        let mut contestants_iter = match_.contestants().iter().map(|c| c.contestant().unwrap());

        Self {
            id,
            contestants: array::from_fn(|_| contestants_iter.next().unwrap()),
        }
    }
}
