use crate::cartridge::Cartridge;
use crate::cpu::cpu::Cpu;
use crate::ppu::ppu::Ppu;

pub struct Nes {
    cart: Cartridge,

    cpu_ram: [u8; 0x800],
    ppu_ram: [u8; 0x4000],

    pub cpu: Cpu,
    pub ppu: Ppu,
}

impl Nes {
    pub fn new(cart: Cartridge) -> Self {
        Self {
            cart,
            cpu_ram: [0u8; 0x800],
            ppu_ram: [0u8; 0x4000],

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
}

impl Nes {
    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x7FF) as usize],
            0x2000..=0x3FFF => unimplemented!("self.ppu_read(addr & 0x7 | 0x2000)"),
            0x4000..=0x4017 => unimplemented!(),
            0x4018..=0x401F => unimplemented!(),
            0x4020..=0xFFFF => self.cart.cpu_view().get(addr),
        }
    }
    pub fn cpu_write(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[addr as usize] = v,
            0x2000..=0x3FFF => unimplemented!("self.ppu_write(addr, v)"),
            0x4000..=0x4017 => unimplemented!(),
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
