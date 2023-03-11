use crate::contestant::{Contestant, Id};

#[derive(Default)]
pub struct Factory {
    next_id: Id,
}

impl Factory {
    #[allow(dead_code)]
    pub fn create_contestant(&mut self, name: String) -> Contestant {
        let id = self.next_id;
        self.next_id += 1;
        Contestant::new(id, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_contestant() {
        let mut factory = Factory::default();

        let contestant = factory.create_contestant("Nathan".to_string());

        assert_eq!(contestant.name(), &"Nathan".to_string());
    }

    #[test]
    fn two_contestants() {
        let mut factory = Factory::default();

        let contestant1 = factory.create_contestant("Nathan".to_string());
        let contestant2 = factory.create_contestant("Not Nathan".to_string());

        assert_ne!(contestant1.id(), contestant2.id());
    }
}
