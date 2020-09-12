use crate::getset::GetSet;

pub struct CpuMap<'a, RAM: GetSet, PPU: GetSet, IO: GetSet> {
    pub ram: &'a mut RAM,
    pub ppu: &'a mut PPU,
    pub io: &'a mut IO,
    pub cart: &'a mut dyn GetSet,
}

impl<'a, RAM: GetSet, PPU: GetSet, IO: GetSet> GetSet for CpuMap<'a, RAM, PPU, IO> {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram.get(addr & 0x7FF),
            0x2000..=0x3FFF => self.ppu.get(addr),
            0x4000..=0x4017 => self.io.get(addr),
            0x4018..=0x401F => unimplemented!(),
            0x4020..=0xFFFF => self.cart.get(addr),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram.set(addr, val),
            0x2000..=0x3FFF => self.ppu.set(addr, val),
            0x4000..=0x4017 => self.io.set(addr, val),
            0x4018..=0x401F => unimplemented!(),
            0x4020..=0xFFFF => self.cart.set(addr, val),
            _ => unreachable!(),
        }
    }
}

/*
use std::ops::{Index, IndexMut};

impl<'a> Index<u16> for CpuMap<'a> {
    type Output = u8;

    fn index(&self, addr: u16) -> &u8 {
        match addr {
            0x0000 ..= 0x1FFF => &self.ram[addr | 0x07FF],
            0x2000 ..= 0x3FFF => &self.ppu[(addr - 0x2000) | 0x0008],
            0x4000 ..= 0x4017 => &self.io[(addr - 0x4000)],
            0x4018 ..= 0x401F => unimplemented!(),
            0x4020 ..= 0xFFFF => &self.cart[(addr - 0x4020)],
            _ => unreachable!(),
        }
    }
}

impl<'a> IndexMut<u16> for CpuMap<'a> {
    fn index_mut(&mut self, addr: u16) -> &mut u8 {
        match addr {
            0x0000 ..= 0x1FFF => &mut self.ram[addr | 0x07FF],
            0x2000 ..= 0x3FFF => &mut self.ppu[(addr - 0x0200) | 0x0008],
            0x4000 ..= 0x4017 => &mut self.io[(addr - 0x4000)],
            0x4018 ..= 0x401F => unimplemented!(),
            0x4020 ..= 0xFFFF => &mut self.cart[(addr - 0x4020)],
            _ => unreachable!(),
        }
    }
}*/
