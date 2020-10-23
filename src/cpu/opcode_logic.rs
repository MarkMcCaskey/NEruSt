use crate::cpu::cpu::ProcessorStatusFlag;
use crate::nes::Nes;

impl Nes {
    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Load register opcodes
    pub fn lda(&mut self, val: u8) {
        self.cpu.acc = val;

        let zero_flag: bool = self.cpu.acc == 0;
        let negative_flag: bool = self.cpu.acc & 0x80 == 0x80;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    }

    pub fn ldx(&mut self, val: u8) {
        self.cpu.x = val;

        let zero_flag: bool = self.cpu.x == 0;
        let negative_flag: bool = self.cpu.x & 0x80 == 0x80;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    }

    pub fn ldy(&mut self, val: u8) {
        self.cpu.y = val;

        let zero_flag: bool = self.cpu.y == 0;
        let negative_flag: bool = self.cpu.y & 0x80 == 0x80;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
    }

    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Set self.cpu_mapdes
    pub fn sta(&mut self, addr: u16) {
        self.cpu_write(addr, self.cpu.acc);
    }

    pub fn stx(&mut self, addr: u16) {
        self.cpu_write(addr, self.cpu.x);
    }

    pub fn sty(&mut self, addr: u16) {
        self.cpu_write(addr, self.cpu.y);
    }

    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Bitwise opcodes
    pub fn eor(&mut self, val: u8) {
        self.cpu.acc ^= val;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.acc == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, (self.cpu.acc as i8) < 0);
    }

    pub fn and(&mut self, val: u8) {
        self.cpu.acc &= val;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.acc == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, (self.cpu.acc as i8) < 0);
    }

    pub fn ora(&mut self, val: u8) {
        self.cpu.acc |= val;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.acc == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, (self.cpu.acc as i8) < 0);
    }

    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Set/clear flag opcodes
    pub fn clc(&mut self) {
        self.cpu.clear_flag(ProcessorStatusFlag::Carry);
    }

    pub fn cld(&mut self) {
        self.cpu.clear_flag(ProcessorStatusFlag::Decimal);
    }

    pub fn cli(&mut self) {
        self.cpu.clear_flag(ProcessorStatusFlag::Interrupt);
    }

    pub fn clv(&mut self) {
        self.cpu.clear_flag(ProcessorStatusFlag::Overflow);
    }

    pub fn sec(&mut self) {
        self.cpu.set_flag(ProcessorStatusFlag::Carry);
    }

    pub fn sed(&mut self) {
        self.cpu.set_flag(ProcessorStatusFlag::Decimal);
    }

    pub fn sei(&mut self) {
        self.cpu.set_flag(ProcessorStatusFlag::Interrupt);
    }

    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Transfer opcodes
    pub fn tax(&mut self) {
        self.cpu.x = self.cpu.acc;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.acc == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.acc & 0x80 == 0x80);
    }

    pub fn txa(&mut self) {
        self.cpu.acc = self.cpu.x;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.x == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.x & 0x80 == 0x80);
    }

    pub fn tay(&mut self) {
        self.cpu.y = self.cpu.acc;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.acc == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.acc & 0x80 == 0x80);
    }

    pub fn tya(&mut self) {
        self.cpu.acc = self.cpu.y;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.y == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.y & 0x80 == 0x80);
    }

    pub fn tsx(&mut self) {
        self.cpu.x = self.cpu.s;
        let is_zero = self.cpu.s == 0;
        let is_neg = (self.cpu.s as i8) < 0;
        self.cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, is_neg);
    }

    pub fn txs(&mut self) {
        self.cpu.s = self.cpu.x;
    }

    pub fn pla(&mut self) {
        self.cpu.s = self.cpu.s.wrapping_add(1);
        let idx = self.cpu.s as u16 + 0x100;
        self.cpu.acc = self.cpu_read(idx);

        let is_zero = self.cpu.acc == 0;
        let is_neg = (self.cpu.acc as i8) < 0;
        self.cpu.set_flag_value(ProcessorStatusFlag::Zero, is_zero);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, is_neg);
    }

    pub fn pha(&mut self) {
        let idx = self.cpu.s as u16 + 0x100;
        self.cpu_write(idx, self.cpu.acc);
        self.cpu.s = self.cpu.s.wrapping_sub(1);
    }

    // TODO: possibly wrong
    pub fn plp(&mut self) {
        self.cpu.s = self.cpu.s.wrapping_add(1);
        let idx = self.cpu.s as u16 + 0x100;
        self.cpu.p = self.cpu_read(idx) & 0b11001111 | 0b00100000;
    }

    // TODO: possibly wrong
    pub fn php(&mut self) {
        let idx = self.cpu.s as u16 + 0x100;
        self.cpu_write(idx as u16, self.cpu.p | 0b00110000);
        self.cpu.s = self.cpu.s.wrapping_sub(1);
    }

    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Increment opcodes
    pub fn dex(&mut self) {
        self.cpu.x = self.cpu.x.wrapping_sub(1);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.x == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.x & 0x80 == 0x80);
    }

    pub fn inx(&mut self) {
        self.cpu.x = self.cpu.x.wrapping_add(1);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.x == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.x & 0x80 == 0x80);
    }

    pub fn dey(&mut self) {
        self.cpu.y = self.cpu.y.wrapping_sub(1);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.y == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.y & 0x80 == 0x80);
    }

    pub fn iny(&mut self) {
        self.cpu.y = self.cpu.y.wrapping_add(1);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.y == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, self.cpu.y & 0x80 == 0x80);
    }

    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Math opcodes
    pub fn adc(&mut self, val: u8) {
        let old_acc = self.cpu.acc;
        self.cpu.acc = self.cpu.acc.wrapping_add(val).wrapping_add(
            self.cpu
                .get_processor_status_flag(ProcessorStatusFlag::Carry) as u8,
        );

        let negative_flag = self.cpu.acc & 0x80 == 0x80;
        let overflow_flag = (!old_acc & !val & self.cpu.acc & 0x80) == 0x80
            || (old_acc & val & !self.cpu.acc & 0x80) == 0x80;
        let carry_flag = self.cpu.acc < old_acc;
        let zero_flag = self.cpu.acc == 0;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Overflow, overflow_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    }

    pub fn sbc(&mut self, val: u8) {
        self.adc(!val);
        /*
        let old_acc: u8 = self.cpu.acc;
        self.cpu.acc = self.cpu.acc.wrapping_sub(val);
        let negative_flag = self.cpu.acc & 0x80 == 0x80;
        let overflow_flag =
            (!old_acc & !val & self.cpu.acc & 0x80) == 0x80 || (old_acc & val & !self.cpu.acc & 0x80) == 0x80;
        let carry_flag = self.cpu.acc > old_acc;
        let zero_flag = self.cpu.acc == 0;
        self.cpu.set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu.set_flag_value(ProcessorStatusFlag::Overflow, overflow_flag);
        self.cpu.set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu.set_flag_value(ProcessorStatusFlag::Carry, carry_flag);*/
    }

    //////////////////////////////////////////////////
    //////////////////////////////////////////////////
    ///// Shift opcodes
    pub fn rol_imp(&mut self) {
        let (old_val, new_val) = {
            let old_val = self.cpu.acc;
            self.cpu.acc <<= 1;
            self.cpu.acc |=
                self.cpu
                    .get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            (old_val, self.cpu.acc)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x80 == 0x80;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    pub fn rol(&mut self, addr: u16) {
        let (old_val, new_val) = {
            let old_val = self.cpu_read(addr);
            let mut temp = old_val;
            temp <<= 1;
            temp |= self
                .cpu
                .get_processor_status_flag(ProcessorStatusFlag::Carry) as u8;
            self.cpu_write(addr, temp);
            (old_val, temp)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x80 == 0x80;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    pub fn ror_imp(&mut self) {
        let (old_val, new_val) = {
            let old_val = self.cpu.acc;
            self.cpu.acc >>= 1;
            self.cpu.acc |= (self
                .cpu
                .get_processor_status_flag(ProcessorStatusFlag::Carry)
                as u8)
                << 7;
            (old_val, self.cpu.acc)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x01 == 0x01;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    pub fn ror(&mut self, addr: u16) {
        let (old_val, new_val) = {
            let old_val = self.cpu_read(addr);
            let mut temp = old_val;
            temp >>= 1;
            temp |= (self
                .cpu
                .get_processor_status_flag(ProcessorStatusFlag::Carry) as u8)
                << 7;
            self.cpu_write(addr, temp);
            (old_val, temp)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x01 == 0x01;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    pub fn lsr_imp(&mut self) {
        let (old_val, new_val) = {
            let old_val = self.cpu.acc;
            self.cpu.acc >>= 1;
            (old_val, self.cpu.acc)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x01 == 0x01;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    pub fn lsr(&mut self, addr: u16) {
        let (old_val, new_val) = {
            let old_val = self.cpu_read(addr);
            let mut temp = old_val;
            temp >>= 1;
            self.cpu_write(addr, temp);
            (old_val, temp)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x01 == 0x01;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    pub fn asl_imp(&mut self) {
        let (old_val, new_val) = {
            let old_val = self.cpu.acc;
            self.cpu.acc <<= 1;
            (old_val, self.cpu.acc)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x80 == 0x80;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    pub fn asl(&mut self, addr: u16) {
        let (old_val, new_val) = {
            let old_val = self.cpu_read(addr);
            let mut temp = old_val;
            temp <<= 1;
            self.cpu_write(addr, temp);
            (old_val, temp)
        };

        let negative_flag = new_val & 0x80 == 0x80;
        let zero_flag = new_val == 0;
        let carry_flag = old_val & 0x80 == 0x80;
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
    }

    /// logic

    /// Function that implements the flag setting logic of `cmp`, `cpx`, and `cpy`
    fn common_cmp(&mut self, first: u8, second: u8) {
        let negative_flag = first.wrapping_sub(second) & 0x80 == 0x80;
        let carry_flag = first >= second;
        let zero_flag = first == second;

        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, negative_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Carry, carry_flag);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, zero_flag);
    }

    /*0110 0101
    X--- --XX*/

    pub fn cmp(&mut self, val: u8) {
        self.common_cmp(self.cpu.acc, val);
    }

    pub fn cpx(&mut self, val: u8) {
        self.common_cmp(self.cpu.x, val)
    }

    pub fn cpy(&mut self, val: u8) {
        self.common_cmp(self.cpu.y, val)
    }

    pub fn dec(&mut self, addr: u16) {
        let new_value = self.cpu_read(addr).wrapping_sub(1);
        self.cpu_write(addr, new_value);

        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
    }

    pub fn inc(&mut self, addr: u16) {
        let new_value = self.cpu_read(addr).wrapping_add(1);
        self.cpu_write(addr, new_value);

        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, new_value == 0);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, new_value & 0x80 == 0x80);
    }

    // Branching
    pub fn bpl(&mut self, val: u8) -> u8 {
        if !self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Negative)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i8 as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    pub fn bmi(&mut self, val: u8) -> u8 {
        if self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Negative)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i8 as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    pub fn bvc(&mut self, val: u8) -> u8 {
        if !self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Overflow)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i8 as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    pub fn bvs(&mut self, val: u8) -> u8 {
        if self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Overflow)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i8 as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    pub fn bcc(&mut self, val: u8) -> u8 {
        if !self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Carry)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i8 as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    pub fn bcs(&mut self, val: u8) -> u8 {
        if self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Carry)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i8 as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    pub fn bne(&mut self, val: u8) -> u8 {
        if !self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Zero)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i8 as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    pub fn beq(&mut self, val: u8) -> u8 {
        if self
            .cpu
            .get_processor_status_flag(ProcessorStatusFlag::Zero)
        {
            let _old_pc = self.cpu.pc;
            self.cpu.pc = (self.cpu.pc as i16 + val as i16) as u16;
            return 1 + (0/*self.cpu.pc & 0xFF00 != old_pc & 0xFF00*/) as u8;
        }

        0
    }

    // TODO: this function is probably wrong
    pub fn brk(&mut self) {
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Interrupt, true);
        self.cpu.set_flag_value(ProcessorStatusFlag::Break, true);
        self.cpu.nmi = true;
    }

    pub fn rti(&mut self) {
        self.cpu.p = self.cpu_read(self.cpu.s.wrapping_add(1) as u16 | 0x100) & 0xCF | 0x20;
        let lo = self.cpu_read(self.cpu.s.wrapping_add(2) as u16 | 0x100);
        let hi = self.cpu_read(self.cpu.s.wrapping_add(3) as u16 | 0x100);
        self.cpu.pc = lo as u16 | ((hi as u16) << 8);
        self.cpu.s = self.cpu.s.wrapping_add(3);
    }

    pub fn jsr(&mut self, addr: u16) {
        let push = self.cpu.pc.wrapping_add(3).wrapping_sub(1);
        self.cpu_write(self.cpu.s as u16 | 0x100, (push >> 8) as u8);
        self.cpu_write(self.cpu.s.wrapping_sub(1) as u16 | 0x100, push as u8);
        self.cpu.s = self.cpu.s.wrapping_sub(2);
        self.cpu.pc = addr
    }

    pub fn rts(&mut self) {
        let lo = self.cpu_read(self.cpu.s.wrapping_add(1) as u16 | 0x100);
        let hi = self.cpu_read(self.cpu.s.wrapping_add(2) as u16 | 0x100);
        self.cpu.pc = (lo as u16 | ((hi as u16) << 8)).wrapping_add(1);
        self.cpu.s = self.cpu.s.wrapping_add(2);
    }

    pub fn jmp(&mut self, addr: u16) {
        self.cpu.pc = addr;
    }

    pub fn bit(&mut self, addr: u16) {
        let val = self.cpu_read(addr);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Negative, (val >> 7) & 1 == 1);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Overflow, ((val >> 6) & 1) == 1);
        self.cpu
            .set_flag_value(ProcessorStatusFlag::Zero, self.cpu.acc & val == 0);
    }
}
