use ram;
use cpu;

fn lda_set_flags(cpu: &mut cpu::cpu::CPU) {
    let zero_flag: bool = cpu.acc == 0;
    let negative_flag: bool = cpu.acc & 0x80 == 0x80;
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Negative, negative_flag);
}

// A9: LDA #$aa
pub fn lda_imm(cpu: &mut cpu::cpu::CPU, val: u8) {
    cpu.acc = val;
    lda_set_flags(cpu);
}

// A5: LDA $aa
pub fn lda_zp(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
    cpu.acc = ram.data[adr as usize];
    lda_set_flags(cpu);
}

// B5: LDA $aa,x
pub fn lda_zpx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
    cpu.acc = ram.data[adr.wrapping_add(cpu.x) as usize];
    lda_set_flags(cpu);
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
    // get the 2 byte value at the address (Yes, it will not cross page boundries)
    let val: u16 = (ram.data[adr as usize] as u16) << 8 | ram.data[adr.wrapping_add(1) as usize] as u16;
    // treat it like an aby now
    cpu.acc = ram.data[(val + cpu.y as u16) as usize];
    lda_set_flags(cpu);
}

// AD: LDA $aabb
pub fn lda_abs(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
    cpu.acc = ram.data[adr as usize];
    lda_set_flags(cpu);
}

// BD: LDA $aabb,x
pub fn lda_abx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
    cpu.acc = ram.data[adr.wrapping_add(cpu.x as u16) as usize];
    lda_set_flags(cpu);
}

// B9: LDA $aabb,y
pub fn lda_aby(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
    cpu.acc = ram.data[adr.wrapping_add(cpu.y as u16) as usize];
    lda_set_flags(cpu);
}

// LDA ($aabb)
// Does not exist
