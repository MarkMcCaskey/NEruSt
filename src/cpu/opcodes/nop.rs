use ram;
use cpu;

// (imp) 1A, 3A, 5A, 7A, DA, EA, FA: [NOP]
pub fn nop_imp(cpu: &cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
	2
}

// (imm) 80, 83, C3, E3, 89: [NOP #$aa]
pub fn nop_imm(cpu: &cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
	2
}

// (zp) 04, 44, 64: [NOP $aa]
pub fn nop_zp(cpu: &cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
	3
}

// (zpx) 14, 34, 54, 74, D4, F4: [NOP $aa,x]
pub fn nop_zpx(cpu: &cpu::cpu::CPU, &ram::ram::RAM, adr: u8) -> u8 {
	4
}

// (izx) NOP ($aa,x)
// Does not exist

// (izy) NOP ($aa),y
// Does not exist

// (abs) 0C: [NOP $aabb]
pub fn nop_abs(cpu: &cpu::cpu::CPU, &ram::ram::RAM, adr: u16) -> u8 {
	4
}

// (abx) 1C, 3C, 5C, 7C, DC, FC: [NOP $aabb,x]
pub fn nop_abx(cpu: &cpu::cpu::CPU, &ram::ram::RAM, adr: u16) -> u8 {
	let indexed_adr: u16 = adr + cpu.x as u16;
	4 + (indexed_adr > adr | 0x0FF) as u8
}

// (aby) [NOP $aabb,y]
// Does not exist