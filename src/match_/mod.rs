mod factory;

pub use factory::Factory;
use std::cell::RefCell;

use crate::contestant;
use crate::contestant::Contestant;
use crate::match_contender::MatchContender;
use std::rc::Rc;

pub type Id = u32;
pub type Contenders = [Box<dyn MatchContender>; 2];
pub type RcMatch = Rc<RefCell<Match>>;

#[derive(Clone)]
pub enum MatchState {
    NotReady,
    Ready,
    Won(Rc<Contestant>),
}

#[allow(dead_code)]
pub struct Match {
    id: Id,
    contenders: Contenders,
    winner: Option<Rc<Contestant>>,
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

    pub fn set_winner(&mut self, id: &contestant::Id) -> Result<(), SetWinnerError> {
        if !matches!(self.state(), MatchState::Ready) {
            return Err(SetWinnerError::InvalidState);
        }

        let maybe_winner = self
            .contenders
            .iter()
            .find(|c| c.contestant().unwrap().id() == id);

        match maybe_winner {
            None => Err(SetWinnerError::InvalidId),
            Some(winner) => {
                self.winner = winner.contestant();
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum SetWinnerError {
    InvalidId,
    InvalidState,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_contender::tests::dummy_contenders;
    use std::rc::Rc;

    struct NoContender;
    impl MatchContender for NoContender {
        fn contestant(&self) -> Option<Rc<Contestant>> {
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
        let winner = match_.contestants().first().unwrap().contestant().unwrap();

        match_.set_winner(winner.id()).unwrap();

        assert!(matches!(match_.state(), MatchState::Won(_)));
    }
}
