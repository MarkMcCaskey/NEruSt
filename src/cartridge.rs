/*
    THIS CARTRIDGE ONLY WORK FOR SMB
*/

use crate::getset::GetSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use crate::header::*;
use crate::memory::*;

pub struct Cartridge {
    pub header: INESHeader,
    pub prg_rom: Memory,
    pub chr_rom: Memory,
}

impl Cartridge {
    pub fn load_from_file(rom_name: &Path) -> Self {
        let file = File::open(rom_name).unwrap();
        let mut buf_reader = BufReader::new(file);

        // read the first 16 bytes
        let mut header_bytes = [0u8; 16];
        buf_reader.read_exact(&mut header_bytes).unwrap();

        // create header
        let header = INESHeader::from(header_bytes);

        // load the mapper
        let _mapper = match header.get_mapper_id() {
            0x00 => {}
            _ => panic!("Mapper for this cart has not been implemented!"),
        };

        // extract trainer if it exists (Do nothing with it for now)
        if header.contains_trainer() {
            // the hell even is a trainer??
            let mut trainer_bytes = [0u8; 512];
            buf_reader.read_exact(&mut trainer_bytes).unwrap();
        }

        // extract PRG rom
        let mut prg_rom_bytes = vec![0u8; header.get_prg_rom_size()].into_boxed_slice();
        buf_reader.read_exact(&mut prg_rom_bytes).unwrap();
        let prg_rom = Memory::from_boxed_slice(prg_rom_bytes);

        // extract chr rom
        let mut chr_rom_bytes = vec![0u8; header.get_chr_rom_size()].into_boxed_slice();
        buf_reader.read_exact(&mut chr_rom_bytes).unwrap();
        let chr_rom = Memory::from_boxed_slice(chr_rom_bytes);

        // some other shit later

        // create the cart
        Self {
            header,
            prg_rom,
            chr_rom,
        }
    }

    /// Get the CPU's view of the cartridge.
    pub fn cpu_view(&mut self) -> CartridgeCpuView {
        CartridgeCpuView { cart: self }
    }

    /// Get the PPU's view of the cartridge.
    pub fn ppu_view(&mut self) -> CartridgeCpuView {
        CartridgeCpuView { cart: self }
    }
}

/// The view the CPU has of the cartridge.
pub struct CartridgeCpuView<'a> {
    cart: &'a mut Cartridge,
}

impl<'a> GetSet for CartridgeCpuView<'a> {
    fn get(&self, addr: u16) -> u8 {
        let wrap = match self.cart.header.get_prg_rom_size() {
            0x4000 => 0x3FFF,
            0x8000 => 0x7FFF,
            e => panic!("Invalid header rom for this mapper: {:x}", e), // probably
        };
        match addr {
            0x4020..=0x5FFF => unreachable!(), //probably
            0x6000..=0x7FFF => unreachable!(), // probably
            0x8000..=0xFFFF => self.cart.prg_rom.get((addr - 0x8000) & wrap),
            e => panic!("Invalid address lookup in Cartridge: {:x}", e), // probably
        }
    }

    fn set(&mut self, _addr: u16, _val: u8) {}
}

/// The view the PPU has of the cartridge.
pub struct CartridgePpuView<'a> {
    cart: &'a mut Cartridge,
}

impl<'a> GetSet for CartridgePpuView<'a> {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cart.chr_rom.get(addr),
            e => panic!("Invalid address lookup in Cartridge for PPU: {:x}", e), // probably
        }
    }

    fn set(&mut self, _addr: u16, _val: u8) {}
}
