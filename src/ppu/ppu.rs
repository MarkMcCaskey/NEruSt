//! The Pixel Processing Unit.
//!
//! Used to draw to the screen.

use crate::nes::Nes;

#[derive(Debug, Clone, Default)]
pub struct Ppu {
    scanline: u16,
    cycle: u16,

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
    ///
    pub fn get_frame(&mut self) -> Box<[u8]> {
        let color1 = [255, 0, 0];
        let color2 = [0, 255, 0];
        let color3 = [0, 0, 255];
        let color4 = [255, 255, 255];
        let colors = [color1, color2, color3, color4];

        let mut out = Box::new([0u8; 30 * 32 * 3]);

        // loop the attribute table 0x2000 .. 0x2800
        for col in 0..32 {
            for row in 0..30 {
                // get the pattern index from the nametable at (row, col)
                let pattern = self.ppu_read(0x2000 + row + col * 30) as u16;

                // extract the 16 bytes from the pattern table
                let b01 = self.ppu_read(pattern << 4 | 0x0);
                let b02 = self.ppu_read(pattern << 4 | 0x1);
                let b03 = self.ppu_read(pattern << 4 | 0x2);
                let b04 = self.ppu_read(pattern << 4 | 0x3);
                let b05 = self.ppu_read(pattern << 4 | 0x4);
                let b06 = self.ppu_read(pattern << 4 | 0x5);
                let b07 = self.ppu_read(pattern << 4 | 0x6);
                let b08 = self.ppu_read(pattern << 4 | 0x7);
                let b09 = self.ppu_read(pattern << 4 | 0x8);
                let b10 = self.ppu_read(pattern << 4 | 0x9);
                let b11 = self.ppu_read(pattern << 4 | 0xA);
                let b12 = self.ppu_read(pattern << 4 | 0xB);
                let b13 = self.ppu_read(pattern << 4 | 0xC);
                let b14 = self.ppu_read(pattern << 4 | 0xD);
                let b15 = self.ppu_read(pattern << 4 | 0xE);
                let b16 = self.ppu_read(pattern << 4 | 0xF);

                // extract pixel data from the above 16 bytes
                let bs1 = [b01, b02, b03, b04, b05, b06, b07, b08];
                let bs2 = [b09, b10, b11, b12, b13, b14, b15, b16];
                for index in 0..8 {
                    for pixel in 0..8 {
                        let pal_index =
                            (bs2[index] >> pixel) & 0x1 | ((bs1[index] >> pixel) & 0x1 << 1);
                        let color = colors[pal_index as usize];

                        let y = col as usize * 8 + index;
                        let x = row as usize * 8 + pixel;
                        let oi = x + y * 30 * 8 * 3;
                        out[oi + 0] = color[0];
                        out[oi + 1] = color[1];
                        out[oi + 2] = color[2];
                    }
                }
            }
        }

        out
    }

    /// Simulates a certain number of PPU cycles.
    /// Returns vblank.
    pub fn step_ppu(&mut self, cycles: u8) -> bool {
        let mut out = false;
        for _ in 0..cycles {
            match (self.ppu.scanline, self.ppu.cycle) {
                // scanlines 0-239 (render)
                (0..=239, 0) => { /* Idle */ }
                (0..=239, 1..=256) => { /* Draw */ }
                (0..=239, 257..=320) => { /* Next SL sprites */ }
                (0..=239, 321..=336) => { /* Next SL tiles */ }
                (0..=239, 337..=340) => { /* Dummy fetches */ }

                // scanline 240 (post-render)
                (240, _) => { /* Do nothing */ }

                // scanlines 241-260 (vblank)
                (241, 1) => {
                    // vblank
                    self.ppu.ppustatus |= 0b1000_0000;
                    out = (self.ppu.ppuctrl & 0b1000_0000) > 0;
                }
                (241..=260, _) => { /* Do nothing */ }

                // scanline 261 (pre-render)
                (261, _) => { /* Pre render scanline */ }
                _ => unreachable!(),
            }

            // advance ppu/scanline
            self.ppu.cycle += 1;
            if self.ppu.cycle > 340 {
                self.ppu.cycle = 0;
                self.ppu.scanline += 1;
                if self.ppu.scanline > 261 {
                    self.ppu.scanline = 0;
                }
            }
        }
        return out;
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
