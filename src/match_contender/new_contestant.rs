use crate::contestant::Contestant;
use crate::match_contender::MatchContender;

#[derive(Clone, Debug, PartialEq)]
pub struct NewContestant {
    contestant: Contestant,
}

impl NewContestant {
    pub fn new(contestant: Contestant) -> Self {
        Self { contestant }
    }
}

impl MatchContender for NewContestant {
    fn contestant(&self) -> Option<Contestant> {
        Some(self.contestant.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_contender() {
        let contestant = NewContestant::new("Nathan".to_string());
        assert_eq!(contestant.contestant().unwrap(), "Nathan".to_string())
    }
}
