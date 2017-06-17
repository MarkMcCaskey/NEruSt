use cpu::cpu::{CPU, ProcessorStatusFlag};

pub fn clc(cpu: &mut CPU) -> u8 {
    cpu.set_flag_value(ProcessorStatusFlag::Carry, false);
    2
}

pub fn cld(cpu: &mut CPU) -> u8 {
    cpu.set_flag_value(ProcessorStatusFlag::Decimal, false);
    2
}

pub fn cli(cpu: &mut CPU) -> u8 {
    cpu.set_flag_value(ProcessorStatusFlag::Interrupt, false);
    2
}

pub fn clv(cpu: &mut CPU) -> u8 {
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, false);
    2
}

pub fn sec(cpu: &mut CPU) -> u8 {
    cpu.set_flag_value(ProcessorStatusFlag::Carry, true);
    2
}

pub fn sed(cpu: &mut CPU) -> u8 {
    cpu.set_flag_value(ProcessorStatusFlag::Decimal, true);
    2
}

pub fn sei(cpu: &mut CPU) -> u8 {
    cpu.set_flag_value(ProcessorStatusFlag::Interrupt, true);
    2
}

// $AA
pub fn tax(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.acc;
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}

// $8A
pub fn txa(cpu: &mut CPU) -> u8 {
    cpu.acc = cpu.x;
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}

// $CA
pub fn dex(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.x.wrapping_sub(1);
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}

// $E8
pub fn inx(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.x.wrapping_add(1);
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}

// $A8
pub fn tay(cpu: &mut CPU) -> u8 {
    cpu.y = cpu.acc;
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}

// $9A
pub fn tya(cpu: &mut CPU) -> u8 {
    cpu.acc = cpu.y;
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}

// $88
pub fn dey(cpu: &mut CPU) -> u8 {
    cpu.y = cpu.y.wrapping_sub(1);
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}

// $C8
pub fn iny(cpu: &mut CPU) -> u8 {
    cpu.y = cpu.y.wrapping_add(1);
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    2
}
