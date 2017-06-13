use cpu::cpu::{CPU, ProcessorStatusFlag};
use ram::ram::RAM;

fn adc_and_set_flags(cpu: &mut CPU, val: u8) {
    let is_bcd = cpu.get_processor_status_flag(ProcessorStatusFlag::Decimal);
    let carry = cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;

    let new_acc = (cpu.acc as u16)
        .wrapping_add(val as u16)
        .wrapping_add(carry as u16);

    cpu.acc = new_acc as u8;

    cpu.set_flag_value(ProcessorStatusFlag::Zero, (new_acc as u8) == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, new_acc > 0xFF);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, new_acc == 0);
}
