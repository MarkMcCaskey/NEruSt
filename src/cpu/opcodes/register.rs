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
