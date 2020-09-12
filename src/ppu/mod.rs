//! The Pixel Processing Unit.
//!
//! Used to draw to the screen.

use crate::getset::GetSet;

#[derive(Debug, Clone, Default)]
pub struct Ppu {
    /// 0x2000
    /// Write only
    control1: u8,

    /// 0x2001
    /// Write only
    control2: u8,

    /// 0x2002
    /// Read only
    status_register: u8,

    /// 0x2003
    /// Write only
    sprite_ram_address: u8,

    /// 0x2004
    /// Read/Write
    sprite_ram_data: u8,

    /// 0x4014 (does this belong here?)
    sprite_dma: u8,

    /// 0x2005
    /// first write
    ppu_background_scroll_x: u8,

    /// 0x2005
    /// second write
    ppu_background_scroll_y: u8,

    /// Keeps track of which byte is being written to.
    ppu_background_scroll_offset_byte: bool,

    /// 0x2006
    /// Write twice
    vram_address: u16,

    /// Keeps track of which byte is being written to.
    vram_address_byte: bool,

    /// 0x2007
    /// Read/Write
    vram_data: u8,
}

impl Ppu {
    /// Construct a `Ppu` in its starting state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of cycles that have passed.
    pub fn run_instruction<MEM: GetSet, MAP: GetSet>(&mut self, ppu_mem: MEM, ppu_map: MAP) -> u8 {
        0
    }
}

impl GetSet for Ppu {
    fn get(&self, address: u16) -> u8 {
        todo!()
    }
    /// It's undefined behavior to give an address that's not between
    /// 0x2000 and 0x3FFF inclusive.
    fn set(&mut self, address: u16, value: u8) {
        match address & 0x7 {
            0 => self.control1 = value,
            1 => self.control2 = value,
            2 => self.status_register = value,
            3 => self.sprite_ram_address = value,
            4 => self.sprite_ram_data = value,
            5 => {
                if self.ppu_background_scroll_offset_byte {
                    self.ppu_background_scroll_y = value;
                } else {
                    self.ppu_background_scroll_x = value;
                }
                self.vram_address_byte = !self.vram_address_byte;
            }
            6 => {
                if self.vram_address_byte {
                    todo!("figure out what to do here, second write causes the first write to update?")
                } else {
                    todo!("figure out what to do here, first write does not update")
                }
                self.vram_address_byte = !self.vram_address_byte;
            }
            7 => self.vram_data = value,
            _ => unreachable!("Todo replace this with the unsafe unreachable"),
        }
    }
}
