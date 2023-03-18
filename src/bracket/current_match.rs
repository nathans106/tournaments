use crate::bracket::match_ref::MatchRef;
use crate::bracket::MatchId;
use crate::contestant::Contestant;
use crate::match_::MatchState;

pub struct CurrentMatch {
    pub id: MatchId,
    pub contestants: [Contestant; 2],
}

impl CurrentMatch {
    pub fn new(id: MatchId, match_ref: &MatchRef) -> Self {
        let match_ = match_ref.borrow();
        match match_.state() {
            MatchState::InProgress(contestants) => {
                let mut contestants_iter = contestants.iter().cloned();

                Self {
                    id,
                    contestants: [
                        contestants_iter.next().unwrap(),
                        contestants_iter.next().unwrap(),
                    ],
                }
            }
            _ => panic!(),
        }
    }
}
