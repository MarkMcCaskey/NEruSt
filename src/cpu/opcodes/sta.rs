use ram;
use cpu;

// (imm) STA #$aa
// Does not exist

// (zp) 85: STA $aa
pub fn sta_zp(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	ram.data[adr as usize] = cpu.acc;
	3
}

// (zpx) 95: STA $aa,x
pub fn sta_zpx(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	let indexed_adr: u8 = adr.wrapping_add(cpu.x);
	ram.data[indexed_adr as usize] = cpu.acc;
	4
}

// (zpy) STA $00,y
// Does not exist

// (izx) 81: STA ($aa,x)
pub fn sta_izx(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	let indexed_adr: u8 = adr.wrapping_add(cpu.x);
	let derefed_indexed_adr: u16 = (ram.data[indexed_adr as usize] as u16) << 8 | ram.data[indexed_adr.wrapping_add(1) as usize] as u16;
	ram.data[derefed_indexed_adr as usize] = cpu.acc;
	6
}

// (izy) 91: STA ($aa),y
pub fn sta_izy(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) -> u8 {
	let derefed_adr: u16 = (ram.data[adr as usize] as u16) << 8 | ram.data[adr.wrapping_add(1) as usize] as u16;
	let indexed_derefed_adr: u16 = derefed_adr + cpu.y as u16; 
	ram.data[indexed_derefed_adr as usize] = cpu.acc;
	6
}

// (abs) 7D: STA $aabb
pub fn sta_abs(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) -> u8 {
	ram.data[adr as usize] = cpu.acc;
	4
}

// (abx) 9D: STA $aabb,x
pub fn sta_abx(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) -> u8 {
	ram.data[(adr + cpu.x as u16) as usize] = cpu.acc;
	5
}

// (aby) 99: STA $aabb,y
pub fn sta_aby(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) -> u8 {
	ram.data[(adr + cpu.y as u16) as usize] = cpu.acc;
	5
}