// simple abstraction for mapping
pub trait GetSet {
	fn get(&self, u16) -> u8;
	fn set(&mut self, u16, u8);
}