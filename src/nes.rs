use crate::cartridge::Cartridge;
use crate::cpu::cpu::Cpu;
use crate::ppu::ppu::Ppu;

pub struct Nes {
    cart: Cartridge,

    cpu_ram: [u8; 0x800],
    ppu_ram: [u8; 0x4000],

    controller_device: ControllerDevice,

    pub cpu: Cpu,
    pub ppu: Ppu,
}

impl Nes {
    pub fn new(cart: Cartridge) -> Self {
        Self {
            cart,
            cpu_ram: [0u8; 0x800],
            ppu_ram: [0u8; 0x4000],

            controller_device: ControllerDevice::default(),

            cpu: Cpu::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn step(&mut self) {
        // run cpu
        let cpu_cyc = self.step_cpu() as usize;

        // run ppu
        let vblank = self.step_ppu(3 * cpu_cyc as u8);
        if vblank {
            self.cpu.nmi();
        }
    }

    pub fn set_controller_bits(&mut self, controller: Controller, bits: u8) {
        match controller {
            Controller::One => {
                self.controller_device.controller1 = bits;
            }
            Controller::Two => {
                self.controller_device.controller2 = bits;
            }
        }
    }
}

impl Nes {
    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x7FF) as usize],
            0x2000..=0x3FFF => self.ppu_read_reg(addr),
            0x4000..=0x4015 => unimplemented!(),
            0x4016 => self.controller_device.read_p1_next_bit(),
            0x4017 => self.controller_device.read_p2_next_bit(),
            0x4018..=0x401F => unimplemented!(),
            0x4020..=0xFFFF => self.cart.cpu_view().get(addr),
        }
    }
    pub fn cpu_write(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[addr as usize] = v,
            0x2000..=0x3FFF => self.ppu_write(addr, v),
            0x4000..=0x4015 => unimplemented!(),
            0x4016 => {
                if v & 1 == 1 {
                    self.controller_device.set_loading_bits();
                } else {
                    self.controller_device.load_bits();
                }
            }
            0x4017 => unimplemented!(),
            0x4018..=0x401F => unimplemented!(),
            0x4020..=0xFFFF => self.cart.cpu_view().set(addr, v),
        }
    }
}

impl Nes {
    pub fn ppu_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cart.ppu_view().get(addr),
            0x2000..=0x2FFF => self.ppu_ram[addr as usize],
            0x3000..=0x3EFF => self.ppu_ram[addr as usize - 0x1000], // mirror
            0x3F00..=0x3FFF => todo!("Palette RAM index get"),
            _ => unreachable!(),
        }
    }
    pub fn ppu_write(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x1FFF => unreachable!("Maybe?"),
            0x2000..=0x2FFF => self.ppu_ram[addr as usize] = v,
            0x3000..=0x3Eff => self.ppu_ram[addr as usize - 0x1000] = v, // mirror
            0x3F00..=0x3FFF => todo!("Palette RAM index set"),
            _ => unreachable!(),
        }
    }
}

impl Nes {
    pub fn draw_screen(&mut self, screen: &mut [u8]) {
        let color1 = [255, 0, 0];
        let color2 = [0, 255, 0];
        let color3 = [0, 0, 255];
        let color4 = [255, 255, 255];
        let colors = [color1, color2, color3, color4];

        let mut s_idx = 0;
        // draw 32 tiles per line, do 1 line for now
        for i in 0..256 {
            // 8 bytes per tile
            for j in 0..8 {
                // 8 bits per byte
                for k in 0..8 {
                    let color_bit1 = (self.ppu_ram[j] >> k) & 1;
                    let color_bit2 = (self.ppu_ram[j + 8] >> k) & 1;
                    let color_idx = color_bit1 | (color_bit2 << 1);
                    for byte in &colors[color_idx as usize] {
                        screen[s_idx] = *byte;
                        s_idx += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct ControllerDevice {
    /// The bits for controller 1.
    /// Order is RIGHT LEFT DOWN UP START SELECT B A
    /// Mapped to 0x4016
    pub controller1: u8,
    /// The bits for controller 2.
    /// Mapped to 0x4017
    pub controller2: u8,

    c1_idx: u8,
    c2_idx: u8,

    loading_bits: bool,
}

impl ControllerDevice {
    fn read_p1_next_bit(&mut self) -> u8 {
        // official NES controllers return 1 after all buttons have been read
        let cur_bit = if self.c1_idx > 7 {
            1
        } else {
            (self.controller1 >> (7 - self.c1_idx)) & 1
        };
        self.c1_idx += 1;
        // D1-D4 are not useful for us right now.
        // we use 0x40 because the top 3 pins are not connected
        0x40 | cur_bit
    }

    fn read_p2_next_bit(&mut self) -> u8 {
        // official NES controllers return 1 after all buttons have been read
        let cur_bit = if self.c2_idx > 7 {
            1
        } else {
            (self.controller2 >> (7 - self.c2_idx)) & 1
        };
        self.c2_idx += 1;
        // D1-D4 are not useful for us right now.
        // we use 0x40 because the top 3 pins are not connected
        0x40 | cur_bit
    }

    fn set_loading_bits(&mut self) {
        self.loading_bits = true;
    }
    fn load_bits(&mut self) {
        if self.loading_bits {
            self.c1_idx = 0;
            self.c2_idx = 0;
            self.loading_bits = false;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Controller {
    One,
    Two,
}
