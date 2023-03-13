mod winner;
pub use winner::Winner;

use crate::contestant::Contestant;

pub trait MatchContender {
    fn contestant(&self) -> Option<Contestant>;
}

#[cfg(test)]
pub mod tests {
    use crate::contestant::Contestant;
    use crate::match_::Contenders;

    pub fn dummy_contenders() -> Contenders {
        [
            Box::new(Contestant::new(0, "Nathan".to_string())),
            Box::new(Contestant::new(1, "Not Nathan".to_string())),
        ]
    }
}
