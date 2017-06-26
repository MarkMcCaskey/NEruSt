pub struct CPU {
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    /// process status register
    pub p: u8,
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

impl CPU {
    /// Initializes the CPU registers (currently placeholder values)
    pub fn new() -> Self {
        CPU {
            acc: 0,
            x: 0,
            y: 0,
            p: 0,
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
}