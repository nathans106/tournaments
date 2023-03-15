mod new_contestant;
mod winner;

pub use new_contestant::NewContestant;
pub use winner::Winner;

use crate::contestant::Contestant;

pub trait MatchContender {
    fn contestant(&self) -> Option<Contestant>;
}

#[cfg(test)]
pub mod tests {
    use crate::match_::Contenders;
    use crate::match_contender::NewContestant;

    pub fn dummy_contenders() -> Contenders {
        [
            Box::new(NewContestant::new("Nathan".to_string())),
            Box::new(NewContestant::new("Not Nathan".to_string())),
        ]
    }
}
