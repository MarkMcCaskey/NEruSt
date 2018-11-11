/*
    THIS CARTRIDGE ONLY WORK FOR SMB
*/

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use getset::GetSet;

use header::*;
use memory::*;

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
        buf_reader.read(&mut header_bytes).unwrap();

        // create header
        let header = INESHeader::from(header_bytes);

        // load the mapper
        let mapper = match header.get_mapper_id() {
            0x00 => { },
            _ => panic!("Mapper for this cart has not been implemented!"),
        };

        // extract trainer if it exists (Do nothing with it for now)
        if header.contains_trainer() {
            // the hell even is a trainer??
            let mut trainer_bytes = [0u8; 512];
            buf_reader.read(&mut trainer_bytes).unwrap();
        }

        // extract PRG rom
        let mut prg_rom_bytes = vec![0u8; header.get_prg_rom_size()].into_boxed_slice();
        buf_reader.read(&mut prg_rom_bytes).unwrap();
        let prg_rom = Memory::from_boxed_slice(prg_rom_bytes);

        // extract chr rom
        let mut chr_rom_bytes = vec![0u8; header.get_chr_rom_size()].into_boxed_slice();
        buf_reader.read(&mut chr_rom_bytes).unwrap();
        let chr_rom = Memory::from_boxed_slice(chr_rom_bytes);

        // some other shit later

        // create the cart
        Self {
            header,
            prg_rom,
            chr_rom,
        }
    }
}

impl GetSet for Cartridge {
    fn get(&self, addr: u16) -> u8 {
        let wrap = match self.header.get_prg_rom_size() {
            0x4000 => 0x3FFF,
            0x8000 => 0x7FFF,
            e => panic!("Invalid header rom for this mapper: {:x}", e), // probably
        };
        match addr {
            0x4020 ..= 0x5FFF => unreachable!(), //probably
            0x6000 ..= 0x7FFF => unreachable!(), // probably
            0x8000 ..= 0xFFFF => self.prg_rom.get((addr - 0x8000) & wrap),
            e => panic!("Invalid address lookup in Cartridge: {:x}", e), // probably
        }     
    }

    fn set(&mut self, addr: u16, val: u8) {
    }
}