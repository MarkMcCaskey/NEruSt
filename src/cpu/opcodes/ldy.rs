use ram;
use cpu;

fn ldy_set_flags(cpu: &mut cpu::cpu::CPU) {
    let zero_flag: bool = cpu.y == 0;
    let negative_flag: bool = cpu.y & 0x80 == 0x80;
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Negative, negative_flag);
}

// A0: LDY #$aa
pub fn ldy_imm(cpu: &mut cpu::cpu::CPU, val: u8) {
    cpu.y = val;
    ldy_set_flags(cpu);
}

// A4: LDY $aa
pub fn ldy_zp(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
    cpu.y = ram.data[adr as usize];
    ldy_set_flags(cpu);
}

// B5: LDY $aa,x
pub fn ldy_zpx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
    cpu.y = ram.data[adr.wrapping_add(cpu.x) as usize];
    ldy_set_flags(cpu);
}

// LDY $aa,y
// Does not exist

// LDY ($aa,x)
// Does not exist

// LDY (&aa),y
// Does not exist

// AC: LDY $aabb
pub fn ldy_abs(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
    cpu.y = ram.data[adr as usize];
    ldy_set_flags(cpu);
}

// BC: LDY $aabb,x
pub fn ldy_abx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
    cpu.y = ram.data[adr.wrapping_add(cpu.x as u16) as usize];
    ldy_set_flags(cpu);
}

// BE: LDY $aabb,y
// Does not exist

// LDY ($aabb)
// Does not exist