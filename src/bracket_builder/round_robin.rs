use crate::bracket::Bracket;
use crate::bracket_builder::BracketBuilder;
use crate::contestant::{Contestant, ContestantsError};
use std::cell::RefCell;
use std::rc::Rc;

use crate::match_::Match;
use std::collections::HashSet;
use std::iter::zip;

/// Round Robin [BracketBuilder]. Uses the
/// [Circle Method algorithm](https://en.wikipedia.org/wiki/Round-robin_tournament#Circle_method)
pub struct RoundRobin;

impl BracketBuilder for RoundRobin {
    fn build_bracket(contestants: &[Contestant]) -> Result<Bracket, ContestantsError> {
        let num_contestants = contestants.len();

        if (num_contestants % 2) != 0 {
            return Err(ContestantsError::InvalidNumber(num_contestants));
        }

        let mut bracket = Bracket::default();
        let mut indexes: Vec<usize> = (0..num_contestants).collect();

        for _round in 0..(num_contestants - 1) {
            let (group1, group2) = indexes.split_at(num_contestants / 2);
            let iter1 = group1.iter();
            let iter2 = group2.iter().rev();

            for (index1, index2) in zip(iter1, iter2) {
                let contestant1 = contestants[*index1].clone();
                let contestant2 = contestants[*index2].clone();
                let match_ = Rc::new(RefCell::new(Match::new(HashSet::from([
                    contestant1,
                    contestant2,
                ]))));
                bracket.insert(match_);
            }

            let last_index = indexes.pop().unwrap();
            indexes.insert(1, last_index);
        }

        Ok(bracket)
    }
}
