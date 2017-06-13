use ram;
use cpu;

// (imm) STY #$aa
// Does not exist

// (zp) 84: STY $aa
pub fn sty_zp(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	ram.data[adr as usize] = cpu.y;
	3
}

// (zpx) 94: STY $aa,x
pub fn sty_zpx(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	let indexed_adr: u8 = adr.wrapping_add(cpu.x);
	ram.data[indexed_adr as usize] = cpu.y;
	4
}

// (zpy) STY $00,y
// Does not exist

// (izx) STY ($aa,x)
// Does not exist

// (izy) STY ($aa),y
// Does not exist

// (abs) 8C: STY $aabb
pub fn sty_abs(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) -> u8 {
	ram.data[adr as usize] = cpu.y;
	4
}

// (abx) STY $aabb,x
// Does not exist

// (aby) STY $aabb,y
// Does not exist