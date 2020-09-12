use crate::getset::GetSet;

pub struct PpuMap<'a> {
    pub cart: &'a mut dyn GetSet,
}

impl<'a> GetSet for PpuMap<'a> {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cart.get(addr),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u8) {
        unreachable!("PPU should probably not be writing to the Cart (not verified in docs, maybe it's supposed to)");
    }
}
