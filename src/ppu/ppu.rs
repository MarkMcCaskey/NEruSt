//! The Pixel Processing Unit.
//!
//! Used to draw to the screen.

use crate::nes::Nes;

#[derive(Debug, Clone, Default)]
pub struct Ppu {
    cycle: u32,
    bus: u8, // [TODO: explain]

    // status flags
    ppuctrl: u8,   // $2000
    ppumask: u8,   // $2001
    ppustatus: u8, // $2002

    // oam
    oamaddr: u8, // $2003

    x_scroll: u8, // $2005 first write
    y_scroll: u8, // $2005 second write
    /// whether to write to x_scroll or y_scroll
    scroll_bit: bool,

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
        //println!("PPU: {}/89342", self.ppu.cycle);
        self.ppu.cycle += cycles as u32;
        if self.ppu.cycle > 89342 {
            self.ppu.cycle -= 89342;
            self.ppu.ppustatus |= 0b1000_0000;
            return (self.ppu.ppuctrl & 0b1000_0000) > 0;
        }
        return false;
    }

    pub fn ppu_read_reg(&mut self, address: u16) -> u8 {
        match address & 0x7 {
            0x0 => unreachable!(),
            0x1 => unreachable!(),
            0x2 => {
                // the first 5 bits are whatever was previously written in a PPU register
                let r = self.ppu.ppustatus | self.ppu.bus;
                self.ppu.ppustatus &= 0b0110_0000; // clear first bit (and first 5)
                r
            }
            0x3 => unreachable!(),
            0x4 => todo!("OAM data read"),
            0x5 => unreachable!("PPU scroll registers: CPU should not need to access these"),
            0x6 => unreachable!(),
            0x7 => {
                let value = self.ppu_ram[self.ppu.ppuaddr as usize];
                let inc = (self.ppu.ppuctrl & 0b0100 > 0) as u16;
                self.ppu.ppuaddr = self.ppu.ppuaddr.wrapping_add(inc) & 0x3FFF;
                value
            }
            _ => unimplemented!(),
        }
    }
    /// It's undefined behavior to give an address that's not between
    /// 0x2000 and 0x3FFF inclusive.
    pub fn ppu_write_reg(&mut self, address: u16, value: u8) {
        if address == 0x4014 {
            todo!("OAM DMA")
        }
        self.ppu.bus = value;
        match address & 0x7 {
            0x0 => self.ppu.ppuctrl = value,
            0x1 => self.ppu.ppumask = value,
            0x2 => unreachable!(),
            0x3 => self.ppu.oamaddr = value,
            0x4 => todo!("OAM data write"),
            0x5 => {
                if !self.ppu.scroll_bit {
                    self.ppu.x_scroll = value;
                } else {
                    self.ppu.y_scroll = value;
                }
                self.ppu.scroll_bit = !self.ppu.scroll_bit;
            }
            0x6 => self.ppu.ppuaddr = (self.ppu.ppuaddr << 8) | (value as u16),
            0x7 => {
                self.ppu_ram[self.ppu.ppuaddr as usize] = value;
                let inc = (self.ppu.ppuctrl & 0b0100 > 0) as u16;
                self.ppu.ppuaddr = self.ppu.ppuaddr.wrapping_add(inc) & 0x3FFF;
            }
            _ => unreachable!(),
        }
    }
}
