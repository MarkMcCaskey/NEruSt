//! The audio processing unit.

#[derive(Debug, Default)]
pub struct Apu {
    /// 0x4000
    p1_timer: u8,
    /// 0x4001
    p1_length: u8,
    /// 0x4002
    p1_envelope: u8,
    /// 0x4003
    p1_sweep: u8,

    /// 0x4004
    p2_timer: u8,
    /// 0x4005
    p2_length: u8,
    /// 0x4006
    p2_envelope: u8,
    /// 0x4007
    p2_sweep: u8,

    /// 0x4008
    triangle_timer: u8,
    /// 0x4009
    triangle_length: u8,
    /// 0x400A
    triangle_linear_counter: u8,

    /// 0x400C
    noise_timer: u8,
    /// 0x400D
    noise_length: u8,
    /// 0x400E
    noise_envelope: u8,
    /// 0x400E
    noise_linear_feedback_shift_register: u8,

    /// 0x4010
    dmc_timer: u8,
    /// 0x4011
    dmc_memory_reader: u8,
    /// 0x4012
    dmc_sample_buffer: u8,
    /// 0x4013
    dmc_output_unit: u8,

    /// 0x4015
    channel_status: u8,

    /// 0x4017
    frame_counter: u8,
}

impl Apu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // channel 1
            0x4000 => self.p1_timer,
            0x4001 => self.p1_length,
            0x4002 => self.p1_envelope,
            0x4003 => self.p1_sweep,

            // channel 2
            0x4004 => self.p2_timer,
            0x4005 => self.p2_length,
            0x4006 => self.p2_envelope,
            0x4007 => self.p2_sweep,

            // channel 3: triangle
            0x4008 => self.triangle_timer,
            0x4009 => self.triangle_length,
            0x400A => self.triangle_linear_counter,

            // channel 4: noise
            0x400B => self.noise_timer,
            0x400C => self.noise_length,
            0x400D => self.noise_envelope,
            0x400E => self.noise_linear_feedback_shift_register,

            // channel 5: dmc
            0x4010 => self.dmc_timer,
            0x4011 => self.dmc_memory_reader,
            0x4012 => self.dmc_sample_buffer,
            0x4013 => self.dmc_output_unit,

            // misc
            0x4015 => self.channel_status,
            0x4017 => self.frame_counter,
            _ => {
                error!(
                    "Tried to read from address 0x{:X}, but that's an invalid address for the PPU",
                    addr
                );
                0
            }
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        warn!(
            "Write to 0x{:X} ignored: sound not fully implemented yet",
            addr
        );
        match addr {
            // channel 1
            0x4000 => self.p1_timer = val,
            0x4001 => self.p1_length = val,
            0x4002 => self.p1_envelope = val,
            0x4003 => self.p1_sweep = val,

            // channel 2
            0x4004 => self.p2_timer = val,
            0x4005 => self.p2_length = val,
            0x4006 => self.p2_envelope = val,
            0x4007 => self.p2_sweep = val,

            // channel 3: triangle
            0x4008 => self.triangle_timer = val,
            0x4009 => self.triangle_length = val,
            0x400A => self.triangle_linear_counter = val,

            // channel 4: noise
            0x400B => self.noise_timer = val,
            0x400C => self.noise_length = val,
            0x400D => self.noise_envelope = val,
            0x400E => self.noise_linear_feedback_shift_register = val,

            // channel 5: dmc
            0x4010 => self.dmc_timer = val,
            0x4011 => self.dmc_memory_reader = val,
            0x4012 => self.dmc_sample_buffer = val,
            0x4013 => self.dmc_output_unit = val,

            // misc
            0x4015 => self.channel_status = val,
            0x4017 => self.frame_counter = val,
            _ => {
                error!(
                    "Tried to write value {} to address 0x{:X}, but that's an invalid address for the PPU",
                    val, addr
                );
            }
        }
    }
}
