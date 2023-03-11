mod new_contestant;

pub type Id = u32;

pub trait Contestant {
    fn id(&self) -> &Id;
    fn name(&self) -> &String;
    fn set_name(&mut self, name: String);
}
