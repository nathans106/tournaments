use crate::bracket::{Bracket, MatchRef};
use crate::bracket_builder::BracketBuilder;
use crate::contestant::{Contestant, ContestantsError};
use crate::match_;
use crate::match_::MatchState;
use crate::match_::SetWinnerInvalid;
use itertools::Itertools;
use std::array;

pub struct Tournament {
    bracket: Bracket,
    final_id: match_::Id,
}

impl Tournament {
    pub fn new<BracketBuilderT: BracketBuilder>(
        contestants: &[String],
    ) -> Result<Self, ContestantsError> {
        if !contestants.iter().all_unique() {
            return Err(ContestantsError::NotUnique);
        }

        let (bracket, final_id) = BracketBuilderT::build_bracket(contestants)?;
        Ok(Tournament { bracket, final_id })
    }

    pub fn current_matches(&self) -> Vec<CurrentMatch> {
        self.bracket
            .iter()
            .filter(|m| {
                let state = m.borrow().state();
                matches!(state, MatchState::Ready)
            })
            .map(CurrentMatch::from)
            .collect()
    }

    pub fn set_winner(
        &mut self,
        match_id: &match_::Id,
        winner: &Contestant,
    ) -> Result<Option<Contestant>, SetWinnerInvalid> {
        self.bracket.set_winner(match_id, winner).map(|_| {
            if match_id == &self.final_id {
                return Some(winner.clone());
            }

            None
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
