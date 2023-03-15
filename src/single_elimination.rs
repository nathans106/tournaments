use crate::contestant::Contestant;
use crate::match_;
use crate::match_::{Contenders, MatchState};
use crate::match_contender::{MatchContender, NewContestant, Winner};
use crate::matches::{MatchRef, Matches};
use itertools::Itertools;
use std::array;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct SingleElimination {
    matches: Matches,
}

impl SingleElimination {
    pub fn new(names: &[String]) -> Result<Self, NewSingleEliminationError> {
        let num_contestants = names.len();

        if !names.iter().all_unique() {
            return Err(NewSingleEliminationError::ContestantsNotUnique);
        }

        if !num_contestants.is_power_of_two() {
            return Err(NewSingleEliminationError::InvalidNumberOfContestants(
                num_contestants,
            ));
        }

        let mut match_factory = match_::Factory::default();
        let mut tournament = SingleElimination {
            matches: Matches::default(),
        };

        let mut last_round = vec![];
        for mut name_pair in &names.iter().chunks(2) {
            let contenders: Contenders = array::from_fn(|i| {
                let name = name_pair
                    .next()
                    .unwrap_or_else(|| panic!("Name {} missing", i));
                let contender: Box<dyn MatchContender> = Box::new(NewContestant::new(name.clone()));
                contender
            });

            let match_ = match_factory.create_match(contenders);
            last_round.push(tournament.matches.insert(match_));
        }

        let num_rounds = (num_contestants as f64).sqrt() as u32;

        for round_num in 1..num_rounds {
            let num_matches = num_contestants / (2_i32.pow(round_num + 1) as usize);
            let mut cur_round = vec![];

            for match_num in 0..num_matches {
                let mut contenders_iter = (0..2).map(|contestant_num| {
                    let qualifying_match_id = last_round[(match_num * 2) + contestant_num];

                    let qualifying_match = tournament.matches.at(&qualifying_match_id).unwrap();
                    let winner: Box<dyn MatchContender> = Box::new(Winner::new(qualifying_match));
                    winner
                });

                let selectors: Contenders =
                    std::array::from_fn(|_| contenders_iter.next().unwrap());

                cur_round.push(
                    tournament
                        .matches
                        .insert(match_factory.create_match(selectors)),
                );
            }

            last_round = cur_round;
        }

        Ok(tournament)
    }

    pub fn current_matches(&self) -> Vec<CurrentMatch> {
        self.matches
            .iter()
            .filter(|m| {
                let state = m.borrow().state();
                matches!(state, MatchState::Ready)
            })
            .map(CurrentMatch::from)
            .collect()
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

#[derive(Debug)]
pub enum NewSingleEliminationError {
    ContestantsNotUnique,
    InvalidNumberOfContestants(usize),
}

impl Display for NewSingleEliminationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NewSingleEliminationError::ContestantsNotUnique => {
                write!(f, "Contestants are not unique")
            }
            NewSingleEliminationError::InvalidNumberOfContestants(num) => {
                write!(
                    f,
                    "Invalid number of contestants, must be a power of 2: {}",
                    num
                )
            }
        }
    }
}

impl Error for NewSingleEliminationError {}
