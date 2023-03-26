mod round_robin;
mod single_elimination;

pub use round_robin::RoundRobin;
pub use single_elimination::SingleElimination;

use crate::bracket::Bracket;
use crate::contestant::{Contestant, ContestantsError};

/// A trait for constructing a [Bracket].
pub trait BracketBuilder {
    fn build_bracket(contestants: &[Contestant]) -> Result<Bracket, ContestantsError>;
}
