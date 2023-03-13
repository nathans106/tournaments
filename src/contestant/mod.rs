mod factory;

use crate::match_contender::MatchContender;
pub use factory::Factory;

pub type Id = u32;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Contestant {
    id: Id,
    name: String,
}

#[allow(dead_code)]
impl Contestant {
    pub fn new(id: Id, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl MatchContender for Contestant {
    fn contestant(&self) -> Option<Contestant> {
        Some(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_contestant() {
        let contestant = Contestant::new(0, "Nathan".to_string());

        assert_eq!(contestant.name(), "Nathan");
        assert_eq!(contestant.id(), &0);
    }

    #[test]
    fn set_name() {
        let mut contestant = Contestant::new(0, "Nathan".to_string());

        contestant.set_name("Not Nathan".to_string());

        assert_eq!(contestant.name(), "Not Nathan");
    }
}
