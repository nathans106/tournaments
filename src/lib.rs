//! Library for the construction and running of tournaments.

use crate::contestant::{Contestant, ContestantsError};
use crate::tournament::Tournament;

pub mod bracket;
pub mod bracket_builder;
pub mod contestant;
mod match_;
mod match_over_observer;
pub mod tournament;

/// Constructs a new Round Robin [Tournament].
pub fn round_robin_tournament(contestants: &[Contestant]) -> Result<Tournament, ContestantsError> {
    Tournament::new::<bracket_builder::RoundRobin>(contestants)
}

/// Constructs a new Single Elimination [Tournament].
pub fn single_elimination_tournament(
    contestants: &[Contestant],
) -> Result<Tournament, ContestantsError> {
    Tournament::new::<bracket_builder::SingleElimination>(contestants)
}
