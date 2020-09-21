// simple abstraction for mapping
pub trait GetSet {
    /// Read 1 byte from the given address.
    fn get(&self, address: u16) -> u8;

    /// Write 1 byte to the memory at the given address..
    fn set(&mut self, address: u16, value: u8);
}
