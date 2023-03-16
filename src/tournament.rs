use crate::bracket::Bracket;
use crate::bracket_builder::BracketBuilder;
use crate::contestant::{Contestant, ContestantsError};
use crate::match_;
use crate::match_::SetWinnerInvalid;
use itertools::Itertools;

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

    pub fn bracket(&self) -> &Bracket {
        &self.bracket
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
