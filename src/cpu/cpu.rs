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

    pub fn get_processor_status_flag(&self, flag: ProcessorStatusFlag) -> bool {
        ((self.p >> (flag as u8)) & 1) == 1
    }

    pub fn set_processor_status_flag(&mut self, flag: ProcessorStatusFlag) {
        let active_flag_bit = 1 << (flag as u8);
        let flag_mask = !active_flag_bit;

        //clear flag
        self.p &= flag_mask;

        //set flag
        self.p |= active_flag_bit;
    }

    pub fn clear_processor_status_flag(&mut self, flag: ProcessorStatusFlag) {
        let active_flag_bit = 1 << (flag as u8);
        let flag_mask = !active_flag_bit;

        //clear flag
        self.p &= flag_mask;
    }
}
