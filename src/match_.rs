use crate::contestant;
use crate::match_contender::MatchContender;

pub type Id = u32;
pub type Contenders = [Box<dyn MatchContender>; 2];

#[derive(Clone)]
pub enum MatchState {
    NotReady,
    Ready,
    Won(contestant::Id),
}

#[allow(dead_code)]
pub struct Match {
    id: Id,
    contenders: Contenders,
    winner_id: Option<contestant::Id>,
}

#[allow(dead_code)]
impl Match {
    pub fn new(id: Id, contestants: Contenders) -> Self {
        Self {
            id,
            contenders: contestants,
            winner_id: None,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn contestants(&self) -> &Contenders {
        &self.contenders
    }

    pub fn state(&self) -> MatchState {
        match self.winner_id {
            None => {
                let not_ready_contender =
                    self.contenders.iter().find(|c| c.contestant_id().is_none());
                match not_ready_contender {
                    None => MatchState::Ready,
                    Some(_) => MatchState::NotReady,
                }
            }
            Some(winner_id) => MatchState::Won(winner_id),
        }
    }

    pub fn set_winner(&mut self, id: &contestant::Id) -> Result<(), SetWinnerError> {
        if !matches!(self.state(), MatchState::Ready) {
            return Err(SetWinnerError::InvalidState);
        }

        let id_valid = self
            .contenders
            .iter()
            .any(|c| &c.contestant_id().unwrap() == id);
        if !id_valid {
            return Err(SetWinnerError::InvalidId);
        }

        self.winner_id = Some(*id);
        Ok(())
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

    struct NoContender;
    impl MatchContender for NoContender {
        fn contestant_id(&self) -> Option<contestant::Id> {
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
        let mut factory = contestant::Factory::default();

        let contestants: Contenders = [
            Box::new(factory.create_contestant("Nathan".to_string())),
            Box::new(factory.create_contestant("Not Nathan".to_string())),
        ];

        let match_ = Match::new(0, contestants);

        assert!(matches!(match_.state(), MatchState::Ready));
    }

    #[test]
    fn set_winner() {
        let mut factory = contestant::Factory::default();

        let contestants: Contenders = [
            Box::new(factory.create_contestant("Nathan".to_string())),
            Box::new(factory.create_contestant("Not Nathan".to_string())),
        ];

        let mut match_ = Match::new(0, contestants);
        let winner = match_.contestants().first().unwrap();
        let winner_id = winner.contestant_id().unwrap();
        match_.set_winner(&winner_id).unwrap();
        let state = match_.state();

        assert!(matches!(state, MatchState::Won(_)));
    }
}
