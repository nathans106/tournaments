mod factory;
pub use factory::Factory;

use crate::contestant;
use crate::contestant::Contestant;
use crate::match_contender::MatchContender;

pub type Id = u32;
pub type Contenders = [Box<dyn MatchContender>; 2];

#[derive(Clone)]
pub enum MatchState {
    NotReady,
    Ready,
    Won(Contestant),
}

#[allow(dead_code)]
pub struct Match {
    id: Id,
    contenders: Contenders,
    winner: Option<Contestant>,
}

#[allow(dead_code)]
impl Match {
    pub fn new(id: Id, contestants: Contenders) -> Self {
        Self {
            id,
            contenders: contestants,
            winner: None,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn contestants(&self) -> &Contenders {
        &self.contenders
    }

    pub fn state(&self) -> MatchState {
        match &self.winner {
            None => {
                let not_ready_contender = self.contenders.iter().find(|c| c.contestant().is_none());
                match not_ready_contender {
                    None => MatchState::Ready,
                    Some(_) => MatchState::NotReady,
                }
            }
            Some(winner) => MatchState::Won(winner.clone()),
        }
    }

    pub fn set_winner(&mut self, id: &contestant::Id) -> Result<(), SetWinnerInvalid> {
        if !matches!(self.state(), MatchState::Ready) {
            return Err(SetWinnerInvalid::State);
        }

        let maybe_winner = self
            .contenders
            .iter()
            .find(|c| c.contestant().unwrap().id() == id);

        match maybe_winner {
            None => Err(SetWinnerInvalid::ContestantId),
            Some(winner) => {
                self.winner = winner.contestant();
                Ok(())
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SetWinnerInvalid {
    ContestantId,
    MatchId,
    State,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_contender::tests::dummy_contenders;

    struct NoContender;
    impl MatchContender for NoContender {
        fn contestant(&self) -> Option<Contestant> {
            None
        }
    }

    #[test]
    fn not_ready() {
        let contestants: Contenders = [Box::new(NoContender), Box::new(NoContender)];

        let match_ = Match::new(0, contestants);

        assert!(matches!(match_.state(), MatchState::NotReady));
    }

    #[test]
    fn ready() {
        let contestants = dummy_contenders();
        let match_ = Match::new(0, contestants);

        assert!(matches!(match_.state(), MatchState::Ready));
    }

    #[test]
    fn set_winner() {
        let contestants = dummy_contenders();
        let mut match_ = Match::new(0, contestants);
        let winner_id = match_
            .contestants()
            .first()
            .unwrap()
            .contestant()
            .unwrap()
            .id()
            .clone();

        match_.set_winner(&winner_id).unwrap();
        let state = match_.state();

        match state {
            MatchState::Won(winner) => assert_eq!(winner.id(), &winner_id),
            _ => panic!(),
        }
    }
}
