use crate::bracket::{Bracket, MatchId};
use crate::bracket_builder::BracketBuilder;
use crate::contestant::{Contestant, ContestantsError};
use crate::match_::{MatchState, SetWinnerInvalid};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Tournament {
    bracket: Bracket,
    contestants: HashSet<Contestant>,
}

impl Tournament {
    pub fn new<BracketBuilderT: BracketBuilder>(
        contestants: &[Contestant],
    ) -> Result<Self, ContestantsError> {
        if !contestants.iter().all_unique() {
            return Err(ContestantsError::NotUnique);
        }

        let bracket = BracketBuilderT::build_bracket(contestants)?;

        Ok(Tournament {
            bracket,
            contestants: HashSet::from_iter(contestants.iter().cloned()),
        })
    }

    pub fn bracket(&self) -> &Bracket {
        &self.bracket
    }

    pub fn rankings(&self) -> Vec<Vec<Contestant>> {
        let mut contestant_wins = self
            .bracket
            .iter()
            .filter_map(|m| match m.borrow().state() {
                MatchState::Finished(match_result) => Some(match_result.winner.clone()),
                _ => None,
            })
            .counts();

        for contestant in &self.contestants {
            if !contestant_wins.contains_key(contestant) {
                contestant_wins.insert(contestant.clone(), 0);
            }
        }

        let contestants_grouped_by_wins = contestant_wins.into_iter().group_by(|(_, w)| *w);

        contestants_grouped_by_wins
            .into_iter()
            .sorted_by_key(|(w, _)| *w)
            .rev()
            .map(|(_, g)| g.map(|(c, _)| c).collect_vec())
            .collect_vec()
    }

    pub fn is_finished(&self) -> bool {
        for match_ in self.bracket.iter() {
            match match_.borrow().state() {
                MatchState::Finished(_) => (),
                _ => return false,
            }
        }

        true
    }

    pub fn set_winner(
        &mut self,
        match_id: &MatchId,
        winner: Contestant,
    ) -> Result<(), SetWinnerInvalid> {
        self.bracket.set_winner(match_id, winner)
    }
}
