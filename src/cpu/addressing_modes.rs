use crate::nes::Nes;

impl Nes {
    // Imp is empty

    // Imm is empty

    pub fn zp(&mut self, addr: u8) -> u16 {
        addr as u16
    }

    pub fn zpx(&mut self, addr: u8, x: u8) -> u16 {
        addr.wrapping_add(x) as u16
    }

    pub fn zpy(&mut self, addr: u8, y: u8) -> u16 {
        addr.wrapping_add(y) as u16
    }

    pub fn izx(&mut self, addr: u8, x: u8) -> u16 {
        let lo = self.cpu_read(addr.wrapping_add(x) as u16);
        let hi = self.cpu_read(addr.wrapping_add(x).wrapping_add(1) as u16);
        lo as u16 | ((hi as u16) << 8)
    }

    pub fn izy(&mut self, addr: u8, y: u8) -> (u16, bool) {
        let lo = self.cpu_read(addr as u16);
        let hi = self.cpu_read(addr.wrapping_add(1) as u16);
        let derefed_addr = lo as u16 | ((hi as u16) << 8);
        let indexed_derefed_addr = derefed_addr + y as u16;
        (
            indexed_derefed_addr,
            indexed_derefed_addr & 0xFF00 != derefed_addr & 0xFF00,
        )
    }

    pub fn abs(&mut self, addr: u16) -> u16 {
        addr
    }

    pub fn abx(&mut self, addr: u16, x: u8) -> (u16, bool) {
        let indexed_addr = addr + x as u16;
        (indexed_addr, indexed_addr & 0xFF00 != addr & 0xFF00)
    }

    pub fn aby(&mut self, addr: u16, y: u8) -> (u16, bool) {
        let indexed_addr = addr + y as u16;
        (indexed_addr, indexed_addr & 0xFF00 != addr & 0xFF00)
    }

    pub fn ind(&mut self, addr: u16) -> u16 {
        let lo = self.cpu_read(addr);
        let hi = self.cpu_read(addr & 0xFF00 | (addr as u8).wrapping_add(1) as u16);
        lo as u16 | ((hi as u16) << 8)
    }
}
