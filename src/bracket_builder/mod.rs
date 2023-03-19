mod single_elimination;
pub use single_elimination::SingleElimination;

use crate::bracket::Bracket;
use crate::contestant::{Contestant, ContestantsError};

pub trait BracketBuilder {
    fn build_bracket(contestants: &[Contestant]) -> Result<Bracket, ContestantsError>;
}
