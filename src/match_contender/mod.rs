pub mod new_contestant;
mod winner;

use crate::contestant::Contestant;
use std::rc::Rc;

pub trait MatchContender {
    fn contestant(&self) -> Option<Rc<Contestant>>;
}
