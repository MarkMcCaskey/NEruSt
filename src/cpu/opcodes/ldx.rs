use ram;
use cpu;

fn ldx_set_flags(cpu: &mut cpu::cpu::CPU) {
    let zero_flag: bool = cpu.x == 0;
    let negative_flag: bool = cpu.x & 0x80 == 0x80;
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Negative, negative_flag);
}

// A2: LDY #$aa
pub fn ldx_imm(cpu: &mut cpu::cpu::CPU, val: u8) {
    cpu.x = val;
    ldx_set_flags(cpu);
}

// A7: LDY $aa
pub fn ldx_zp(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
    cpu.x = ram.data[adr as usize];
    ldx_set_flags(cpu);
}

// LDY $aa,x
// Does not exist

// LDY $aa,y
pub fn ldx_zpy(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) {
    cpu.x = ram.data[adr.wrapping_add(cpu.y) as usize];
    ldx_set_flags(cpu);
}

// LDY ($aa,x)
// Does not exist

// LDY (&aa),y
// Does not exist

// AE: LDY $aabb
pub fn ldx_abs(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
    cpu.x = ram.data[adr as usize];
    ldx_set_flags(cpu);
}

// LDY $aabb,x
// Does not exist

// BE: LDY $aabb,y
pub fn ldx_aby(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) {
    cpu.x = ram.data[adr.wrapping_add(cpu.y as u16) as usize];
    ldx_set_flags(cpu);
}

// LDY ($aabb)
// Does not exist
