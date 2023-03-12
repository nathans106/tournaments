pub mod new_contestant;
mod winner;

use crate::contestant::Contestant;
use std::rc::Rc;

pub trait MatchContender {
    fn contestant(&self) -> Option<Rc<Contestant>>;
}

#[cfg(test)]
pub mod tests {
    use crate::contestant::Contestant;
    use crate::match_::Contenders;
    use crate::match_contender::new_contestant::NewContestant;

    pub fn dummy_contenders() -> Contenders {
        [
            Box::new(NewContestant::new(Contestant::new(0, "Nathan".to_string()))),
            Box::new(NewContestant::new(Contestant::new(
                1,
                "Not Nathan".to_string(),
            ))),
        ]
    }
}
