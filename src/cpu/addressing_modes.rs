use crate::getset::GetSet;

pub enum OpcodeOperand {
    Implied,
    Immediate(u8),
    Address(u16),
}

#[inline(always)]
pub fn imp() -> OpcodeOperand {
    OpcodeOperand::Implied
}

#[inline(always)]
pub fn imm(val: u8) -> OpcodeOperand {
    OpcodeOperand::Immediate(val)
}

#[inline(always)]
pub fn zp(adr: u8) -> OpcodeOperand {
    OpcodeOperand::Address(adr.into())
}

pub fn zpx(adr: u8, x: u8) -> OpcodeOperand {
    let indexed_adr: u8 = adr.wrapping_add(x);
    OpcodeOperand::Address(indexed_adr.into())
}

pub fn zpy(adr: u8, y: u8) -> OpcodeOperand {
    let indexed_adr: u8 = adr.wrapping_add(y);
    OpcodeOperand::Address(indexed_adr.into())
}

pub fn izx(cpu_map: &dyn GetSet, adr: u8, x: u8) -> OpcodeOperand {
    let indexed_adr: u8 = adr.wrapping_add(x);
    let derefed_indexed_adr: u16 = (cpu_map.get(indexed_adr.into()) as u16) << 8
        | cpu_map.get(indexed_adr.wrapping_add(1).into()) as u16;
    OpcodeOperand::Address(derefed_indexed_adr)
}

pub fn izy(cpu_map: &dyn GetSet, adr: u8, y: u8) -> (OpcodeOperand, bool) {
    let derefed_adr: u16 =
        (cpu_map.get(adr.into()) as u16) << 8 | cpu_map.get(adr.wrapping_add(1).into()) as u16;
    let indexed_derefed_adr: u16 = derefed_adr + y as u16;
    (
        OpcodeOperand::Address(indexed_derefed_adr),
        indexed_derefed_adr > derefed_adr | 0x0FF,
    )
}

pub fn abs(adr: u16) -> OpcodeOperand {
    OpcodeOperand::Address(adr)
}

pub fn abx(adr: u16, x: u8) -> (OpcodeOperand, bool) {
    let indexed_adr: u16 = adr + x as u16;
    (
        OpcodeOperand::Address(indexed_adr),
        indexed_adr > adr | 0xFF,
    )
}

pub fn aby(adr: u16, y: u8) -> (OpcodeOperand, bool) {
    let indexed_adr: u16 = adr + y as u16;
    (
        OpcodeOperand::Address(indexed_adr),
        indexed_adr > adr | 0x0FF,
    )
}

pub fn ind(cpu_map: &dyn GetSet, adr: u16) -> OpcodeOperand {
    let derefed_adr: u16 = (cpu_map.get(adr) as u16) << 8 | cpu_map.get(adr.wrapping_add(1)) as u16;
    OpcodeOperand::Address(derefed_adr)
}
