use cpu::cpu::{CPU, ProcessorStatusFlag};
use ram::ram::RAM;

fn shift_and_conditionally_set_carry_and_zero(cpu: &mut CPU, val: u8) -> u8 {
    let old_bit7 = val & (0x80);

    //conditionally set or clear carry flag
    if old_bit7 == 0x80 {
        cpu.set_processor_status_flag(ProcessorStatusFlag::Carry)
    } else {
        cpu.clear_processor_status_flag(ProcessorStatusFlag::Carry)
    }

    let new_val = val << 1;

    //conditionally set or clear zero flag
    if new_val == 0 {
        cpu.set_processor_status_flag(ProcessorStatusFlag::Zero)
    } else {
        cpu.clear_processor_status_flag(ProcessorStatusFlag::Zero)
    }

    new_val
}

// $0A
pub fn asl_acc(cpu: &mut CPU) {
    let val = cpu.acc;
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    cpu.acc = new_val;
}

// $06
pub fn asl_zp(cpu: &mut CPU, ram: &mut RAM, adr: u8) {
    let val = ram.data[adr as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[adr as usize] = new_val;
}

// $16
pub fn asl_zpx(cpu: &mut CPU, ram: &mut RAM, adr: u8) {
    let val = ram.data[(adr + cpu.x) as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[(adr + cpu.x) as usize] = new_val;
}

// $0E
pub fn asl_abs(cpu: &mut CPU, ram: &mut RAM, adr: u16) {
    let val = ram.data[adr as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[adr as usize] = new_val;
}

// $1E
pub fn asl_abx(cpu: &mut CPU, ram: &mut RAM, adr: u16) {
    let val = ram.data[(adr + (cpu.x as u16)) as usize];
    let new_val = shift_and_conditionally_set_carry_and_zero(cpu, val);

    ram.data[(adr + (cpu.x as u16)) as usize] = new_val;
}
