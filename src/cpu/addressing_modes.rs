use crate::getset::GetSet;

// Imp is empty

// Imm is empty

pub fn zp(addr: u8) -> u16 {
    addr as u16
}

pub fn zpx(addr: u8, x: u8) -> u16 {
    addr.wrapping_add(x) as u16
}

pub fn zpy(addr: u8, y: u8) -> u16 {
    addr.wrapping_add(y) as u16
}

pub fn izx(addr: u8, x: u8, cpu_map: &dyn GetSet) -> u16 {
    cpu_map.get_16(addr.wrapping_add(x) as u16)
}

pub fn izy(addr: u8, y: u8, cpu_map: &dyn GetSet) -> (u16, bool) {
    let derefed_addr = cpu_map.get_16(addr as u16);
    let indexed_derefed_addr = derefed_addr + y as u16;
    (
        indexed_derefed_addr,
        indexed_derefed_addr > derefed_addr | 0x0FF,
    )
}

pub fn abs(addr: u16) -> u16 {
    addr
}

pub fn abx(addr: u16, x: u8) -> (u16, bool) {
    let indexed_addr = addr + x as u16;
    (indexed_addr, indexed_addr > addr | 0xFF)
}

pub fn aby(addr: u16, y: u8) -> (u16, bool) {
    let indexed_addr = addr + y as u16;
    (indexed_addr, indexed_addr > addr | 0x0FF)
}

pub fn ind(addr: u16, cpu_map: &dyn GetSet) -> u16 {
    cpu_map.get_16(addr)
}
