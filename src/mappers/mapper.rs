use cartridge::*;

pub trait Mapper {
    fn get(&self, u16, &Cartridge) -> u8;
    fn set(&mut self, u16, u8, &mut Cartridge);
}
