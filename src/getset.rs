// simple abstraction for mapping
pub trait GetSet {
    /// Read 1 byte from the given address.
    fn get(&self, address: u16) -> u8;

    /// Read 2 bytes from the given address.
    fn get_16(&self, address: u16) -> u16 {
        ((self.get(address + 1) as u16) << 8) | self.get(address) as u16
    }

    /// Write 1 byte to the memory at the given address..
    fn set(&mut self, address: u16, value: u8);
}

impl<T: GetSet> GetSet for &mut T {
    fn get(&self, address: u16) -> u8 {
        (*self).get(address)
    }

    fn set(&mut self, address: u16, value: u8) {
        (*self).set(address, value)
    }
}
