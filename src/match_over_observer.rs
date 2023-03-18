use crate::contestant::Contestant;
use std::cell::RefCell;
use std::rc::Weak;

pub trait MatchOverPublisher {
    fn subscribe_winner(&mut self, subscriber: Weak<RefCell<dyn MatchOverSubscriber>>);
    fn subscribe_loser(&mut self, subscriber: Weak<RefCell<dyn MatchOverSubscriber>>);
    fn notify_subscribers(&mut self);
}

pub trait MatchOverSubscriber {
    fn update(&mut self, contestant: &Contestant);
}
