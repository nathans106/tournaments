use crate::bracket::match_ref::MatchRef;
use crate::contestant::Contestant;
use crate::match_::MatchState;
use crate::match_contender::MatchContender;

pub struct Winner {
    match_: MatchRef,
}

impl Winner {
    pub fn new(match_: MatchRef) -> Self {
        Self { match_ }
    }
}

impl MatchContender for Winner {
    fn contestant(&self) -> Option<Contestant> {
        match self.match_.borrow().state() {
            MatchState::Won(winner) => Some(winner),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bracket::Bracket;
    use crate::match_::Match;
    use crate::match_contender::tests::dummy_contenders;

    #[test]
    fn no_winner() {
        let contestants = dummy_contenders();
        let mut matches = Bracket::default();
        matches.insert(Match::new(contestants));

        let winner = Winner::new(matches.match_(&0).unwrap());

        assert!(winner.contestant().is_none());
    }

    #[test]
    fn winner() {
        let contestants = dummy_contenders();
        let mut matches = Bracket::default();

        matches.insert(Match::new(contestants));
        let winner = Winner::new(matches.match_(&0).unwrap());
        matches.set_winner(&0, &"Nathan".to_string()).unwrap();

        assert_eq!(winner.contestant().unwrap(), "Nathan".to_string());
    }
}
