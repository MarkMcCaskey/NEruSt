use cpu::cpu::{CPU, ProcessorStatusFlag};
use ram::ram::RAM;

fn rol_value(cpu: &mut CPU, val: u8) -> u8 {
    let old_carry = cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
    let old_bit7 = ((val >> 7) & 1);

    let mut new_val = val << 1;
    new_val |= old_carry;

    cpu.set_flag_value(ProcessorStatusFlag::Carry, old_bit7 == 1);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_val == 0);

    new_val
}

pub fn rol_acc(cpu: &mut CPU) -> u8 {
    let val = cpu.acc;
    cpu.acc = rol_value(cpu, val);
    0
}

pub fn rol_zp(cpu: &mut CPU, ram: &mut RAM, adr: u8) -> u8 {
    let val = ram.data[adr as usize];
    ram.data[adr as usize] = rol_value(cpu, val);
    0
}

pub fn rol_zpx(cpu: &mut CPU, ram: &mut RAM, adr: u8) -> u8 {
    let val = ram.data[((adr as u16) + (cpu.x as u16)) as usize];
    ram.data[((adr as u16) + (cpu.x as u16)) as usize] = rol_value(cpu, val);
    0
}

pub fn rol_abs(cpu: &mut CPU, ram: &mut RAM, adr: u16) -> u8 {
    let val = ram.data[adr as usize];
    ram.data[adr as usize] = rol_value(cpu, val);
    0
}

pub fn rol_abx(cpu: &mut CPU, ram: &mut RAM, adr: u16) -> u8 {
    let val = ram.data[(adr + (cpu.x as u16)) as usize];
    ram.data[(adr + (cpu.x as u16)) as usize] = rol_value(cpu, val);
    0
}
