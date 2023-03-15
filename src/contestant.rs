use std::fmt::{Display, Formatter};

pub type Contestant = String;

#[derive(Debug)]
pub enum ContestantsError {
    NotUnique,
    InvalidNumber(usize),
}

impl Display for ContestantsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContestantsError::NotUnique => {
                write!(f, "Contestants are not unique")
            }
            ContestantsError::InvalidNumber(num) => {
                write!(
                    f,
                    "Invalid number of contestants, must be a power of 2: {}",
                    num
                )
            }
        }
    }
}

impl std::error::Error for ContestantsError {}
