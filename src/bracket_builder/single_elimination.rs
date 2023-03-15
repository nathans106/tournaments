use crate::bracket::Bracket;
use crate::bracket_builder::BracketBuilder;
use crate::contestant::{Contestant, ContestantsError};
use crate::match_;
use crate::match_::Contenders;
use crate::match_contender::{MatchContender, NewContestant, Winner};
use itertools::Itertools;
use std::array;

pub struct SingleElimination {}

impl BracketBuilder for SingleElimination {
    fn build_bracket(
        contestants: &[Contestant],
    ) -> Result<(Bracket, match_::Id), ContestantsError> {
        let num_contestants = contestants.len();

        if !num_contestants.is_power_of_two() {
            return Err(ContestantsError::InvalidNumber(num_contestants));
        }

        let mut match_factory = match_::Factory::default();
        let mut bracket = Bracket::default();

        let mut last_round = vec![];
        for mut contestants_pair in &contestants.iter().chunks(2) {
            let contenders: Contenders = array::from_fn(|i| {
                let name = contestants_pair
                    .next()
                    .unwrap_or_else(|| panic!("Name {} missing", i));
                let contender: Box<dyn MatchContender> = Box::new(NewContestant::new(name.clone()));
                contender
            });

            let match_ = match_factory.create_match(contenders);
            last_round.push(bracket.insert(match_));
        }

        let num_rounds = (num_contestants as f64).sqrt() as u32;

        for round_num in 1..num_rounds {
            let num_matches = num_contestants / (2_i32.pow(round_num + 1) as usize);
            let mut cur_round = vec![];

            for match_num in 0..num_matches {
                let mut contenders_iter = (0..2).map(|contestant_num| {
                    let qualifying_match_id = last_round[(match_num * 2) + contestant_num];

                    let qualifying_match = bracket.match_(&qualifying_match_id).unwrap();
                    let winner: Box<dyn MatchContender> = Box::new(Winner::new(qualifying_match));
                    winner
                });

                let selectors: Contenders =
                    std::array::from_fn(|_| contenders_iter.next().unwrap());

                cur_round.push(bracket.insert(match_factory.create_match(selectors)));
            }

            last_round = cur_round;
        }

        let final_id = *last_round.last().unwrap();
        Ok((bracket, final_id))
    }
}
