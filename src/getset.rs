// simple abstraction for mapping
pub trait GetSet {
    fn get(&self, _: u16) -> u8;
    fn set(&mut self, _: u16, _: u8);
}
