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
            // LDA
            0x49 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = imm(data);
                lda(self, &ram, opop);
                2
            }

            0xA5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zp(data);
                lda(self, &ram, opop);
                3
            }

            0xB5 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = zpx(data, self.x);
                lda(self, &ram, opop);
                4
            }

            0xA1 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                lda(self, &ram, opop);
                6
            }

            0xB1 => {
                let data = get_byte(&rom, &mut self.pc);
                let (opop, add_cycle) = izy(&ram, data, self.y);
                lda(self, &ram, opop);
                5 + add_cycle as u8
            }

            0xAD => {
                let data = get_word(&rom, &mut self.pc);
                let opop = abs(data);
                lda(self, &ram, opop);
                4
            }

            0xBD => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = abx(data, self.x);
                lda(self, &ram, opop);
                4 + add_cycle as u8
            }

            0xB9 => {
                let data = get_word(&rom, &mut self.pc);
                let (opop, add_cycle) = aby(data, self.y);
                lda(self, &ram, opop);
                4 + add_cycle as u8
            }

            // STA
            0x81 => {
                let data = get_byte(&rom, &mut self.pc);
                let opop = izx(&ram, data, self.x);
                sta(&self, ram, opop);
                6
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
