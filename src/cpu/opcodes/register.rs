use cpu::cpu::{CPU, ProcessorStatusFlag};

pub fn clc(cpu: &mut CPU) {
    cpu.clear_processor_status_flag(ProcessorStatusFlag::Carry);
}

pub fn cld(cpu: &mut CPU) {
    cpu.clear_processor_status_flag(ProcessorStatusFlag::Decimal);
}

pub fn cli(cpu: &mut CPU) {
    cpu.clear_processor_status_flag(ProcessorStatusFlag::Interrupt);
}

pub fn clv(cpu: &mut CPU) {
    cpu.clear_processor_status_flag(ProcessorStatusFlag::Overflow);
}

pub fn sec(cpu: &mut CPU) {
    cpu.set_processor_status_flag(ProcessorStatusFlag::Carry);
}

pub fn sed(cpu: &mut CPU) {
    cpu.set_processor_status_flag(ProcessorStatusFlag::Decimal);
}

pub fn sei(cpu: &mut CPU) {
    cpu.set_processor_status_flag(ProcessorStatusFlag::Interrupt);
}
