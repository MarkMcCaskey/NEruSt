/*use ram::ram::RAM;
use cpu::cpu::CPU;*/
use ram;
use cpu;

// A9: LDA #$aa
pub fn lda_imm(cpu: &mut cpu::cpu::CPU, val: u8) {
	cpu.acc = val;
}

// A5: LDA $aa
pub fn lda_zp(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
	cpu.acc = ram.data[adr as usize];
}

// AD: LDA $aabb
pub fn lda_abs(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
	cpu.acc = ram.data[adr as usize];
}

// B5: LDA $aa,x
pub fn lda_zpx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
	cpu.acc = ram.data[(adr + cpu.x) as usize]; // ?
}

// LDA $aa,y
// Does not exist

// A1: LDA ($aa,x)
pub fn lda_izx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
	// I'm not entirely sure how to implement this one... and I've never really 
	// seen anyone use it... so for now, we'll just yolo it
	println!("The dumbass devs never implemented lda izx.");
}

// B1: LDA (&aa),y
pub fn lda_izy(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
	cpu.acc = ram.data[(ram.data[adr as usize] + cpu.y) as usize];
}

// BD: LDA $aabb,x
pub fn lda_abx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
	cpu.acc = ram.data[(adr + cpu.x as u16) as usize]; // ?
}

// B9: LDA $aabb,y
pub fn lda_aby(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
	cpu.acc = ram.data[(adr + cpu.y as u16) as usize]; // ?
}

// LDA ($aa)
// Does not exist