use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct InvalidNumberOfContestantsError {
    num: usize,
}

impl InvalidNumberOfContestantsError {
    pub fn new(num: usize) -> Self {
        Self { num }
    }
}

impl Display for InvalidNumberOfContestantsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid number of contestants, must be a power of 2: {}",
            self.num
        )
    }
}

impl Error for InvalidNumberOfContestantsError {}
