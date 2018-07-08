use cartridge::*;

pub trait Mapper {
    fn get(u16, &Cartridge) -> u8;
    fn set(u16, u8, &mut Cartridge);
}
