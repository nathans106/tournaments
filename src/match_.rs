use crate::contestant::Contestant;
use crate::match_over_observer::{MatchOverPublisher, MatchOverSubscriber};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Weak;

pub type Contestants = HashSet<Contestant>;

#[derive(Clone)]
pub struct MatchResult {
    pub winner: Contestant,
    pub loser: Contestant,
}

#[derive(Clone)]
pub enum MatchState {
    Waiting(Contestants),
    InProgress(Contestants),
    Finished(MatchResult),
}

#[derive(Clone)]
pub struct Match {
    state: MatchState,
    winner_subscribers: Vec<Weak<RefCell<dyn MatchOverSubscriber>>>,
    loser_subscribers: Vec<Weak<RefCell<dyn MatchOverSubscriber>>>,
}

impl Match {
    pub fn new(contestants: Contestants) -> Self {
        if contestants.len() > 2 {
            panic!()
        }

        let state = if contestants.len() == 2 {
            MatchState::InProgress(contestants)
        } else {
            MatchState::Waiting(contestants)
        };

        Self {
            state,
            winner_subscribers: vec![],
            loser_subscribers: vec![],
        }
    }

    pub fn state(&self) -> &MatchState {
        &self.state
    }

    pub fn set_winner(&mut self, winner: Contestant) -> Result<(), SetWinnerInvalid> {
        match &mut self.state {
            MatchState::InProgress(contestants) => {
                let winner_valid = contestants.remove(&winner);
                if !winner_valid {
                    return Err(SetWinnerInvalid::Contestant);
                }

                assert_eq!(contestants.len(), 1);
                let loser = contestants.iter().next().unwrap();

                self.state = MatchState::Finished(MatchResult {
                    winner,
                    loser: loser.clone(),
                });

                self.notify_subscribers();
                Ok(())
            }
            _ => Err(SetWinnerInvalid::State),
        }
    }
}

impl Default for Match {
    fn default() -> Self {
        Self {
            state: MatchState::Waiting(HashSet::default()),
            winner_subscribers: vec![],
            loser_subscribers: vec![],
        }
    }
}

impl MatchOverPublisher for Match {
    fn subscribe_winner<'a>(&mut self, subscriber: Weak<RefCell<dyn MatchOverSubscriber>>) {
        self.winner_subscribers.push(subscriber);
    }

    fn subscribe_loser<'a>(&mut self, subscriber: Weak<RefCell<dyn MatchOverSubscriber>>) {
        self.loser_subscribers.push(subscriber);
    }

    fn notify_subscribers(&mut self) {
        match &self.state {
            MatchState::Finished(match_result) => {
                for subscriber in self.winner_subscribers.iter_mut() {
                    subscriber
                        .upgrade()
                        .unwrap()
                        .borrow_mut()
                        .update(&match_result.winner);
                }

                for subscriber in self.loser_subscribers.iter_mut() {
                    subscriber
                        .upgrade()
                        .unwrap()
                        .borrow_mut()
                        .update(&match_result.loser);
                }
            }
            _ => panic!(),
        }
    }
}

impl MatchOverSubscriber for Match {
    fn update(&mut self, contestant: &Contestant) {
        match &mut self.state {
            MatchState::Waiting(contestants) => {
                assert!(contestants.len() < 2);
                contestants.insert(contestant.clone());

                if contestants.len() == 2 {
                    self.state = MatchState::InProgress(contestants.clone())
                }
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum SetWinnerInvalid {
    Contestant,
    MatchId,
    State,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_ready() {
        let match_ = Match::default();

        assert!(matches!(match_.state(), MatchState::Waiting(_)));
    }

    #[test]
    fn ready() {
        let match_ = Match::new(HashSet::from([
            "Nathan".to_string(),
            "Not Nathan".to_string(),
        ]));

        assert!(matches!(match_.state(), MatchState::InProgress(_)));
    }

    #[test]
    fn set_winner() {
        let mut match_ = Match::new(HashSet::from([
            "Nathan".to_string(),
            "Not Nathan".to_string(),
        ]));
        let winner = "Nathan".to_string();

        match_.set_winner(winner.clone()).unwrap();
        let state = match_.state();

        match state {
            MatchState::Finished(result) => assert_eq!(result.winner, winner),
            _ => panic!(),
        }
    }
}
