use crate::contestant::ContestantsError;
use crate::tournament::Tournament;

mod bracket;
mod bracket_builder;
mod contestant;
mod match_;
mod match_contender;
mod tournament;

pub fn single_elimination_tournament(
    contestants: &[String],
) -> Result<Tournament, ContestantsError> {
    Tournament::new::<bracket_builder::SingleElimination>(contestants)
}
