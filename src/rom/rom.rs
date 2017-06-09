struct ROM {
	data: Vec<u8>
}

fn get_byte(rom: &ROM, it: &mut usize) -> u8 {
	let ret: u8 = rom.data[*it];
	*it += 1;
	ret
}

fn get_word(rom: &ROM, it: &mut usize) -> u16 {
	let ret: u16 = (rom.data[*it] as u16) << 8 + rom.data[*it + 1];
	*it += 2;
	ret
}