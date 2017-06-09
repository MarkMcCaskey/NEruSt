use ram;
use cpu;

// STA #$aa
// Does not exist

// 85: STA $aa
pub fn sta_zp(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) {
	ram.data[adr as usize] = cpu.acc;
}

// 95: STA $aa,x
pub fn sta_zpx(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) {
	ram.data[(adr + cpu.x) as usize] = cpu.acc;
}

// STA $00,y
// Does not exist

// 81: STA ($aa,x)
pub fn sta_izx(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) {
	// I'm not entirely sure how to implement this one... and I've never really 
	// seen anyone use it... so for now, we'll just yolo it
	println!("The dumbass devs never implemented sta izx.");
}

// 91: STA ($aa),y
pub fn sta_izy(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u8) {
	let temp : usize = (ram.data[adr as usize] + cpu.y) as usize;
	ram.data[temp] = cpu.acc;
}

// 8D: STA $aabb

// 7D: STA $aabb
pub fn sta_abs(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) {
	ram.data[adr as usize] = cpu.acc;
}

// 9D: STA $aabb,x
pub fn sta_abx(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) {
	ram.data[(adr + cpu.x as u16) as usize] = cpu.acc;
}

// 99: STA $aabb,y
pub fn sta_aby(cpu: &cpu::cpu::CPU, ram: &mut ram::ram::RAM, adr: u16) {
	ram.data[(adr + cpu.y as u16) as usize] = cpu.acc;
}