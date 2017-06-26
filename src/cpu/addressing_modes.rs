use ram::ram::RAM;

pub enum OpcodeOperand {
    Implied,
    Immediate(u8),
    Address(u16),
}

pub fn imp() -> OpcodeOperand {
    OpcodeOperand::Implied
}

pub fn imm(val: u8) -> OpcodeOperand {
    OpcodeOperand::Immediate(val)
}

pub fn zp(adr: u8) -> OpcodeOperand {
    OpcodeOperand::Address(adr as u16)
}

pub fn zpx(adr: u8, x: u8) -> OpcodeOperand {
    let indexed_adr: u8 = adr.wrapping_add(x);
    OpcodeOperand::Address(indexed_adr as u16)
}

pub fn zpy(adr: u8, y: u8) -> OpcodeOperand {
    let indexed_adr: u8 = adr.wrapping_add(y);
    OpcodeOperand::Address(indexed_adr as u16)
}

pub fn izx(ram: &RAM, adr: u8, x: u8) -> OpcodeOperand {
    let indexed_adr: u8 = adr.wrapping_add(x);
    let derefed_indexed_adr: u16 = (ram.data[indexed_adr as usize] as u16) << 8 | ram.data[indexed_adr.wrapping_add(1) as usize] as u16;
    OpcodeOperand::Address(derefed_indexed_adr as u16)
}

pub fn izy(ram: &RAM, adr: u8, y: u8) -> (OpcodeOperand, bool) {
    let derefed_adr: u16 = (ram.data[adr as usize] as u16) << 8 | ram.data[adr.wrapping_add(1) as usize] as u16;
    let indexed_derefed_adr: u16 = derefed_adr + y as u16;
    (OpcodeOperand::Address(indexed_derefed_adr), indexed_derefed_adr > derefed_adr | 0x0FF)
}

pub fn abs(adr: u16) -> OpcodeOperand {
    OpcodeOperand::Address(adr)
}

pub fn abx(adr: u16, x: u8) -> (OpcodeOperand, bool) {
    let indexed_adr: u16 = adr + x as u16;
    (OpcodeOperand::Address(indexed_adr), indexed_adr > adr | 0xFF)
}

pub fn aby(adr: u16, y: u8) -> (OpcodeOperand, bool) {
    let indexed_adr: u16 = adr + y as u16;
    (OpcodeOperand::Address(indexed_adr), indexed_adr > adr | 0x0FF)
}

pub fn ind(ram: &RAM, adr: u16) -> OpcodeOperand {
    let derefed_adr: u16 = (ram.data[adr as usize] as u16) << 8 | ram.data[adr.wrapping_add(1) as usize] as u16;
    OpcodeOperand::Address(derefed_adr)
}