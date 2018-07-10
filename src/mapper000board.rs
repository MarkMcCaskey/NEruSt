use header::*;
use memory::*;
use cartridge::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Mapper000 supports 16k or 32k prg rom, and 8k chr rom
pub struct Mapper000Board {
    header: INESHeader,
    prg_rom: Memory,
    chr_rom: Memory,
}

impl Mapper000Board {
    pub fn new(header: INESHeader, mut buf_reader: BufReader<File>) -> Self {
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

        Self {
            header,
            prg_rom,
            chr_rom,
        }
    }
}

impl Cartridge for Mapper000Board {
    fn get(&self, index: u16) -> u8 {
        let local_index = index - 0x8000;
        match local_index {
            // first 16kb
            0x0000..=0x3FFF => self.prg_rom[local_index as usize],
            // second 16kb
            0x4000..=0x7FFF => match self.header.get_prg_rom_size() {
                // if the prg rom is only 16kb, mirror the first 16kb
                0x4000 => self.prg_rom[local_index as usize - 0x4000],
                // else just be normal
                0x8000 => self.prg_rom[local_index as usize],
                //
                _ => unreachable!(),
            }
            //
            _ => unreachable!(),
        }
    }

    fn set(&mut self, _index: u16, _data: u8) {
        unreachable!();
    }
}