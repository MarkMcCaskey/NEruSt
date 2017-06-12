use cpu::cpu::{CPU, ProcessorStatusFlag};
use ram::ram::RAM;

fn set_acc_ora_conditionally_zero_flag(cpu: &mut CPU, val: u8) {
    cpu.acc |= val;
    let new_zero_flag_value = cpu.acc == 0;

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_zero_flag_value)
}

// $09
pub fn ora_imm(cpu: &mut CPU, val: u8) -> u8 {
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}

// $05
pub fn ora_zp(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[adr as usize];
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}

// $15
pub fn ora_zpx(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[((adr as u16) + (cpu.x as u16)) as usize];
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}

// $0D
pub fn ora_abs(cpu: &mut CPU, ram: &RAM, adr: u16) -> u8 {
    let val = ram.data[adr as usize];
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}

// $1D
pub fn ora_abx(cpu: &mut CPU, ram: &RAM, adr: u16) -> u8 {
    let val = ram.data[(adr + (cpu.x as u16)) as usize];
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}

// $19
pub fn ora_aby(cpu: &mut CPU, ram: &RAM, adr: u16) -> u8 {
    let val = ram.data[(adr + (cpu.y as u16)) as usize];
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}

// $01
pub fn ora_izx(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[(ram.data[((adr as u16) + (cpu.x as u16)) as usize]) as usize];
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}

// $11
pub fn ora_izy(cpu: &mut CPU, ram: &RAM, adr: u8) -> u8 {
    let val = ram.data[(ram.data[adr as usize] + cpu.y) as usize];
    set_acc_ora_conditionally_zero_flag(cpu, val);
    0
}
