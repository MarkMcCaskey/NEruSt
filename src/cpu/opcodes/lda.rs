use ram;
use cpu;

fn lda_set_flags(cpu: &mut cpu::cpu::CPU) {
    let zero_flag: bool = cpu.acc == 0;
    let negative_flag: bool = cpu.acc & 0x80 == 0x80;
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(cpu::cpu::ProcessorStatusFlag::Negative, negative_flag);
}

// A9: LDA #$aa
pub fn lda_imm(cpu: &mut cpu::cpu::CPU, val: u8) -> u8 {
    cpu.acc = val;
    lda_set_flags(cpu);
    2
}

// A5: LDA $aa
pub fn lda_zp(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
    cpu.acc = ram.data[adr as usize];
    lda_set_flags(cpu);
    3
}

// B5: LDA $aa,x
pub fn lda_zpx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
    let indexed_adr: u8 = adr.wrapping_add(cpu.x);
    cpu.acc = ram.data[indexed_adr as usize];
    lda_set_flags(cpu);
    4
}

// LDA $aa,y
// Does not exist

// A1: LDA ($aa,x)
pub fn lda_izx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
    let indexed_adr: u8 = adr.wrapping_add(cpu.x);
    let derefed_indexed_adr: u16 = (ram.data[indexed_adr as usize] as u16) << 8 | ram.data[indexed_adr.wrapping_add(1) as usize] as u16;
    cpu.acc = ram.data[derefed_indexed_adr as usize];
    lda_set_flags(cpu);
    6
}

// B1: LDA ($aa),y
pub fn lda_izy(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u8) -> u8 {
    let derefed_adr: u16 = (ram.data[adr as usize] as u16) << 8 | ram.data[adr.wrapping_add(1) as usize] as u16;
    let indexed_derefed_adr: u16 = derefed_adr + cpu.y as u16;
    cpu.acc = ram.data[indexed_derefed_adr as usize];
    lda_set_flags(cpu);
    5 + (indexed_derefed_adr > derefed_adr | 0x0FF) as u8
}

// AD: LDA $aabb
pub fn lda_abs(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) -> u8 {
    cpu.acc = ram.data[adr as usize];
    lda_set_flags(cpu);
    4
}

// BD: LDA $aabb,x
pub fn lda_abx(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) -> u8 {
    let indexed_adr: u16 = adr + cpu.x as u16;
    cpu.acc = ram.data[indexed_adr as usize];
    lda_set_flags(cpu);
    4 + (indexed_adr > adr | 0x0FF) as u8
}

// B9: LDA $aabb,y
pub fn lda_aby(cpu: &mut cpu::cpu::CPU, ram: &ram::ram::RAM, adr: u16) -> u8 {
    let indexed_adr: u16 = adr + cpu.y as u16;
    cpu.acc = ram.data[indexed_adr as usize];
    lda_set_flags(cpu);
    4 + (indexed_adr > adr | 0x0FF) as u8
}

// LDA ($aabb)
// Does not exist