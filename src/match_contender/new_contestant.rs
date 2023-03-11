use crate::contestant::Contestant;
use crate::match_contender::MatchContender;
use std::rc::Rc;

pub struct NewContestant {
    contestant: Rc<Contestant>,
}

impl NewContestant {
    #[allow(dead_code)]
    pub fn new(contestant: Contestant) -> Self {
        Self {
            contestant: Rc::new(contestant),
        }
    }
}

impl MatchContender for NewContestant {
    fn contestant(&self) -> Option<Rc<Contestant>> {
        Some(self.contestant.clone())
    }
}
