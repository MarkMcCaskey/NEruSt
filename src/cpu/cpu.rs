use crate::cpu::addressing_modes::*;
use crate::cpu::opcode_logic::*;
use crate::getset::GetSet;

pub struct Cpu {
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    /// process status register
    pub p: u8,
    pub pc: u16,
    /// stack pointer
    pub s: u8,
    /// Whether the CPU is running
    // TODO: when set data bus returns $FF
    pub halt: bool,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ProcessorStatusFlag {
    Carry = 0,
    Zero,
    Interrupt,
    Decimal,
    Break,
    Always,
    Overflow,
    Negative,
}

impl Cpu {
    /// Initializes the SELF registers (currently placeholder values)
    pub fn new() -> Self {
        Self {
            acc: 0,
            x: 0,
            y: 0,
            p: 0x24,
            pc: 0,
            s: 0xFD,
            halt: false,
        }
    }

    #[inline(always)]
    pub fn clear_flag(&mut self, flag: ProcessorStatusFlag) {
        self.p &= !(1 << flag as u8);
    }

    #[inline(always)]
    pub fn set_flag(&mut self, flag: ProcessorStatusFlag) {
        self.p |= 1 << (flag as u8);
    }

    #[inline(always)]
    pub fn set_flag_value(&mut self, flag: ProcessorStatusFlag, val: bool) {
        match val {
            true => self.set_flag(flag),
            false => self.clear_flag(flag),
        }
    }

    pub fn get_processor_status_flag(&self, flag: ProcessorStatusFlag) -> bool {
        let flag_bit = 1 << (flag as u8);
        (self.p & flag_bit) == flag_bit
    }

