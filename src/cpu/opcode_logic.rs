use crate::cpu::addressing_modes::OpcodeOperand;
use crate::cpu::cpu::{Cpu, ProcessorStatusFlag};
use crate::getset::GetSet;

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Load register opcodes
pub fn lda(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc = val,
        OpcodeOperand::Address(adr) => cpu.acc = cpu_map.get(adr),
    };

    let zero_flag: bool = cpu.acc == 0;
    let negative_flag: bool = cpu.acc & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldx(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.x = val,
        OpcodeOperand::Address(adr) => cpu.x = cpu_map.get(adr),
    };

    let zero_flag: bool = cpu.x == 0;
    let negative_flag: bool = cpu.x & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldy(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.y = val,
        OpcodeOperand::Address(adr) => cpu.y = cpu_map.get(adr),
    };

    let zero_flag: bool = cpu.y == 0;
    let negative_flag: bool = cpu.y & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Set cpu_mapdes
pub fn sta(cpu: &Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => cpu_map.set(adr, cpu.acc),
    };

    // no flags to be set
}

pub fn stx(cpu: &Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => cpu_map.set(adr, cpu.x),
    };

    // no flags to be set
}

pub fn sty(cpu: &Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => cpu_map.set(adr, cpu.y),
    };

    // no flags to be set
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Bitwise opcodes
pub fn eor(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc ^= val,
        OpcodeOperand::Address(adr) => cpu.acc ^= cpu_map.get(adr),
    };

    let zero_flag = cpu.acc == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
}

pub fn and(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc &= val,
        OpcodeOperand::Address(adr) => cpu.acc &= cpu_map.get(adr),
    };

    let zero_flag = cpu.acc == 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
}

pub fn ora(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc |= val,
        OpcodeOperand::Address(adr) => cpu.acc |= cpu_map.get(adr),
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
    // TODO: set Negative flag for these instructions
}

// TODO: verify the indexing of tsx and txs is correct
// is it immediate? if so why do the docs call x an index?
pub fn tsx(cpu: &mut Cpu) {
    cpu.x = cpu.s;

    let is_zero = cpu.s == 0;
    let is_neg = (cpu.s as i8) < 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, is_neg);
}

pub fn txs(cpu: &mut Cpu) {
    cpu.s = cpu.x;
}

pub fn pla(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    cpu.s += 1;
    let idx = cpu.s;
    cpu.acc = cpu_map.get(idx as u16);

    let is_zero = cpu.acc == 0;
    let is_neg = (cpu.acc as i8) < 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, is_neg);
}

pub fn pha(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let idx = cpu.s;
    cpu_map.set(idx as u16, cpu.acc);
    cpu.s -= 1;
}

// Note: inconsistent documentation.  Some say that B flag is not affected by this, others say it's the only way
pub fn plp(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    cpu.s += 1;
    let idx = cpu.s;
    cpu.p = cpu_map.get(idx as u16);
}

