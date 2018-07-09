use cartridge::*;
use mappers::mapper::*;

pub struct Mapper000 {}

impl Mapper000 {
    pub fn new() -> Self {
        Self { 
        }
    }
}

impl Mapper for Mapper000 {
    fn get(&self, index: u16, cart: &Cartridge) -> u8 {
        // offset the cpu address
        let local_index = index - 0x8000;

        // match the local prg rom address
        match local_index {
            // first 16kb
            0x0000..=0x3FFF => cart.prg_rom[local_index as usize],
            // second 16kb
            0x4000..=0x7FFF => match cart.header.get_prg_rom_size() {
                // If the prg rom is only 16kb, mirror
                0x4000 => cart.prg_rom[local_index as usize - 0x4000],
                // else just be normal
                0x8000 => cart.prg_rom[local_index as usize],
                //
                _ => unreachable!(),
            },
            //
            _ => unreachable!(),
        }
    }

    fn set(&mut self, _index: u16, _data: u8, _cart: &mut Cartridge) {
        // I don't think this will ever be called
        unreachable!();
    }
}