    pub fn run_instruction<CPU_MAP: GetSet>(&mut self, cpu_map: &mut CPU_MAP) -> u8 {
        let op = cpu_map.get(self.pc);

        let pc_inc_by;
        let cyc_inc_by;
        match op {
            0x00 => {
                brk(self, cpu_map);
                pc_inc_by = 1;
                cyc_inc_by = 7;
            }

            0x01 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                ora(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x05 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                ora(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x06 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                asl(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x08 => {
                php(self, cpu_map);
                pc_inc_by = 1;
                cyc_inc_by = 3;
            }

            0x09 => {
                let operand = cpu_map.get(self.pc + 1);
                ora(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x0A => {
                asl_imp(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x0D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                ora(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x0E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                asl(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x10 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = bpl(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x11 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, add_cycle) = izy(operand, self.y, cpu_map);
                ora(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x15 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                ora(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x16 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                asl(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x18 => {
                clc(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x19 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                ora(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x1D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                ora(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x1E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = abx(operand, self.x);
                asl(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x20 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                jsr(self, addr, cpu_map);
                pc_inc_by = 0;
                cyc_inc_by = 6;
            }

            0x21 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                and(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x24 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                bit(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x25 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                and(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x26 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                rol(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x28 => {
                plp(self, cpu_map);
                pc_inc_by = 1;
                cyc_inc_by = 4;
            }

            0x29 => {
                let operand = cpu_map.get(self.pc + 1);
                and(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x2A => {
                rol_imp(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x2C => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                bit(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }
            0x2D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                and(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x2E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                rol(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x30 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = bmi(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x31 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, add_cycle) = izy(operand, self.y, cpu_map);
                and(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x35 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                and(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x36 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                rol(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x38 => {
                sec(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x39 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                and(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x3D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                and(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x3E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = abx(operand, self.x);
                rol(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x40 => {
                rti(self, cpu_map);
                pc_inc_by = 0; // rti as subtler behaviors with pc
                cyc_inc_by = 6;
            }

            0x41 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                eor(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x45 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                eor(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x46 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                lsr(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x48 => {
                pha(self, cpu_map);
                pc_inc_by = 1;
                cyc_inc_by = 3;
            }

            0x49 => {
                let operand = cpu_map.get(self.pc + 1);
                eor(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x4A => {
                lsr_imp(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x4C => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                jmp(self, addr);
                pc_inc_by = 0;
                cyc_inc_by = 3;
            }

            0x4D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                eor(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x4E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                lsr(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x50 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = bvc(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x51 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, add_cycle) = izy(operand, self.y, cpu_map);
                eor(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x55 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                eor(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x56 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                lsr(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x58 => {
                cli(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x59 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                eor(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x5D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                eor(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x5E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = abx(operand, self.x);
                lsr(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x60 => {
                rts(self, cpu_map);
                pc_inc_by = 0; // rts as subtler behaviors with pc
                cyc_inc_by = 6;
            }

            0x61 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                adc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x65 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                adc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x66 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                ror(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x68 => {
                pla(self, cpu_map);
                pc_inc_by = 1;
                cyc_inc_by = 4;
            }

            0x69 => {
                let operand = cpu_map.get(self.pc + 1);
                adc(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x6A => {
                ror_imp(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x6C => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = ind(operand, cpu_map);
                jmp(self, addr);
                pc_inc_by = 0;
                cyc_inc_by = 5;
            }

            0x6D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                adc(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x6E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                ror(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x70 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = bvs(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x71 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, add_cycle) = izy(operand, self.y, cpu_map);
                adc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x75 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                adc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x76 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                ror(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x78 => {
                sei(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x79 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                adc(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x7D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                adc(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x7E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = abx(operand, self.x);
                ror(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x81 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                sta(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x84 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                sty(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x85 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                sta(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x86 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                stx(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x88 => {
                dey(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x8A => {
                txa(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x8C => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                sty(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x8D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                sta(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x8E => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                stx(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x90 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = bcc(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x91 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, _) = izy(operand, self.y, cpu_map);
                sta(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x94 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                sty(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x95 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                sta(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x96 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpy(operand, self.y);
                stx(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x98 => {
                tya(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x99 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = aby(operand, self.y);
                sta(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 5;
            }

            0x9A => {
                txs(self);
                // TODO: review
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x9D => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = abx(operand, self.x);
                sta(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 5;
            }

            0xA0 => {
                let operand = cpu_map.get(self.pc + 1);
                ldy(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xA1 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                lda(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xA2 => {
                let operand = cpu_map.get(self.pc + 1);
                ldx(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xA4 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                ldy(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xA5 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                lda(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xA6 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                ldx(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xA8 => {
                tay(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xA9 => {
                let operand = cpu_map.get(self.pc + 1);
                lda(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xAA => {
                tax(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xAC => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                ldy(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xAD => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                lda(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xAE => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                ldx(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xB0 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = bcs(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0xB1 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, add_cycle) = izy(operand, self.y, cpu_map);
                lda(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0xB4 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                ldy(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xB5 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                lda(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xB6 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpy(operand, self.y);
                ldx(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xB8 => {
                clv(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xB9 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                lda(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xBA => {
                tsx(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xBC => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                ldy(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xBD => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                lda(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xBE => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                ldx(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xC0 => {
                let operand = cpu_map.get(self.pc + 1);
                cpy(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xC1 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                cmp(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xC4 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                cpy(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xC5 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                cmp(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xC6 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                dec(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0xC8 => {
                iny(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xC9 => {
                let operand = cpu_map.get(self.pc + 1);
                cmp(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xCA => {
                dex(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xCC => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                cpy(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xCD => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                cmp(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xCE => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                dec(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0xD0 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = bne(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0xD1 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, add_cycle) = izy(operand, self.y, cpu_map);
                cmp(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0xD5 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                cmp(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xD6 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                dec(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xD8 => {
                cld(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xD9 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                cmp(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xDD => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                cmp(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xDE => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = abx(operand, self.x);
                dec(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0xE0 => {
                let operand = cpu_map.get(self.pc + 1);
                cpx(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xE1 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = izx(operand, self.x, cpu_map);
                sbc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xE4 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                cpx(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xE5 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                sbc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xE6 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zp(operand);
                inc(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0xE8 => {
                inx(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xE9 => {
                let operand = cpu_map.get(self.pc + 1);
                sbc(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xEC => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                cpx(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xED => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                sbc(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xEE => {
                let operand = cpu_map.get_16(self.pc + 1);
                let addr = abs(operand);
                inc(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0xF0 => {
                let operand = cpu_map.get(self.pc + 1);
                let result = beq(self, operand);
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0xF1 => {
                let operand = cpu_map.get(self.pc + 1);
                let (addr, add_cycle) = izy(operand, self.y, cpu_map);
                sbc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0xF5 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                sbc(self, cpu_map.get(addr));
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xF6 => {
                let operand = cpu_map.get(self.pc + 1);
                let addr = zpx(operand, self.x);
                inc(self, addr, cpu_map);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xF8 => {
                sed(self);
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xF9 => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = aby(operand, self.y);
                sbc(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xFD => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, add_cycle) = abx(operand, self.x);
                sbc(self, cpu_map.get(addr));
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xFE => {
                let operand = cpu_map.get_16(self.pc + 1);
                let (addr, _) = abx(operand, self.x);
                inc(self, addr, cpu_map);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => {
                self.halt = true;
                pc_inc_by = 0;
                cyc_inc_by = 4;
            }
            0x80 => {
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }
            0x1A | 0x3A | 0x5A | 0x7A | 0x82 | 0x89 | 0xC2 | 0xDA | 0xE2 | 0xEA | 0xFA => {
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }
            0x04 | 0x44 | 0x64 => {
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }
            0x0C => {
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }
            0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => {
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            // Shouldn't ever happen.
            // NOTE: can use unreachable!() to tell the compiler this ^
            otherwise => {
                panic!("Opcode 0x{:X} has not yet been implemented", otherwise);
            }
        };

        self.pc += pc_inc_by;
        cyc_inc_by
    }
}
