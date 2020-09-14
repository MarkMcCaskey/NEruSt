use crate::cpu::cpu::{Cpu, ProcessorStatusFlag};
use crate::getset::GetSet;

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Load register opcodes
pub fn lda(cpu: &mut Cpu, val: u8) {
    cpu.acc = val;

    let zero_flag: bool = cpu.acc == 0;
    let negative_flag: bool = cpu.acc & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldx(cpu: &mut Cpu, val: u8) {
    cpu.x = val;

    let zero_flag: bool = cpu.x == 0;
    let negative_flag: bool = cpu.x & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

pub fn ldy(cpu: &mut Cpu, val: u8) {
    cpu.y = val;

    let zero_flag: bool = cpu.y == 0;
    let negative_flag: bool = cpu.y & 0x80 == 0x80;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Set cpu_mapdes
pub fn sta(cpu: &Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
    cpu_map.set(addr, cpu.acc);
}

pub fn stx(cpu: &Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
    cpu_map.set(addr, cpu.x);
}

pub fn sty(cpu: &Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
    cpu_map.set(addr, cpu.y);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Bitwise opcodes
pub fn eor(cpu: &mut Cpu, val: u8) {
    cpu.acc ^= val;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, (cpu.acc as i8) < 0);
}

pub fn and(cpu: &mut Cpu, val: u8) {
    cpu.acc &= val;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, (cpu.acc as i8) < 0);
}

pub fn ora(cpu: &mut Cpu, val: u8) {
    cpu.acc |= val;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, (cpu.acc as i8) < 0);
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
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.acc & 0x80 == 0x80);
}

pub fn txa(cpu: &mut Cpu) {
    cpu.acc = cpu.x;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.x == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.x & 0x80 == 0x80);
}

pub fn tay(cpu: &mut Cpu) {
    cpu.y = cpu.acc;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.acc & 0x80 == 0x80);
}

pub fn tya(cpu: &mut Cpu) {
    cpu.acc = cpu.y;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.y == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.y & 0x80 == 0x80);
}

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
    let idx = cpu.s as u16 + 0x100;
    cpu.acc = cpu_map.get(idx);

    let is_zero = cpu.acc == 0;
    let is_neg = (cpu.acc as i8) < 0;
    cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, is_neg);
}

pub fn pha(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let idx = cpu.s as u16 + 0x100;
    cpu_map.set(idx, cpu.acc);
    cpu.s -= 1;
}

// TODO: possibly wrong
pub fn plp(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    cpu.s += 1;
    let idx = cpu.s as u16 + 0x100;
    cpu.p = cpu_map.get(idx) & 0b11001111 | 0b00100000;
}

// TODO: possibly wrong
pub fn php(cpu: &mut Cpu, cpu_map: &mut dyn GetSet) {
    let idx = cpu.s as u16 + 0x100;
    cpu_map.set(idx as u16, cpu.p | 0b00110000);
    cpu.s -= 1;
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Increment opcodes
pub fn dex(cpu: &mut Cpu) {
    cpu.x = cpu.x.wrapping_sub(1);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.x == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.x & 0x80 == 0x80);
}

pub fn inx(cpu: &mut Cpu) {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.x == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.x & 0x80 == 0x80);
}

pub fn dey(cpu: &mut Cpu) {
    cpu.y = cpu.y.wrapping_sub(1);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.y == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.y & 0x80 == 0x80);
}

pub fn iny(cpu: &mut Cpu) {
    cpu.y = cpu.y.wrapping_add(1);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.y == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, cpu.y & 0x80 == 0x80);
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
///// Math opcodes
pub fn adc(cpu: &mut Cpu, val: u8) {
    let old_acc = cpu.acc;
    cpu.acc = cpu
        .acc
        .wrapping_add(val)
        .wrapping_add(cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) as u8);

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