pub fn php(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let idx = cpu.s;
    cpu_map.set(idx as u16, cpu.p);
    cpu.s -= 1;
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
pub fn adc(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    let old_acc: u8 = cpu.acc;
    let val: u8 = match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => {
            cpu.acc.wrapping_add(val);
            val
        }
        OpcodeOperand::Address(adr) => {
            cpu.acc.wrapping_add(cpu_map.get(adr));
            cpu_map.get(adr)
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

pub fn sbc(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    let old_acc: u8 = cpu.acc;
    let val: u8 = match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => {
            cpu.acc.wrapping_sub(val);
            val
        }
        OpcodeOperand::Address(adr) => {
            cpu.acc.wrapping_sub(cpu_map.get(adr));
            cpu_map.get(adr)
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
pub fn rol(cpu: &mut Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc <<= 1;
            cpu.acc |= cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => {
            let old_val = cpu_map.get(adr);
            let mut temp = old_val;
            temp <<= 1;
            temp |= cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            cpu_map.set(adr, temp);
            (old_val, temp)
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn ror(cpu: &mut Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc >>= 1;
            cpu.acc |= cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => {
            let old_val = cpu_map.get(adr);
            let mut temp = old_val;
            temp >>= 1;
            temp |= (cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8) << 7;
            cpu_map.set(adr, temp);
            (old_val, temp)
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn lsr(cpu: &mut Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc >>= 1;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => {
            let old_val = cpu_map.get(adr);
            let mut temp = old_val;
            temp >>= 1;
            cpu_map.set(adr, temp);
            (old_val, temp)
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn asl(cpu: &mut Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    let (old_val, new_val) = match opop {
        OpcodeOperand::Implied => {
            let old_val = cpu.acc;
            cpu.acc <<= 1;
            (old_val, cpu.acc)
        }
        OpcodeOperand::Immediate(_) => unreachable!(),
        OpcodeOperand::Address(adr) => {
            let old_val = cpu_map.get(adr);
            let mut temp = old_val;
            temp <<= 1;
            cpu_map.set(adr, temp);
            (old_val, temp)
        }
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

/// logic

/// Function that implements the flag setting logic of `cmp`, `cpx`, and `cpy`
fn common_cmp(cpu: &mut Cpu, first: u8, second: u8) {
    use std::cmp::Ordering;
    match first.cmp(&second) {
        Ordering::Equal => {
            cpu.set_flag_value(ProcessorStatusFlag::Negative, false);
            cpu.set_flag_value(ProcessorStatusFlag::Zero, true);
            cpu.set_flag_value(ProcessorStatusFlag::Carry, true);
        }
        Ordering::Greater => {
            cpu.set_flag_value(ProcessorStatusFlag::Negative, true);
            cpu.set_flag_value(ProcessorStatusFlag::Zero, false);
            cpu.set_flag_value(ProcessorStatusFlag::Carry, false);
        }
        Ordering::Less => {
            cpu.set_flag_value(ProcessorStatusFlag::Negative, false);
            cpu.set_flag_value(ProcessorStatusFlag::Zero, false);
            cpu.set_flag_value(ProcessorStatusFlag::Carry, true);
        }
    }
}

pub fn cmp(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    let value = match opop {
        OpcodeOperand::Address(addr) => cpu_map.get(addr),
        OpcodeOperand::Immediate(imm) => imm,
        OpcodeOperand::Implied => unreachable!("Cmp doesn't have a default to compare to"),
    };
    let first_val = cpu.acc;
    common_cmp(cpu, first_val, value)
}

pub fn cpx(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    let value = match opop {
        OpcodeOperand::Address(addr) => cpu_map.get(addr),
        OpcodeOperand::Immediate(imm) => imm,
        OpcodeOperand::Implied => unreachable!("cpx doesn't have a default to compare to"),
    };
    let first_val = cpu.x;
    common_cmp(cpu, first_val, value)
}

pub fn cpy(cpu: &mut Cpu, cpu_map: &dyn GetSet, opop: OpcodeOperand) {
    let value = match opop {
        OpcodeOperand::Address(addr) => cpu_map.get(addr),
        OpcodeOperand::Immediate(imm) => imm,
        OpcodeOperand::Implied => unreachable!("cpy doesn't have a default to compare to"),
    };
    let first_val = cpu.y;
    common_cmp(cpu, first_val, value)
}

pub fn dec(cpu: &mut Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    let new_value = match opop {
        OpcodeOperand::Address(addr) => {
            let val = cpu_map.get(addr).wrapping_sub(1);
            cpu_map.set(addr, val);
            val
        }
        OpcodeOperand::Immediate(_) => unreachable!("immediate values cannot be decremented"),
        OpcodeOperand::Implied => unreachable!("dec doesn't have a default"),
    };

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
}

pub fn inc(cpu: &mut Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    let new_value = match opop {
        OpcodeOperand::Address(addr) => {
            let val = cpu_map.get(addr).wrapping_add(1);
            cpu_map.set(addr, val);
            val
        }
        OpcodeOperand::Immediate(_) => unreachable!("immediate values cannot be decremented"),
        OpcodeOperand::Implied => unreachable!("dec doesn't have a default"),
    };

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
}

// Branching
pub fn bpl(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let negative = cpu.get_processor_status_flag(ProcessorStatusFlag::Negative);
    if !negative {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid bpl argument"),
        }
    }

    !negative
}

pub fn bmi(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let negative = cpu.get_processor_status_flag(ProcessorStatusFlag::Negative);
    if negative {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid bmi argument"),
        }
    }

    negative
}

pub fn bvc(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let overflow = cpu.get_processor_status_flag(ProcessorStatusFlag::Overflow);
    if !overflow {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid bvc argument"),
        }
    }

    !overflow
}

pub fn bvs(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let overflow = cpu.get_processor_status_flag(ProcessorStatusFlag::Overflow);
    if overflow {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid bvs argument"),
        }
    }

    overflow
}

pub fn bcc(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let carry = cpu.get_processor_status_flag(ProcessorStatusFlag::Carry);
    if !carry {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid bcc argument"),
        }
    }

    !carry
}

pub fn bcs(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let carry = cpu.get_processor_status_flag(ProcessorStatusFlag::Carry);
    if carry {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid bcs argument"),
        }
    }

    carry
}

pub fn bne(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let zero = cpu.get_processor_status_flag(ProcessorStatusFlag::Zero);
    if !zero {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid bne argument"),
        }
    }

    !zero
}

pub fn beq(cpu: &mut Cpu, opop: OpcodeOperand) -> bool {
    let zero = cpu.get_processor_status_flag(ProcessorStatusFlag::Zero);
    if zero {
        match opop {
            OpcodeOperand::Immediate(v) => {
                cpu.pc = (cpu.pc as i16 + v as i16) as u16;
            }
            _ => unreachable!("invalid beq argument"),
        }
    }

    zero
}

// TODO: this function is probably wrong
pub fn brk(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    cpu.set_flag_value(ProcessorStatusFlag::Interrupt, true);
    cpu.set_flag_value(ProcessorStatusFlag::Break, true);
    let idx = cpu.s;
    cpu_map.set(idx as u16, cpu.pc as u8);
    cpu_map.set(idx.wrapping_sub(1) as u16, (cpu.pc >> 8) as u8);
    cpu_map.set(idx.wrapping_sub(2) as u16, cpu.p);
    cpu.s = cpu.s.wrapping_sub(3);
    cpu.pc = cpu_map.get(0xFFFE) as u16 | ((cpu_map.get(0xFFFF) as u16) << 8);
}

pub fn rti(cpu: &mut Cpu, cpu_map: &dyn GetSet) {
    let idx = cpu.s;
    cpu.p = cpu_map.get(idx as u16 + 1);
    cpu.pc = cpu_map.get(idx as u16 + 2) as u16 | ((cpu_map.get(idx as u16 + 3) as u16) << 8);
    cpu.s += 3;
}

pub fn jsr(cpu: &mut Cpu, cpu_map: &mut dyn GetSet, opop: OpcodeOperand) {
    let idx = cpu.s;
    cpu_map.set(idx as u16, cpu.pc as u8);
    cpu_map.set(idx as u16 - 1, (cpu.pc >> 8) as u8);
    cpu.s -= 2;
    match opop {
        OpcodeOperand::Address(addr) => {
            cpu.pc = addr;
        }
        _ => unreachable!(),
    }
}

pub fn rts(cpu: &mut Cpu, cpu_map: &dyn GetSet) {
    let idx = cpu.s;
    // TODO: docs imply address here is off by 1!!!
    cpu.pc = cpu_map.get(idx as u16 + 1) as u16 | ((cpu_map.get(idx as u16 + 2) as u16) << 8);
    cpu.s += 2;
}

pub fn jmp(cpu: &mut Cpu, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Address(addr) => {
            cpu.pc = addr;
        }
        _ => unreachable!(),
    }
}

//review this, notation used was weird
pub fn bit(cpu: &mut Cpu, opop: OpcodeOperand) {
    let acc = cpu.acc as u16;
    match opop {
        OpcodeOperand::Address(addr) => {
            cpu.set_flag_value(ProcessorStatusFlag::Negative, (addr >> 7) & 1 == 1);
            cpu.set_flag_value(ProcessorStatusFlag::Overflow, ((addr >> 6) & 1) == 1);
            cpu.set_flag_value(ProcessorStatusFlag::Zero, acc & addr == 0);
        }
        _ => unreachable!(),
    }
}
