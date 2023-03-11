use crate::contestant;

pub trait MatchContender {
    fn contestant_id(&self) -> Option<contestant::Id>;
}
