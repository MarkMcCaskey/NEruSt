use cpu::addressing_modes::*;
use cpu::opcode_logic::*;
use ram::ram::*;
use rom::rom::*;

pub struct Cpu {
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    /// process status register
    pub p: u8,
    pub pc: u16,
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
            p: 0,
            pc: 0,
        }
    }

    pub fn set_flag_value(&mut self, flag: ProcessorStatusFlag, val: bool) {
        let bit = (val as u8) << (flag as u8);
        self.p &= !bit;
        self.p |= bit;
    }

    pub fn get_processor_status_flag(&self, flag: ProcessorStatusFlag) -> bool {
        let flag_bit = 1 << (flag as u8);
        (self.p & flag_bit) == flag_bit
    }

    pub fn run_instruction(&mut self, ram: &mut Ram, rom: &Rom) -> u8 {
        let op = get_byte(&rom, &mut self.pc);
        match op {
            0x01 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                ora(self, &ram, opop);
                // unverified; same with other recently added opcodes to dispatch;
                // TODO: use gitblame and update recently added time delays
                6
            }

            0x05 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                ora(self, &ram, opop);
                3
            }

            0x06 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                asl(self, ram, opop);
                4
            }

            0x09 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                ora(self, &ram, opop);
                2
            }

            0x0A => {
                let opop = imp();
                asl(self, ram, opop);
                4
            }

            0x0D => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                ora(self, &ram, opop);
                4
            }

            0x0E => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                asl(self, ram, opop);
                4
            }

            0x11 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.y);
                ora(self, &ram, opop);
                5 + add_cycle as u8
            }

            0x15 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                ora(self, &ram, opop);
                4
            }

            0x16 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                asl(self, ram, opop);
                4
            }

            0x19 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                ora(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x1D => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                ora(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x1E => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                asl(self, ram, opop);
                4 + add_cycle as u8
            }

            0x21 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                and(self, &ram, opop);
                6
            }

            0x25 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                and(self, &ram, opop);
                3
            }

            0x26 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                rol(self, ram, opop);
                4
            }

            0x29 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                and(self, &ram, opop);
                2
            }

            0x2A => {
                let opop = imp();
                rol(self, ram, opop);
                4
            }

            0x2D => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                and(self, &ram, opop);
                4
            }

            0x2E => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                rol(self, ram, opop);
                4
            }

            0x31 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.y);
                and(self, &ram, opop);
                5 + add_cycle as u8
            }

            0x35 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                and(self, &ram, opop);
                4
            }

            0x36 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                rol(self, ram, opop);
                4
            }

            0x39 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                and(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x3D => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                and(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x3E => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                rol(self, ram, opop);
                4 + add_cycle as u8
            }

            0x41 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                eor(self, &ram, opop);
                6
            }

            0x45 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                eor(self, &ram, opop);
                3
            }

            0x46 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                lsr(self, ram, opop);
                4
            }

            0x49 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                eor(self, &ram, opop);
                2
            }

            0x4A => {
                let opop = imp();
                lsr(self, ram, opop);
                4
            }

            0x4D => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                eor(self, &ram, opop);
                4
            }

            0x4E => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                lsr(self, ram, opop);
                4
            }

            0x51 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.y);
                eor(self, &ram, opop);
                5 + add_cycle as u8
            }

            0x55 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                eor(self, &ram, opop);
                4
            }

            0x56 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                lsr(self, ram, opop);
                4
            }

            0x59 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                eor(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x5D => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                eor(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x5E => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                lsr(self, ram, opop);
                4 + add_cycle as u8
            }

            0x61 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                adc(self, &ram, opop);
                6
            }

            0x65 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                adc(self, &ram, opop);
                3
            }

            0x66 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                ror(self, ram, opop);
                4
            }

            0x69 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                adc(self, &ram, opop);
                2
            }

            0x6A => {
                let opop = imp();
                ror(self, ram, opop);
                4
            }

            0x6D => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                adc(self, &ram, opop);
                4
            }

            0x6E => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                ror(self, ram, opop);
                4
            }

            0x71 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.y);
                adc(self, &ram, opop);
                5 + add_cycle as u8
            }

            0x75 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                adc(self, &ram, opop);
                4
            }

            0x76 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                ror(self, ram, opop);
                4
            }

            0x79 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                adc(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x7D => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                adc(self, &ram, opop);
                4 + add_cycle as u8
            }

            0x7E => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                ror(self, ram, opop);
                4 + add_cycle as u8
            }

            0x81 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                sta(&self, ram, opop);
                6
            }

            0x88 => {
                dey(self);
                4
            }

            0xA1 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                lda(self, &ram, opop);
                6
            }

            0xA5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                lda(self, &ram, opop);
                3
            }

            0xA9 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                lda(self, &ram, opop);
                2
            }

            0xAD => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                lda(self, &ram, opop);
                4
            }

            0xB1 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.y);
                lda(self, &ram, opop);
                5 + add_cycle as u8
            }

            0xB5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                lda(self, &ram, opop);
                4
            }

            0xB9 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                lda(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xBD => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                lda(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xC0 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                cpy(self, &ram, opop);
                4
            }

            0xC1 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                cmp(self, &ram, opop);
                4
            }

            0xC4 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                cpy(self, &ram, opop);
                4
            }

            0xC5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                cmp(self, &ram, opop);
                4
            }

            0xC6 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                dec(self, ram, opop);
                4
            }

            0xC8 => {
                iny(self);
                4
            }

            0xC9 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                cmp(self, &ram, opop);
                4
            }

            0xCA => {
                dex(self);
                4
            }

            0xCC => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                cpy(self, &ram, opop);
                4
            }

            0xCD => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                cmp(self, &ram, opop);
                4
            }

            0xCE => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                dec(self, ram, opop);
                4
            }

            0xD1 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.x);
                cmp(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xD5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                cmp(self, &ram, opop);
                4
            }

            0xD6 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                dec(self, ram, opop);
                4
            }

            0xD9 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                cmp(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xDD => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                cmp(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xDE => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                dec(self, ram, opop);
                4 + add_cycle as u8
            }

            0xE0 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                cpx(self, &ram, opop);
                4
            }

            0xE1 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                sbc(self, &ram, opop);
                6
            }

            0xE4 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                cpx(self, &ram, opop);
                4
            }

            0xE5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                sbc(self, &ram, opop);
                3
            }

            0xE6 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                inc(self, ram, opop);
                4
            }

            0xE8 => {
                inx(self);
                4
            }

            0xE9 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                sbc(self, &ram, opop);
                2
            }

            0xEC => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                cpx(self, &ram, opop);
                4
            }

            0xED => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                sbc(self, &ram, opop);
                4
            }

            0xEE => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                inc(self, ram, opop);
                4
            }

            0xF1 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.y);
                sbc(self, &ram, opop);
                5 + add_cycle as u8
            }

            0xF5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                sbc(self, &ram, opop);
                4
            }

            0xF6 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                inc(self, ram, opop);
                4
            }

            0xF9 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                sbc(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xFD => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                sbc(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xFE => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                inc(self, ram, opop);
                4 + add_cycle as u8
            }

            // Shouldn't ever happen. If it does... well, yuh dun fuck'd son
            // NOTE: can use unreachable!() to tell the compiler this ^
            _ => {
                unreachable!();
                255
            }
        }
    }
}
