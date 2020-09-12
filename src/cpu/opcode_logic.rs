use crate::cpu::cpu::{Cpu, ProcessorStatusFlag};
use crate::getset::GetSet;

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Load register opcodes
pub fn lda(val: u8, cpu: &mut Cpu) {
    cpu.acc = val;

    let zero_flag: bool = cpu.acc == 0;
    let negative_flag: bool = cpu.acc & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldx(val: u8, cpu: &mut Cpu) {
    cpu.x = val;

    let zero_flag: bool = cpu.x == 0;
    let negative_flag: bool = cpu.x & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldy(val: u8, cpu: &mut Cpu) {
    cpu.y = val;

    let zero_flag: bool = cpu.y == 0;
    let negative_flag: bool = cpu.y & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Set cpu_mapdes
pub fn sta(addr: u16, cpu: &Cpu, cpu_map: &mut dyn GetSet) {
    cpu_map.set(addr, cpu.acc);
}

pub fn stx(addr: u16, cpu: &Cpu, cpu_map: &mut dyn GetSet) {
    cpu_map.set(addr, cpu.x);
}

pub fn sty(addr: u16, cpu: &Cpu, cpu_map: &mut dyn GetSet) {
    cpu_map.set(addr, cpu.y);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Bitwise opcodes
pub fn eor(val: u8, cpu: &mut Cpu) {
    cpu.acc ^= val;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
}

pub fn and(val: u8, cpu: &mut Cpu) {
    cpu.acc &= val;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
}

pub fn ora(val: u8, cpu: &mut Cpu) {
    cpu.acc |= val;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Set/clear flag opcodes
pub fn clc(cpu: &mut Cpu) {
    cpu.clear_flag(ProcessorStatusFlag::Carry);
}

pub fn cld(cpu: &mut Cpu) {
    cpu.clear_flag(ProcessorStatusFlag::Decimal);
}

pub fn cli(cpu: &mut Cpu) {
    cpu.clear_flag(ProcessorStatusFlag::Interrupt);
}

pub fn clv(cpu: &mut Cpu) {
    cpu.clear_flag(ProcessorStatusFlag::Overflow);
}

pub fn sec(cpu: &mut Cpu) {
    cpu.set_flag(ProcessorStatusFlag::Carry);
}

pub fn sed(cpu: &mut Cpu) {
    cpu.set_flag(ProcessorStatusFlag::Decimal);
}

pub fn sei(cpu: &mut Cpu) {
    cpu.set_flag(ProcessorStatusFlag::Interrupt);
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
pub fn adc(val: u8, cpu: &mut Cpu) {
    let old_acc = cpu.acc;
    cpu.acc = cpu.acc.wrapping_add(val);

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

pub fn sbc(val: u8, cpu: &mut Cpu) {
    let old_acc: u8 = cpu.acc;
    cpu.acc = cpu.acc.wrapping_sub(val);

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
pub fn rol_imp(cpu: &mut Cpu) {
    let (old_val, new_val) = {
        let old_val = cpu.acc;
        cpu.acc <<= 1;
        cpu.acc |= cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
        (old_val, cpu.acc)
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn rol(addr: u16, cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let (old_val, new_val) = {
        let old_val = cpu_map.get(addr);
        let mut temp = old_val;
        temp <<= 1;
        temp |= cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
        cpu_map.set(addr, temp);
        (old_val, temp)
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn ror_imp(cpu: &mut Cpu) {
    let (old_val, new_val) = {
        let old_val = cpu.acc;
        cpu.acc >>= 1;
        cpu.acc |= cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
        (old_val, cpu.acc)
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn ror(addr: u16, cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let (old_val, new_val) = {
        let old_val = cpu_map.get(addr);
        let mut temp = old_val;
        temp >>= 1;
        temp |= (cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8) << 7;
        cpu_map.set(addr, temp);
        (old_val, temp)
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn lsr_imp(cpu: &mut Cpu) {
    let (old_val, new_val) = {
        let old_val = cpu.acc;
        cpu.acc >>= 1;
        (old_val, cpu.acc)
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn lsr(addr: u16, cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let (old_val, new_val) = {
        let old_val = cpu_map.get(addr);
        let mut temp = old_val;
        temp >>= 1;
        cpu_map.set(addr, temp);
        (old_val, temp)
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x01 == 0x01;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn asl_imp(cpu: &mut Cpu) {
    let (old_val, new_val) = {
        let old_val = cpu.acc;
        cpu.acc <<= 1;
        (old_val, cpu.acc)
    };

    let negative_flag = new_val & 0x80 == 0x80;
    let zero_flag = new_val == 0;
    let carry_flag = old_val & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
}

pub fn asl(addr: u16, cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let (old_val, new_val) = {
        let old_val = cpu_map.get(addr);
        let mut temp = old_val;
        temp <<= 1;
        cpu_map.set(addr, temp);
        (old_val, temp)
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

pub fn cmp(val: u8, cpu: &mut Cpu) {
    common_cmp(cpu, cpu.acc, val);
}

pub fn cpx(val: u8, cpu: &mut Cpu) {
    common_cmp(cpu, cpu.x, val)
}

pub fn cpy(val: u8, cpu: &mut Cpu) {
    common_cmp(cpu, cpu.y, val)
}

pub fn dec(addr: u16, cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let new_value = cpu_map.get(addr).wrapping_sub(1);
    cpu_map.set(addr, new_value);

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
}

pub fn inc(addr: u16, cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let new_value = cpu_map.get(addr).wrapping_add(1);
    cpu_map.set(addr, new_value);

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
}

// Branching
pub fn bpl(val: u8, cpu: &mut Cpu) -> bool {
    let negative = cpu.get_processor_status_flag(ProcessorStatusFlag::Negative);
    if !negative {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
    }

    !negative
}

pub fn bmi(val: u8, cpu: &mut Cpu) -> bool {
    let negative = cpu.get_processor_status_flag(ProcessorStatusFlag::Negative);
    if negative {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
    }

    negative
}

pub fn bvc(val: u8, cpu: &mut Cpu) -> bool {
    let overflow = cpu.get_processor_status_flag(ProcessorStatusFlag::Overflow);
    if !overflow {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
    }

    !overflow
}

pub fn bvs(val: u8, cpu: &mut Cpu) -> bool {
    let overflow = cpu.get_processor_status_flag(ProcessorStatusFlag::Overflow);
    if overflow {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
    }

    overflow
}

pub fn bcc(val: u8, cpu: &mut Cpu) -> bool {
    let carry = cpu.get_processor_status_flag(ProcessorStatusFlag::Carry);
    if !carry {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
    }

    !carry
}

pub fn bcs(val: u8, cpu: &mut Cpu) -> bool {
    let carry = cpu.get_processor_status_flag(ProcessorStatusFlag::Carry);
    if carry {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
    }

    carry
}

pub fn bne(val: u8, cpu: &mut Cpu) -> bool {
    let zero = cpu.get_processor_status_flag(ProcessorStatusFlag::Zero);
    if !zero {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
    }

    !zero
}

pub fn beq(val: u8, cpu: &mut Cpu) -> bool {
    let zero = cpu.get_processor_status_flag(ProcessorStatusFlag::Zero);
    if zero {
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
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

pub fn jsr(addr: u16, cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let idx = cpu.s;
    cpu_map.set(idx as u16, cpu.pc as u8);
    cpu_map.set(idx as u16 - 1, (cpu.pc >> 8) as u8);
    cpu.s -= 2;
    cpu.pc = addr
}

pub fn rts(cpu: &mut Cpu, cpu_map: &dyn GetSet) {
    let idx = cpu.s;
    // TODO: docs imply address here is off by 1!!!
    cpu.pc = cpu_map.get(idx as u16 + 1) as u16 | ((cpu_map.get(idx as u16 + 2) as u16) << 8);
    cpu.s += 2;
}

pub fn jmp(addr: u16, cpu: &mut Cpu) {
    cpu.pc = addr;
}

//review this, notation used was weird
pub fn bit(addr: u16, cpu: &mut Cpu) {
    let acc = cpu.acc as u16;
    cpu.set_flag_value(ProcessorStatusFlag::Negative, (addr >> 7) & 1 == 1);
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, ((addr >> 6) & 1) == 1);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, acc & addr == 0);
}
