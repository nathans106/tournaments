use crate::bracket::Bracket;
use crate::bracket_builder::BracketBuilder;
use crate::contestant::{Contestant, ContestantsError};
use crate::match_::Match;
use crate::match_over_observer::MatchOverSubscriber;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SingleElimination {}

impl BracketBuilder for SingleElimination {
    fn build_bracket(contestants: &[Contestant]) -> Result<Bracket, ContestantsError> {
        let num_contestants = contestants.len();

        if !num_contestants.is_power_of_two() {
            return Err(ContestantsError::InvalidNumber(num_contestants));
        }

        let mut bracket = Bracket::default();
        let mut last_round = vec![];

        for contestants_pair in &contestants.iter().chunks(2) {
            let match_contestants = contestants_pair.into_iter().cloned().collect();
            let match_ = Rc::new(RefCell::new(Match::new(match_contestants)));
            last_round.push(bracket.insert(match_));
        }

        let num_rounds = (num_contestants as f64).sqrt() as u32;

        for round_num in 1..num_rounds {
            let num_matches = num_contestants / (2_i32.pow(round_num + 1) as usize);
            let mut cur_round = vec![];

            for match_num in 0..num_matches {
                let match_ = Rc::new(RefCell::new(Match::default()));

                for contestant_num in 0..2 {
                    let qualifying_match_id = last_round[(match_num * 2) + contestant_num];
                    let qualifying_publisher =
                        bracket.match_over_publisher(&qualifying_match_id).unwrap();

                    let subscriber: Rc<RefCell<dyn MatchOverSubscriber>> = match_.clone();

                    qualifying_publisher
                        .borrow_mut()
                        .subscribe_winner(Rc::downgrade(&subscriber));
                }

                cur_round.push(bracket.insert(match_));
            }

            last_round = cur_round;
        }

        Ok(bracket)
    }
}
