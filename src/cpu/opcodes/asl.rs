use cpu::cpu::{CPU, ProcessorStatusFlag};
use ram::ram::RAM;

fn shift_and_conditionally_set_carry_and_zero(cpu: &mut CPU, val: u8) -> u8 {
    let old_bit7 = val & (0x80);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, old_bit7 == 0x80);

    let new_val = val << 1;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_val == 0);

    new_val
}

// $0A
pub fn asl_acc(cpu: &mut CPU) -> u8 {
    let val = cpu.acc;
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    cpu.acc = new_val;
    0
}

// $06
pub fn asl_zp(cpu: &mut CPU, ram: &mut RAM, adr: u8) -> u8 {
    let val = ram.data[adr as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[adr as usize] = new_val;
    0
}

// $16
pub fn asl_zpx(cpu: &mut CPU, ram: &mut RAM, adr: u8) -> u8 {
    let val = ram.data[(adr + cpu.x) as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[(adr + cpu.x) as usize] = new_val;
    0
}

// $0E
pub fn asl_abs(cpu: &mut CPU, ram: &mut RAM, adr: u16) -> u8 {
    let val = ram.data[adr as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[adr as usize] = new_val;
    0
}

// $1E
pub fn asl_abx(cpu: &mut CPU, ram: &mut RAM, adr: u16) -> u8 {
    let val = ram.data[(adr + (cpu.x as u16)) as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[(adr + (cpu.x as u16)) as usize] = new_val;
    0
}
