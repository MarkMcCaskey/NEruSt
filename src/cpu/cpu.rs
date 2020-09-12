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
            p: 0x34,
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
        self.p |= 1 << flag as u8;
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

    pub fn run_instruction(&mut self, cpu_map: &mut dyn GetSet) -> u8 {
        let op = cpu_map.get(self.pc);

        let pc_inc_by;
        let time_passed = match op {
            0x00 => {
                brk(self, cpu_map);
                pc_inc_by = 1;
                7
            }

            0x01 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                ora(self, cpu_map, opop);
                // unverified; same with other recently added opcodes to dispatch;
                // TODO: use gitblame and update recently added time delays
                pc_inc_by = 2;
                6
            }

            0x05 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                ora(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0x06 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                asl(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x08 => {
                php(self, cpu_map);
                pc_inc_by = 1;
                3
            }

            0x09 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                ora(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0x0A => {
                let opop = imp();
                asl(self, cpu_map, opop);
                pc_inc_by = 1;
                4
            }

            0x0D => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                ora(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x0E => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                asl(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x10 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = bpl(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0x11 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.y);
                ora(self, cpu_map, opop);
                pc_inc_by = 2;
                5 + add_cycle as u8
            }

            0x15 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                ora(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x16 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                asl(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x18 => {
                clc(self);
                pc_inc_by = 1;
                2
            }

            0x19 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                ora(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x1D => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                ora(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x1E => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                asl(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x20 => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                jsr(self, cpu_map, opop);

                pc_inc_by = 0;
                6
            }

            0x21 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                and(self, cpu_map, opop);
                pc_inc_by = 2;
                6
            }

            0x24 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                bit(self, opop);
                pc_inc_by = 2;
                3
            }

            0x25 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                and(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0x26 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                rol(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x28 => {
                plp(self, cpu_map);
                pc_inc_by = 1;
                4
            }

            0x29 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                and(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0x2A => {
                let opop = imp();
                rol(self, cpu_map, opop);
                pc_inc_by = 1;
                4
            }

            0x2C => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                bit(self, opop);
                pc_inc_by = 3;
                4
            }
            0x2D => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                and(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x2E => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                rol(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x30 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = bmi(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0x31 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.y);
                and(self, cpu_map, opop);
                pc_inc_by = 2;
                5 + add_cycle as u8
            }

            0x35 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                and(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x36 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                rol(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x38 => {
                sec(self);
                pc_inc_by = 1;
                2
            }

            0x39 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                and(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x3D => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                and(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x3E => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                rol(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x40 => {
                rti(self, cpu_map);
                pc_inc_by = 0;
                6
            }

            0x41 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                eor(self, cpu_map, opop);
                pc_inc_by = 2;
                6
            }

            0x45 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                eor(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0x46 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                lsr(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x48 => {
                pha(self, cpu_map);
                pc_inc_by = 1;
                3
            }

            0x49 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                eor(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0x4A => {
                let opop = imp();
                lsr(self, cpu_map, opop);
                pc_inc_by = 1;
                4
            }

            0x4C => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                jmp(self, opop);
                pc_inc_by = 0;
                3
            }

            0x4D => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                eor(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x4E => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                lsr(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x50 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = bvc(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0x51 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.y);
                eor(self, cpu_map, opop);
                pc_inc_by = 2;
                5 + add_cycle as u8
            }

            0x55 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                eor(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x56 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                lsr(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x58 => {
                cli(self);
                pc_inc_by = 1;
                2
            }

            0x59 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                eor(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x5D => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                eor(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x5E => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                lsr(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x60 => {
                rts(self, cpu_map);
                pc_inc_by = 0;
                6
            }

            0x61 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                adc(self, cpu_map, opop);
                pc_inc_by = 2;
                6
            }

            0x65 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                adc(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0x66 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                ror(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x68 => {
                pla(self, cpu_map);
                pc_inc_by = 1;
                4
            }

            0x69 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                adc(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0x6A => {
                let opop = imp();
                ror(self, cpu_map, opop);
                pc_inc_by = 1;
                4
            }

            0x6C => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = ind(cpu_map, data);
                jmp(self, opop);
                pc_inc_by = 0;
                5
            }

            0x6D => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                adc(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x6E => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                ror(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x70 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = bvs(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0x71 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.y);
                adc(self, cpu_map, opop);
                pc_inc_by = 2;
                5 + add_cycle as u8
            }

            0x75 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                adc(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x76 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                ror(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x78 => {
                sei(self);
                pc_inc_by = 1;
                2
            }

            0x79 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                adc(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x7D => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                adc(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x7E => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                ror(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x81 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                sta(&self, cpu_map, opop);
                pc_inc_by = 2;
                6
            }

            0x84 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                sty(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0x85 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                sta(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0x86 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                stx(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x88 => {
                dey(self);
                pc_inc_by = 1;
                4
            }

            0x8A => {
                txa(self);
                pc_inc_by = 1;
                2
            }

            0x8C => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                sty(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x8D => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                sta(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x8E => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                stx(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0x90 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = bcc(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0x91 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.y);
                sta(self, cpu_map, opop);
                pc_inc_by = 2;
                5 + add_cycle as u8
            }

            0x94 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                sty(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x95 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                sta(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x96 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpy(data, self.y);
                stx(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0x98 => {
                tya(self);
                pc_inc_by = 1;
                2
            }

            0x99 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                sta(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0x9A => {
                txs(self);
                // TODO: review
                pc_inc_by = 0;
                2
            }

            0x9D => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                sta(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xA0 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                ldy(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0xA1 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                lda(self, cpu_map, opop);
                pc_inc_by = 2;
                6
            }

            0xA2 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                ldx(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0xA4 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                ldy(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0xA5 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                lda(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0xA6 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                ldx(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0xA8 => {
                tay(self);
                pc_inc_by = 1;
                2
            }

            0xA9 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                lda(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0xAA => {
                tax(self);
                pc_inc_by = 1;
                2
            }

            0xAD => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                ldy(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xAE => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                ldx(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xB0 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = bcs(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0xB1 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.y);
                lda(self, cpu_map, opop);
                pc_inc_by = 2;
                5 + add_cycle as u8
            }

            0xB4 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                ldy(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xB5 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                lda(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xB6 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpy(data, self.y);
                ldx(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xB8 => {
                clv(self);
                pc_inc_by = 1;
                2
            }

            0xB9 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                lda(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xBA => {
                tsx(self);
                pc_inc_by = 1;
                2
            }

            0xBC => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                ldy(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xBD => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                lda(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xBE => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                ldx(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xC0 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                cpy(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xC1 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                cmp(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xC4 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                cpy(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xC5 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                cmp(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xC6 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                dec(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xC8 => {
                iny(self);
                pc_inc_by = 1;
                4
            }

            0xC9 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                cmp(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xCA => {
                dex(self);
                pc_inc_by = 1;
                4
            }

            0xCC => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                cpy(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xCD => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                cmp(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xCE => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                dec(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xD0 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = bne(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0xD1 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.x);
                cmp(self, cpu_map, opop);
                pc_inc_by = 2;
                4 + add_cycle as u8
            }

            0xD5 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                cmp(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xD6 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                dec(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xD8 => {
                cld(self);
                pc_inc_by = 1;
                2
            }

            0xD9 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                cmp(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xDD => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                cmp(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xDE => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                dec(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xE0 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                cpx(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xE1 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = izx(cpu_map, data, self.x);
                sbc(self, cpu_map, opop);
                pc_inc_by = 2;
                6
            }

            0xE4 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                cpx(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xE5 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                sbc(self, cpu_map, opop);
                pc_inc_by = 2;
                3
            }

            0xE6 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zp(data);
                inc(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xE8 => {
                inx(self);
                pc_inc_by = 1;
                4
            }

            0xE9 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                sbc(self, cpu_map, opop);
                pc_inc_by = 2;
                2
            }

            0xEC => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                cpx(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xED => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                sbc(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xEE => {
                let data = cpu_map.get_16(self.pc + 1);
                let opop = abs(data);
                inc(self, cpu_map, opop);
                pc_inc_by = 3;
                4
            }

            0xF0 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = imm(data);
                let result = beq(self, opop);
                pc_inc_by = 2;
                2 + result as u8 // + page boundary crossed
            }

            0xF1 => {
                let data = cpu_map.get(self.pc + 1);
                let (opop, add_cycle) = izy(cpu_map, data, self.y);
                sbc(self, cpu_map, opop);
                pc_inc_by = 2;
                5 + add_cycle as u8
            }

            0xF5 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                sbc(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xF6 => {
                let data = cpu_map.get(self.pc + 1);
                let opop = zpx(data, self.x);
                inc(self, cpu_map, opop);
                pc_inc_by = 2;
                4
            }

            0xF8 => {
                sed(self);
                pc_inc_by = 1;
                2
            }

            0xF9 => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = aby(data, self.y);
                sbc(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xFD => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                sbc(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }

            0xFE => {
                let data = cpu_map.get_16(self.pc + 1);
                let (opop, add_cycle) = abx(data, self.x);
                inc(self, cpu_map, opop);
                pc_inc_by = 3;
                4 + add_cycle as u8
            }
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => {
                self.halt = true;
                pc_inc_by = 0;
                4
            }
            0x1A | 0x3A | 0x5A | 0x7A | 0x80 | 0x82 | 0x89 | 0xC2 | 0xDA | 0xE2 | 0xEA | 0xFA => {
                pc_inc_by = 1;
                2
            }
            0x04 | 0x44 | 0x64 => {
                pc_inc_by = 1;
                3
            }
            0x0C | 0x14 | 0x1C | 0x34 | 0x3C | 0x54 | 0x5C | 0x74 | 0x7C | 0xD4 | 0xDC | 0xF4 => {
                pc_inc_by = 1;
                4
            }

            // Shouldn't ever happen.
            // NOTE: can use unreachable!() to tell the compiler this ^
            otherwise => {
                panic!("Opcode 0x{:X} has not yet been implemented", otherwise);
            }
        };

        self.pc += pc_inc_by;
        time_passed
    }
}