pub fn sbc(cpu: &mut Cpu, val: u8) {
    adc(cpu, !val);
    /*

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
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);*/
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

pub fn rol(cpu: &mut Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
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

pub fn ror(cpu: &mut Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
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

pub fn lsr(cpu: &mut Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
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

pub fn asl(cpu: &mut Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
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
    let negative_flag = first.wrapping_sub(second) & 0x80 == 0x80;
    let carry_flag = first >= second;
    let zero_flag = first == second;

    cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
}

pub fn cmp(cpu: &mut Cpu, val: u8) {
    common_cmp(cpu, cpu.acc, val);
}

pub fn cpx(cpu: &mut Cpu, val: u8) {
    common_cmp(cpu, cpu.x, val)
}

pub fn cpy(cpu: &mut Cpu, val: u8) {
    common_cmp(cpu, cpu.y, val)
}

pub fn dec(cpu: &mut Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
    let new_value = cpu_map.get(addr).wrapping_sub(1);
    cpu_map.set(addr, new_value);

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
}

pub fn inc(cpu: &mut Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
    let new_value = cpu_map.get(addr).wrapping_add(1);
    cpu_map.set(addr, new_value);

    cpu.set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
}

// Branching
pub fn bpl(cpu: &mut Cpu, val: u8) -> u8 {
    if !cpu.get_processor_status_flag(ProcessorStatusFlag::Negative) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
}

pub fn bmi(cpu: &mut Cpu, val: u8) -> u8 {
    if cpu.get_processor_status_flag(ProcessorStatusFlag::Negative) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
}

pub fn bvc(cpu: &mut Cpu, val: u8) -> u8 {
    if !cpu.get_processor_status_flag(ProcessorStatusFlag::Overflow) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
}

pub fn bvs(cpu: &mut Cpu, val: u8) -> u8 {
    if cpu.get_processor_status_flag(ProcessorStatusFlag::Overflow) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
}

pub fn bcc(cpu: &mut Cpu, val: u8) -> u8 {
    if !cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
}

pub fn bcs(cpu: &mut Cpu, val: u8) -> u8 {
    if cpu.get_processor_status_flag(ProcessorStatusFlag::Carry) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
}

pub fn bne(cpu: &mut Cpu, val: u8) -> u8 {
    if !cpu.get_processor_status_flag(ProcessorStatusFlag::Zero) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
}

pub fn beq(cpu: &mut Cpu, val: u8) -> u8 {
    if cpu.get_processor_status_flag(ProcessorStatusFlag::Zero) {
        let old_pc = cpu.pc;
        cpu.pc = (cpu.pc as i16 + val as i16) as u16;
        return 1 + (cpu.pc > old_pc | 0x0FF) as u8;
    }

    0
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

pub fn jsr(cpu: &mut Cpu, addr: u16, cpu_map: &mut dyn GetSet) {
    let idx = cpu.s;
    cpu_map.set(idx as u16, cpu.pc as u8);
    cpu_map.set(idx as u16 - 1, (cpu.pc >> 8) as u8);
    cpu.s -= 2;
    cpu.pc = addr
}

pub fn rts(cpu: &mut Cpu, cpu_map: &dyn GetSet) {
    let idx = cpu.s;
    // TODO: docs imply address here is off by 1!!!
    cpu.pc = cpu_map.get(idx as u16 + 2) as u16 | ((cpu_map.get(idx as u16 + 1) as u16) << 8);
    cpu.s += 2;
}

pub fn jmp(cpu: &mut Cpu, addr: u16) {
    cpu.pc = addr;
}

pub fn bit(cpu: &mut Cpu, addr: u16, cpu_map: &dyn GetSet) {
    let val = cpu_map.get(addr);
    cpu.set_flag_value(ProcessorStatusFlag::Negative, (val >> 7) & 1 == 1);
    cpu.set_flag_value(ProcessorStatusFlag::Overflow, ((val >> 6) & 1) == 1);
    cpu.set_flag_value(ProcessorStatusFlag::Zero, cpu.acc & val == 0);
}
