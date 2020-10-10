/*
    THIS CARTRIDGE ONLY WORK FOR SMB
*/

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use crate::header::*;

pub struct Cartridge {
    pub header: INESHeader,
    pub prg_rom: Box<[u8]>,
    pub chr_rom: Box<[u8]>,
}

impl Cartridge {
    pub fn load_from_file(rom_name: &Path) -> Self {
        let file = File::open(rom_name).unwrap();
        let mut buf_reader = BufReader::new(file);

        Self::load_from_bytes(buf_reader)
    }

    pub fn load_from_bytes<R: Read>(mut buf_reader: BufReader<R>) -> Self {
        // read the first 16 bytes
        let mut header_bytes = [0u8; 16];
        buf_reader
            .read_exact(&mut header_bytes)
            .expect("read header");

        // create header
        let header = INESHeader::from(header_bytes);

        // load the mapper
        let _mapper = match header.get_mapper_id() {
            0x00 => {}
            _ => panic!("Mapper for this cart has not been implemented!"),
        };

        // extract trainer if it exists (Do nothing with it for now)
        if header.contains_trainer() {
            // what even is a trainer??
            let mut trainer_bytes = [0u8; 512];
            buf_reader
                .read_exact(&mut trainer_bytes)
                .expect("read trainer");
        }

        // extract PRG rom
        let mut prg_rom_bytes = vec![0u8; header.get_prg_rom_size()].into_boxed_slice();
        buf_reader
            .read_exact(&mut prg_rom_bytes)
            .expect("read prg rom");
        let prg_rom = prg_rom_bytes;

        // extract chr rom
        let mut chr_rom_bytes = vec![0u8; header.get_chr_rom_size()].into_boxed_slice();
        buf_reader
            .read_exact(&mut chr_rom_bytes)
            .expect("chr rom bytes");
        let chr_rom = chr_rom_bytes;

        // more later

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
    pub fn ppu_view(&mut self) -> CartridgePpuView {
        CartridgePpuView { cart: self }
    }
}

/// The view the CPU has of the cartridge.
pub struct CartridgeCpuView<'a> {
    cart: &'a mut Cartridge,
}

impl<'a> CartridgeCpuView<'a> {
    pub fn get(&mut self, addr: u16) -> u8 {
        let wrap = (self.cart.header.get_prg_rom_size() as usize)
            .checked_sub(1)
            .unwrap_or(0);
        match addr {
            0x4020..=0x5FFF => unreachable!("cart CPU view 0x4020..=0x5FFF"),
            0x6000..=0x7FFF => unreachable!("cart CPU view 0x6000..=0x7FFF"),
            0x8000..=0xFFFF => self.cart.prg_rom[(addr - 0x8000) as usize & wrap],
            e => panic!("Invalid address lookup in Cartridge for CPU: {:x}", e),
        }
    }

    pub fn set(&mut self, _addr: u16, _val: u8) {
        unreachable!()
    }
}

/// The view the PPU has of the cartridge.
pub struct CartridgePpuView<'a> {
    cart: &'a mut Cartridge,
}

impl<'a> CartridgePpuView<'a> {
    pub fn get(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cart.chr_rom[addr as usize],
            e => unreachable!("Invalid address lookup in Cartridge for PPU: {:x}", e),
        }
    }

    pub fn set(&mut self, _addr: u16, _val: u8) {
        unreachable!()
    }
}
