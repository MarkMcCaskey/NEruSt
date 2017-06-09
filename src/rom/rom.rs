pub struct ROM {
    pub data: Vec<u8>,
}

pub fn get_byte(rom: &ROM, it: &mut usize) -> u8 {
    let ret: u8 = rom.data[*it];
    *it += 1;
    ret
}

pub fn get_word(rom: &ROM, it: &mut usize) -> u16 {
    let ret: u16 = ((rom.data[*it] as u16) << 8) + rom.data[*it + 1];
    *it += 2;
    ret
}
