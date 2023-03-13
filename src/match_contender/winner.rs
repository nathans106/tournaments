use crate::contestant::Contestant;
use crate::match_::MatchState;
use crate::match_contender::MatchContender;
use crate::matches::MatchRef;

pub struct Winner {
    match_: MatchRef,
}

impl Winner {
    #[allow(dead_code)]
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
    use crate::match_::Match;
    use crate::match_contender::tests::dummy_contenders;
    use crate::matches::Matches;

    #[test]
    fn no_winner() {
        let contestants = dummy_contenders();
        let mut matches = Matches::default();
        matches.insert(Match::new(0, contestants));

        let winner = Winner::new(matches.at(&0).unwrap());

        assert!(winner.contestant().is_none());
    }

    #[test]
    fn winner() {
        let contestants = dummy_contenders();
        let mut matches = Matches::default();

        matches.insert(Match::new(0, contestants));
        let winner = Winner::new(matches.at(&0).unwrap());
        matches.set_winner(&0, &0).unwrap();

        assert_eq!(winner.contestant().unwrap().id(), &0);
    }
}
