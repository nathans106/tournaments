use crate::error::InvalidNumberOfContestantsError;
use crate::match_::{Contenders, Match, MatchState, RcMatch};
use crate::match_contender::new_contestant::NewContestant;
use crate::match_contender::{MatchContender, Winner};
use crate::{contestant, match_};
use itertools::Itertools;
use std::array;
use std::cell::Ref;

pub struct SingleElimination {
    matches: Vec<RcMatch>,
}

impl SingleElimination {
    pub fn new(names: &[String]) -> Result<Self, InvalidNumberOfContestantsError> {
        let num_contestants = names.len();

        if !num_contestants.is_power_of_two() {
            return Err(InvalidNumberOfContestantsError::new(num_contestants));
        }

        let mut match_factory = match_::Factory::default();
        let mut contestant_factory = contestant::Factory::default();
        let mut matches = vec![];

        for mut name_pair in &names.iter().chunks(2) {
            let contenders: Contenders = array::from_fn(|i| {
                let name = name_pair
                    .next()
                    .unwrap_or_else(|| panic!("Name {} missing", i));
                let contestant = contestant_factory.create_contestant(name.clone());
                let contender: Box<dyn MatchContender> = Box::new(NewContestant::new(contestant));
                contender
            });

            let match_ = match_factory.create_match(contenders);
            matches.push(match_);
        }

        let num_rounds = (num_contestants as f64).sqrt() as u32;

        for round_num in 1..num_rounds {
            let num_matches = num_contestants / (2_i32.pow(round_num + 1) as usize);

            for match_num in 0..num_matches {
                let match_id = matches.len();

                let mut selectors_iter = (0..2).map(|contestant_num| {
                    let rev_idx = match_id + match_num - contestant_num - 1;
                    let qualifying_match = matches
                        .iter()
                        .rev()
                        .nth(rev_idx)
                        .unwrap_or_else(|| panic!("Reverse index {} not valid", rev_idx));
                    let winner: Box<dyn MatchContender> = Box::new(Winner::new(qualifying_match));
                    winner
                });

                let selectors: Contenders = std::array::from_fn(|_| selectors_iter.next().unwrap());

                matches.push(match_factory.create_match(selectors));
            }
        }

        Ok(SingleElimination { matches })
    }

    pub fn current_matches(&self) -> Vec<Ref<Match>> {
        self.matches
            .iter()
            .map(|m| m.borrow())
            .filter(|m| {
                let state = m.state();
                matches!(state, MatchState::Ready)
            })
            .collect()
    }
}
