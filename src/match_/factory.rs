use crate::match_::{Contenders, Match, MatchId};

#[derive(Default)]
#[allow(dead_code)]
pub struct Factory {
    next_id: MatchId,
}

impl Factory {
    #[allow(dead_code)]
    pub fn create_match(&mut self, contestants: Contenders) -> Match {
        let id = self.next_id;
        self.next_id += 1;
        Match::new(id, contestants)
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

        assert_eq!(match_.id(), &0);
    }

    #[test]
    fn two_matches() {
        let mut factory = Factory::default();

        let match1 = factory.create_match(dummy_contenders());
        let match2 = factory.create_match(dummy_contenders());

        assert_ne!(match1.id(), match2.id());
    }
}
