use cpu::cpu::{CPU, ProcessorStatusFlag};
use ram::ram::RAM;

fn set_acc_and_conditionally_zero_flag(cpu: &mut CPU, val: u8) {
    cpu.acc &= val;
    let new_zero_flag_value = cpu.acc == 0;

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_zero_flag_value)
}

// $29
pub fn and_imm(cpu: &mut CPU, val: u8) -> u8 {
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}

// $25
pub fn and_zp(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[adr as usize];
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}

// $35
pub fn and_zpx(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[((adr as u16) + (cpu.x as u16)) as usize];
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}

// $2D
pub fn and_abs(cpu: &mut CPU, ram: &RAM, adr: u16) -> u8 {
    let val = ram.data[adr as usize];
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}

// $3D
pub fn and_abx(cpu: &mut CPU, ram: &RAM, adr: u16) -> u8 {
    let val = ram.data[(adr + (cpu.x as u16)) as usize];
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}

// $39
pub fn and_aby(cpu: &mut CPU, ram: &RAM, adr: u16) -> u8 {
    let val = ram.data[(adr + (cpu.y as u16)) as usize];
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}

// $21
pub fn and_izx(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[(ram.data[((adr as u16) + (cpu.x as u16)) as usize]) as usize];
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}

// $31
pub fn and_izy(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[(ram.data[adr as usize] + cpu.y) as usize];
    set_acc_and_conditionally_zero_flag(cpu, val);
    0
}
