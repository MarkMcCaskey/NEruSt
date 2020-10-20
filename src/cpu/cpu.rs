use crate::cpu::addressing_modes::*;
use crate::cpu::opcode_logic::*;
use crate::nes::Nes;

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
    // interrupt
    pub reset: bool,
    pub nmi: bool,
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
            s: 0x00,
            halt: false,
            reset: true,
            nmi: false,
        }
    }

    pub fn reset(&mut self) {
        self.reset = true;
    }

    pub fn nmi(&mut self) {
        self.nmi = true;
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
}

impl Nes {
    pub fn step_cpu(&mut self) -> u8 {
        let op = self.cpu_read(self.cpu.pc);

        /*trace!(
            "CPU: {:04X} {:2X}    A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.cpu.pc,
            op,
            self.cpu.acc,
            self.cpu.x,
            self.cpu.y,
            self.cpu.p,
            self.cpu.s
        );*/

        // reset interrupt
        if self.cpu.reset {
            trace!("Interrupt: reset");
            self.cpu.reset = false;

            // reset supresses stack writes, so just offset by 3
            self.cpu.s = self.cpu.s.wrapping_sub(3);

            // set PC to address at $FFFC (reset vector)
            let lo = self.cpu_read(0xFFFC);
            let hi = self.cpu_read(0xFFFD);
            self.cpu.pc = lo as u16 | ((hi as u16) << 8);

            // set interrupt flag
            self.cpu.set_flag(ProcessorStatusFlag::Interrupt);

            // interrupts take 7 cycles
            return 7;
        }

        // NMI interrupt
        if self.cpu.nmi {
            trace!("Interrupt: NMI");
            self.cpu.nmi = false;

            // push flags and PC to stack
            self.cpu.s = self.cpu.s.wrapping_sub(3);
            self.cpu_write(self.cpu.s.wrapping_add(1) as u16 | 0x100, self.cpu.p); // this is wrong
            self.cpu_write(self.cpu.s.wrapping_add(2) as u16 | 0x100, self.cpu.pc as u8);
            self.cpu_write(
                self.cpu.s.wrapping_add(3) as u16 | 0x100,
                (self.cpu.pc >> 8) as u8,
            );

            // set PC to address at $FFFA (NMI vector)
            let lo = self.cpu_read(0xFFFA);
            let hi = self.cpu_read(0xFFFB);
            self.cpu.pc = lo as u16 | ((hi as u16) << 8);

            // set interrupt flag
            self.cpu.set_flag(ProcessorStatusFlag::Interrupt);

            // interrupts take 7 cycles
            return 7;
        }

        // BRK eventually

        // little endian get16 for convenience
        let get16 = |nes: &mut Nes, addr| {
            ((nes.cpu_read(addr + 1) as u16) << 8) | nes.cpu_read(addr) as u16
        };

        let pc_inc_by;
        let cyc_inc_by;
        match op {
            0x00 => {
                self.brk();
                pc_inc_by = 1;
                cyc_inc_by = 0; // eh
            }

            0x01 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.ora(byte);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x05 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.ora(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x06 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.asl(addr);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x08 => {
                self.php();
                pc_inc_by = 1;
                cyc_inc_by = 3;
            }

            0x09 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.ora(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x0A => {
                self.asl_imp();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x0D => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.ora(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x0E => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.asl(addr);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x10 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.bpl(operand);
                //crate::log(&format!("Branch taken: BPL -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x11 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, add_cycle) = self.izy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.ora(byte);
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x15 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.ora(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x16 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.asl(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x18 => {
                self.clc();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x19 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.ora(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x1D => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.ora(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x1E => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.abx(operand, self.cpu.x);
                self.asl(addr);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x20 => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.jsr(addr);
                pc_inc_by = 0;
                cyc_inc_by = 6;
            }

            0x21 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.and(byte);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x24 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.bit(addr);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x25 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.and(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x26 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.rol(addr);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x28 => {
                self.plp();
                pc_inc_by = 1;
                cyc_inc_by = 4;
            }

            0x29 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.and(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x2A => {
                self.rol_imp();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x2C => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.bit(addr);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }
            0x2D => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.and(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x2E => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.rol(addr);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x30 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.bmi(operand);
                //crate::log(&format!("Branch taken: BMI -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x31 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, add_cycle) = self.izy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.and(byte);
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x35 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.and(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x36 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.rol(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x38 => {
                self.sec();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x39 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.and(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x3D => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.and(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x3E => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.abx(operand, self.cpu.x);
                self.rol(addr);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x40 => {
                self.rti();
                pc_inc_by = 0; // rti as subtler behaviors with pc
                cyc_inc_by = 6;
            }

            0x41 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.eor(byte);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x45 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.eor(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x46 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.lsr(addr);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x48 => {
                self.pha();
                pc_inc_by = 1;
                cyc_inc_by = 3;
            }

            0x49 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.eor(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x4A => {
                self.lsr_imp();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x4C => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.jmp(addr);
                pc_inc_by = 0;
                cyc_inc_by = 3;
            }

            0x4D => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.eor(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x4E => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.lsr(addr);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x50 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.bvc(operand);
                //crate::log(&format!("Branch taken: BVC -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x51 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, add_cycle) = self.izy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.eor(byte);
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x55 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.eor(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x56 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.lsr(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x58 => {
                self.cli();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x59 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.eor(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x5D => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.eor(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x5E => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.abx(operand, self.cpu.x);
                self.lsr(addr);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x60 => {
                self.rts();
                pc_inc_by = 0; // rts as subtler behaviors with pc
                cyc_inc_by = 6;
            }

            0x61 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.adc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x65 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.adc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x66 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.ror(addr);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0x68 => {
                self.pla();
                pc_inc_by = 1;
                cyc_inc_by = 4;
            }

            0x69 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.adc(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0x6A => {
                self.ror_imp();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x6C => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.ind(operand);
                self.jmp(addr);
                pc_inc_by = 0;
                cyc_inc_by = 5;
            }

            0x6D => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.adc(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x6E => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.ror(addr);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0x70 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.bvs(operand);
                //crate::log(&format!("Branch taken: BVS -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x71 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, add_cycle) = self.izy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.adc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0x75 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.adc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x76 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.ror(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x78 => {
                self.sei();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x79 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.adc(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x7D => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.adc(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0x7E => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.abx(operand, self.cpu.x);
                self.ror(addr);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0x81 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                self.sta(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x84 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.sty(addr);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x85 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.sta(addr);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x86 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.stx(addr);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0x88 => {
                self.dey();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x8A => {
                self.txa();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x8C => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.sty(addr);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x8D => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.sta(addr);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x8E => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.stx(addr);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0x90 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.bcc(operand);
                //crate::log(&format!("Branch taken: BCC -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0x91 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, _) = self.izy(operand, self.cpu.y);
                self.sta(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0x94 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.sty(addr);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x95 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.sta(addr);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x96 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpy(operand, self.cpu.y);
                self.stx(addr);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0x98 => {
                self.tya();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x99 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.aby(operand, self.cpu.y);
                self.sta(addr);
                pc_inc_by = 3;
                cyc_inc_by = 5;
            }

            0x9A => {
                self.txs();
                // TODO: review
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0x9D => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.abx(operand, self.cpu.x);
                self.sta(addr);
                pc_inc_by = 3;
                cyc_inc_by = 5;
            }

            0xA0 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.ldy(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xA1 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.lda(byte);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xA2 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.ldx(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xA4 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.ldy(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xA5 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.lda(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xA6 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.ldx(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xA8 => {
                self.tay();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xA9 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.lda(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xAA => {
                self.tax();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xAC => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.ldy(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xAD => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.lda(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xAE => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.ldx(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xB0 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.bcs(operand);
                //crate::log(&format!("Branch taken: BCS -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0xB1 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, add_cycle) = self.izy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.lda(byte);
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0xB4 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.ldy(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xB5 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.lda(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xB6 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.ldx(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xB8 => {
                self.clv();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xB9 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.lda(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xBA => {
                self.tsx();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xBC => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.ldy(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xBD => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.lda(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xBE => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.ldx(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xC0 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.cpy(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xC1 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.cmp(byte);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xC4 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.cpy(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xC5 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.cmp(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xC6 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.dec(addr);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0xC8 => {
                self.iny();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xC9 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.cmp(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xCA => {
                self.dex();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xCC => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.cpy(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xCD => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.cmp(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xCE => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.dec(addr);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0xD0 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.bne(operand);
                //crate::log(&format!("Branch taken: BNE -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0xD1 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, add_cycle) = self.izy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.cmp(byte);
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0xD5 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.cmp(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xD6 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.dec(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xD8 => {
                self.cld();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xD9 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.cmp(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xDD => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.cmp(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xDE => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.abx(operand, self.cpu.x);
                self.dec(addr);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }

            0xE0 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.cpx(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xE1 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.izx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.sbc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xE4 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.cpx(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xE5 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                let byte = self.cpu_read(addr);
                self.sbc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 3;
            }

            0xE6 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zp(operand);
                self.inc(addr);
                pc_inc_by = 2;
                cyc_inc_by = 5;
            }

            0xE8 => {
                self.inx();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xE9 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                self.sbc(operand);
                pc_inc_by = 2;
                cyc_inc_by = 2;
            }

            0xEC => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.cpx(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xED => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                let byte = self.cpu_read(addr);
                self.sbc(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4;
            }

            0xEE => {
                let operand = get16(self, self.cpu.pc + 1);
                let addr = self.abs(operand);
                self.inc(addr);
                pc_inc_by = 3;
                cyc_inc_by = 6;
            }

            0xF0 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let result = self.beq(operand);
                //crate::log(&format!("Branch taken: BEQ -> {:04X}", self.cpu.pc));
                pc_inc_by = 2;
                cyc_inc_by = 2 + result; // + page boundary crossed
            }

            0xF1 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let (addr, add_cycle) = self.izy(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.sbc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 5 + add_cycle as u8;
            }

            0xF5 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.sbc(byte);
                pc_inc_by = 2;
                cyc_inc_by = 4;
            }

            0xF6 => {
                let operand = self.cpu_read(self.cpu.pc + 1);
                let addr = self.zpx(operand, self.cpu.x);
                self.inc(addr);
                pc_inc_by = 2;
                cyc_inc_by = 6;
            }

            0xF8 => {
                self.sed();
                pc_inc_by = 1;
                cyc_inc_by = 2;
            }

            0xF9 => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.aby(operand, self.cpu.y);
                let byte = self.cpu_read(addr);
                self.sbc(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xFD => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, add_cycle) = self.abx(operand, self.cpu.x);
                let byte = self.cpu_read(addr);
                self.sbc(byte);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            0xFE => {
                let operand = get16(self, self.cpu.pc + 1);
                let (addr, _) = self.abx(operand, self.cpu.x);
                self.inc(addr);
                pc_inc_by = 3;
                cyc_inc_by = 7;
            }
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => {
                self.cpu.halt = true;
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
                let operand = get16(self, self.cpu.pc + 1);
                let (_, add_cycle) = self.abx(operand, self.cpu.x);
                pc_inc_by = 3;
                cyc_inc_by = 4 + add_cycle as u8;
            }

            // Shouldn't ever happen.
            // NOTE: can use unreachable!() to tell the compiler this ^
            otherwise => {
                error!("Opcode 0x{:X} has not yet been implemented", otherwise);
                panic!("Opcode 0x{:X} has not yet been implemented", otherwise);
            }
        };

        self.cpu.pc += pc_inc_by;
        cyc_inc_by
    }
}
