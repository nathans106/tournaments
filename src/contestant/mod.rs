mod new_contestant;
mod new_contestant_factory;

pub type Id = u32;

pub trait Contestant {
    fn id(&self) -> &Id;
    fn name(&self) -> &String;
    fn set_name(&mut self, name: String);
}
