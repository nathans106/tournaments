mod single_elimination;
pub use single_elimination::SingleElimination;

use crate::bracket::Bracket;
use crate::contestant::{Contestant, ContestantsError};
use crate::match_;

pub trait BracketBuilder {
    fn build_bracket(contestants: &[Contestant])
        -> Result<(Bracket, match_::Id), ContestantsError>;
}
