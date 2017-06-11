use cpu::cpu::{CPU, ProcessorStatusFlag};

pub fn clc(cpu: &mut CPU) {
    cpu.set_flag_value(ProcessorStatusFlag::Carry, false);
}

pub fn cld(cpu: &mut CPU) {
    cpu.set_flag_value(ProcessorStatusFlag::Decimal, false);
}

pub fn cli(cpu: &mut CPU) {
    cpu.set_flag_value(ProcessorStatusFlag::Interrupt, false);
}

pub fn clv(cpu: &mut CPU) {
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, false);
}

pub fn sec(cpu: &mut CPU) {
    cpu.set_flag_value(ProcessorStatusFlag::Carry, true);
}

pub fn sed(cpu: &mut CPU) {
    cpu.set_flag_value(ProcessorStatusFlag::Decimal, true);
}

pub fn sei(cpu: &mut CPU) {
    cpu.set_flag_value(ProcessorStatusFlag::Interrupt, true);
}

// $AA
pub fn tax(cpu: &mut CPU) {
    cpu.x = cpu.acc;
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

// $8A
pub fn txa(cpu: &mut CPU) {
    cpu.acc = cpu.x;
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

// $CA
pub fn dex(cpu: &mut CPU) {
    cpu.x = cpu.x.wrapping_sub(1);
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

// $E8
pub fn inx(cpu: &mut CPU) {
    cpu.x = cpu.x.wrapping_add(1);
    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

// $A8
pub fn tay(cpu: &mut CPU) {
    cpu.y = cpu.acc;
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

// $9A
pub fn tya(cpu: &mut CPU) {
    cpu.acc = cpu.y;
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

// $88
pub fn dey(cpu: &mut CPU) {
    cpu.y = cpu.y.wrapping_sub(1);
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

// $C8
pub fn iny(cpu: &mut CPU) {
    cpu.y = cpu.y.wrapping_add(1);
    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}
