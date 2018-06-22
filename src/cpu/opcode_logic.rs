use cpu::addressing_modes::OpcodeOperand;
use cpu::cpu::{Cpu, ProcessorStatusFlag};
use ram::ram::Ram;

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Load register opcodes
pub fn lda(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc = val,
        OpcodeOperand::Address(adr) => cpu.acc = ram.data[adr as usize],
    };

    let zero_flag: bool = cpu.acc == 0;
    let negative_flag: bool = cpu.acc & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldx(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.x = val,
        OpcodeOperand::Address(adr) => cpu.x = ram.data[adr as usize],
    };

    let zero_flag: bool = cpu.x == 0;
    let negative_flag: bool = cpu.x & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldy(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.y = val,
        OpcodeOperand::Address(adr) => cpu.y = ram.data[adr as usize],
    };

    let zero_flag: bool = cpu.y == 0;
    let negative_flag: bool = cpu.y & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Set Ram opcodes
pub fn sta(cpu: &Cpu, ram: &mut Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => ram.data[adr as usize] = cpu.acc,
    };

    // no flags to be set
}

pub fn stx(cpu: &Cpu, ram: &mut Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => ram.data[adr as usize] = cpu.x,
    };

    // no flags to be set
}

pub fn sty(cpu: &Cpu, ram: &mut Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => ram.data[adr as usize] = cpu.y,
    };

    // no flags to be set
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Bitwise opcodes
pub fn eor(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc ^= val,
        OpcodeOperand::Address(adr) => cpu.acc ^= ram.data[adr as usize],
    };

    let zero_flag = cpu.acc == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
}

pub fn and(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc &= val,
        OpcodeOperand::Address(adr) => cpu.acc &= ram.data[adr as usize],
    };

    let zero_flag = cpu.acc == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
}

pub fn ora(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc |= val,
        OpcodeOperand::Address(adr) => cpu.acc |= ram.data[adr as usize],
    };

    let zero_flag = cpu.acc == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Set/clear flag opcodes
pub fn clc(cpu: &mut Cpu) {
    cpu.set_flag_value(ProcessorStatusFlag::Carry, false);
}

pub fn cld(cpu: &mut Cpu) {
    cpu.set_flag_value(ProcessorStatusFlag::Decimal, false);
}

pub fn cli(cpu: &mut Cpu) {
    cpu.set_flag_value(ProcessorStatusFlag::Interrupt, false);
}

pub fn clv(cpu: &mut Cpu) {
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, false);
}

pub fn sec(cpu: &mut Cpu) {
    cpu.set_flag_value(ProcessorStatusFlag::Carry, true);
}

pub fn sed(cpu: &mut Cpu) {
    cpu.set_flag_value(ProcessorStatusFlag::Decimal, true);
}

pub fn sei(cpu: &mut Cpu) {
    cpu.set_flag_value(ProcessorStatusFlag::Interrupt, true);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Transfer opcodes
pub fn tax(cpu: &mut Cpu) {
    cpu.x = cpu.acc;

    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

pub fn txa(cpu: &mut Cpu) {
    cpu.acc = cpu.x;

    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

pub fn tay(cpu: &mut Cpu) {
    cpu.y = cpu.acc;

    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

pub fn tya(cpu: &mut Cpu) {
    cpu.acc = cpu.y;

    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Increment opcodes
pub fn dex(cpu: &mut Cpu) {
    cpu.x = cpu.x.wrapping_sub(1);

    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

pub fn inx(cpu: &mut Cpu) {
    cpu.x = cpu.x.wrapping_add(1);

    let is_zero = cpu.x == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

pub fn dey(cpu: &mut Cpu) {
    cpu.y = cpu.y.wrapping_sub(1);

    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

pub fn iny(cpu: &mut Cpu) {
    cpu.y = cpu.y.wrapping_add(1);

    let is_zero = cpu.y == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Math opcodes
fn adc(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    let old_acc: u8 = cpu.acc;
    let val: u8 = match opop {
        OpcodeOperand::Implied => {
            unreachable!();
            0
        }
        OpcodeOperand::Immediate(val) => {
            cpu.acc.wrapping_add(val);
            val
        }
        OpcodeOperand::Address(adr) => {
            cpu.acc.wrapping_add(ram.data[adr as usize]);
            ram.data[adr as usize]
        }
    };

    let negative_flag = cpu.acc & 0x80 == 0x80;
    let overflow_flag =
        (!old_acc & !val & cpu.acc & 0x80) == 0x80 || (old_acc & val & !cpu.acc & 0x80) == 0x80;
    let carry_flag = cpu.acc < old_acc;
    let zero_flag = cpu.acc == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, overflow_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
}

fn sbc(cpu: &mut Cpu, ram: &Ram, opop: OpcodeOperand) {
    let old_acc: u8 = cpu.acc;
    let val: u8 = match opop {
        OpcodeOperand::Implied => {
            unreachable!();
            0
        }
        OpcodeOperand::Immediate(val) => {
            cpu.acc.wrapping_sub(val);
            val
        }
        OpcodeOperand::Address(adr) => {
            cpu.acc.wrapping_sub(ram.data[adr as usize]);
            ram.data[adr as usize]
        }
    };

    let negative_flag = cpu.acc & 0x80 == 0x80;
    let overflow_flag =
        (!old_acc & !val & cpu.acc & 0x80) == 0x80 || (old_acc & val & !cpu.acc & 0x80) == 0x80;
    let carry_flag = cpu.acc > old_acc;
    let zero_flag = cpu.acc == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, overflow_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Shift opcodes
fn rol(cpu: &mut Cpu, ram: &mut Ram, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc << 1 | cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(val) => {
            unreachable!();
            (0, 0)
        }
        OpcodeOperand::Address(adr) => {
            let old_val = ram.data[adr as usize];
            ram.data[adr as usize] << 1
                | cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            (old_val, ram.data[adr as usize])
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

fn ror(cpu: &mut Cpu, ram: &mut Ram, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc >> 1 | cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(val) => {
            unreachable!();
            (0, 0)
        }
        OpcodeOperand::Address(adr) => {
            let old_val = ram.data[adr as usize];
            ram.data[adr as usize] >> 1
                | ((cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8) << 7);
            (old_val, ram.data[adr as usize])
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

fn lsr(cpu: &mut Cpu, ram: &mut Ram, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc >> 1;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(val) => {
            unreachable!();
            (0, 0)
        }
        OpcodeOperand::Address(adr) => {
            let old_val = ram.data[adr as usize];
            ram.data[adr as usize] >> 1;
            (old_val, ram.data[adr as usize])
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

fn asl(cpu: &mut Cpu, ram: &mut Ram, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc << 1;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(val) => {
            unreachable!();
            (0, 0)
        }
        OpcodeOperand::Address(adr) => {
            let old_val = ram.data[adr as usize];
            ram.data[adr as usize] << 1;
            (old_val, ram.data[adr as usize])
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}
