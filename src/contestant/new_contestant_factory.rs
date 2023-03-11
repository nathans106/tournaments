use crate::contestant::new_contestant::NewContestant;
use crate::contestant::Id;

#[derive(Default)]
pub struct NewContestantFactory {
    next_id: Id,
}

impl NewContestantFactory {
    #[allow(dead_code)]
    pub fn create_contestant(&mut self, name: String) -> NewContestant {
        let id = self.next_id;
        self.next_id += 1;
        NewContestant::new(id, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contestant::Contestant;

    #[test]
    fn create_contestant() {
        let mut factory = NewContestantFactory::default();

        let contestant = factory.create_contestant("Nathan".to_string());

        assert_eq!(contestant.name(), &"Nathan".to_string());
    }

    #[test]
    fn two_contestants() {
        let mut factory = NewContestantFactory::default();

        let contestant1 = factory.create_contestant("Nathan".to_string());
        let contestant2 = factory.create_contestant("Not Nathan".to_string());

        assert_ne!(contestant1.id(), contestant2.id());
    }
}
