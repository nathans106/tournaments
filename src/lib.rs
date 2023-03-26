//! Library for the construction and running of tournaments.

pub mod bracket;
pub mod bracket_builder;
mod contestant;
pub mod match_;
pub mod match_over_observer;
mod tournament;

pub use contestant::{Contestant, ContestantsError};
pub use tournament::Tournament;

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
