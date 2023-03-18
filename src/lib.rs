use crate::contestant::ContestantsError;
use crate::tournament::Tournament;

pub mod bracket;
pub mod bracket_builder;
pub mod contestant;
mod match_;
mod match_over_observer;
pub mod tournament;

pub fn single_elimination_tournament(
    contestants: &[String],
) -> Result<Tournament, ContestantsError> {
    Tournament::new::<bracket_builder::SingleElimination>(contestants)
}
