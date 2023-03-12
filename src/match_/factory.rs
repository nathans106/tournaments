use crate::match_::{Contenders, Id, Match, RcMatch};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
#[allow(dead_code)]
pub struct Factory {
    next_id: Id,
}

impl Factory {
    #[allow(dead_code)]
    pub fn create_match(&mut self, contestants: Contenders) -> RcMatch {
        let id = self.next_id;
        self.next_id += 1;
        Rc::new(RefCell::new(Match::new(id, contestants)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_contender::tests::dummy_contenders;

    #[test]
    fn create_match() {
        let mut factory = Factory::default();

        let match_ = factory.create_match(dummy_contenders());

        assert_eq!(match_.borrow().id(), &0);
    }

    #[test]
    fn two_matches() {
        let mut factory = Factory::default();

        let match1 = factory.create_match(dummy_contenders());
        let match2 = factory.create_match(dummy_contenders());

        assert_ne!(match1.borrow().id(), match2.borrow().id());
    }
}
