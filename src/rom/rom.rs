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
