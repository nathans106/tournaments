use crate::contestant::{Contestant, Id};

pub struct NewContestant {
    id: Id,
    name: String,
}

impl NewContestant {
    #[allow(dead_code)]
    pub fn new(id: Id, name: String) -> Self {
        Self { id, name }
    }
}

impl Contestant for NewContestant {
    fn id(&self) -> &Id {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_contestant() {
        let contestant = NewContestant::new(0, "Nathan".to_string());

        assert_eq!(contestant.name(), "Nathan");
        assert_eq!(contestant.id(), &0);
    }

    #[test]
    fn set_name() {
        let mut contestant = NewContestant::new(0, "Nathan".to_string());

        contestant.set_name("Not Nathan".to_string());

        assert_eq!(contestant.name(), "Not Nathan");
    }
}
