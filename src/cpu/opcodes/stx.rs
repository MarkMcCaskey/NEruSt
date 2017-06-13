use ram;
use cpu;

// (imm) STX #$aa
// Does not exist

// (zp) 86: STX $aa
pub fn stx_zp(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	ram.data[adr as usize] = cpu.x;
	3
}

// (zpx) STX $aa,x
// Does not exist

// (zpy) 96: STX $00,y
pub fn stx_zpy(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	let indexed_adr: u8 = adr.wrapping_add(cpu.y);
	ram.data[indexed_adr as usize] = cpu.x;
	4
}

// (izx) STX ($aa,x)
// Does not exist

// (izy) STX ($aa),y
// Does not exist

// (abs) 8E: STX $aabb
pub fn stx_abs(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) -> u8 {
	ram.data[adr as usize] = cpu.x;
	4
}

// (abx) STX $aabb,x
// Does not exist

// (aby) STX $aabb,y
// Does not exist