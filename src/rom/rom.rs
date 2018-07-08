use std::{fs::File, io::Read, path::PathBuf};

// Copying from GB; no idea if this is a good default
const DEFAULT_ROM_SIZE: usize = 0x4000;

pub struct Rom {
    pub data: Vec<u8>,
}

pub fn get_byte(rom: &Rom, it: &mut u16) -> u8 {
    let ret: u8 = rom.data[*it as usize];
    *it += 1;
    ret
}

pub fn get_word(rom: &Rom, it: &mut u16) -> u16 {
    let ret: u16 = (rom.data[*it as usize] as u16) << 8 | (rom.data[*it as usize + 1] as u16);
    *it += 2;
    ret
}

impl Rom {
    pub fn from_file(rom_location: PathBuf) -> Result<Self, ::std::io::Error> {
        let mut output = Vec::with_capacity(DEFAULT_ROM_SIZE);
        let mut rom = File::open(rom_location)?;
        let _ = rom.read_to_end(&mut output);

        Ok(Self { data: output })
    }
}
