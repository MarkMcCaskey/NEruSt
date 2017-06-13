use ram;
use cpu;

fn ldx_set_flags(cpu: &mut cpu::cpu::CPU) {
    let zero_flag: bool = cpu.x == 0;
    let negative_flag: bool = cpu.x & 0x80 == 0x80;
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Negative, negative_flag);
}

// A2: LDX #$aa
pub fn ldx_imm(cpu: &mut cpu::cpu::CPU, val: u8) -> u8 {
    cpu.x = val;
    ldx_set_flags(cpu);
    2
}

// A6: LDX $aa
pub fn ldx_zp(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
    cpu.x = ram.data[adr as usize];
    ldx_set_flags(cpu);
    3
}

// LDX $aa,x
// Does not exist

// B6: LDX $aa,y
pub fn ldx_zpy(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
    let indexed_adr = adr.wrapping_add(cpu.y);
    cpu.x = ram.data[indexed_adr as usize];
    ldx_set_flags(cpu);
    4
}

// LDX ($aa,x)
// Does not exist

// LDX (&aa),y
// Does not exist

// AE: LDX $aabb
pub fn ldx_abs(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) -> u8 {
    cpu.x = ram.data[adr as usize];
    ldx_set_flags(cpu);
    4
}

// LDX $aabb,x
// Does not exist

// BE: LDX $aabb,y
pub fn ldx_aby(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) -> u8 {
    let indexed_adr: u16 = adr + cpu.y as u16;
    cpu.x = ram.data[indexed_adr as usize];
    ldx_set_flags(cpu);
    4 + (indexed_adr > adr | 0x0FF) as u8
}

// LDX ($aabb)
// Does not exist
