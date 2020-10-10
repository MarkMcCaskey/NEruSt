//! The Pixel Processing Unit.
//!
//! Used to draw to the screen.

use crate::nes::Nes;

#[derive(Debug, Clone, Default)]
pub struct Ppu {
    bus: u8, // [TODO: explain]

    // status flags
    ppuctrl: u8,   // $2000
    ppumask: u8,   // $2001
    ppustatus: u8, // $2002

    // oam
    oamaddr: u8, // $2003

    // ppu
    ppuaddr: u16, // $2006

    oamdma: u8, // $4014 ?
}

impl Ppu {
    /// Construct a `Ppu` in its starting state.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Nes {
    /// Simulates a certain number of PPU cycles.
    /// Returns vblank.
    pub fn step_ppu(&mut self, cycles: u8) -> bool {
        false
    }

    pub fn ppu_read_reg(&mut self, address: u16) -> u8 {
        match address {
            0x2000 => unreachable!(),
            0x2001 => unreachable!(),
            0x2002 => {
                // the first 5 bits are whatever was previously written in a PPU register
                let r = self.ppu.ppustatus | self.ppu.bus;
                self.ppu.ppustatus &= 0b0110_0000; // clear first bit (and first 5)
                r
            }
            0x2003 => unreachable!(),
            0x2004 => todo!("OAM data read"),
            0x2005 => unreachable!(),
            0x2006 => unreachable!(),
            0x2007 => todo!("PPU data read"),
            0x4014 => unreachable!(),
            _ => unimplemented!(),
        }
    }
    /// It's undefined behavior to give an address that's not between
    /// 0x2000 and 0x3FFF inclusive.
    pub fn ppu_write_reg(&mut self, address: u16, value: u8) {
        self.ppu.bus = value;
        match address {
            0x2000 => self.ppu.ppuctrl = value,
            0x2001 => self.ppu.ppumask = value,
            0x2002 => unreachable!(),
            0x2003 => self.ppu.oamaddr = value,
            0x2004 => todo!("OAM data write"),
            0x2005 => todo!("scroll"),
            0x2006 => self.ppu.ppuaddr = (self.ppu.ppuaddr << 8) | (value as u16),
            0x2007 => todo!("PPU data write"),
            0x4014 => todo!("OAM DMA"),
            _ => unreachable!(),
        }
    }
}
