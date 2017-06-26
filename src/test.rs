pub struct RAM {
    pub data: Vec<u8>,
}
pub struct ROM {
    pub data: Vec<u8>,
}
pub struct CPU {
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    /// process status register
    pub p: u8,
}

// addressing modes
pub fn imp() -> OpcodeOperand {
    OpcodeOperand::Implied
}

pub fn imm(ram: &RAM, val: u8) -> OpcodeOperand {
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

// opcode logic
pub fn lda(cpu: &mut CPU, ram: &RAM, opop: OpcodeOperand) {
    // addressing
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => cpu.acc = val,
        OpcodeOperand::Address(adr) => cpu.acc = ram.data[adr as usize],
    }

    // flags
    let zero_flag: bool = cpu.acc == 0;
    let negative_flag: bool = cpu.acc & 0x80 == 0x80;
    // set them
}

pub fn sta(cpu: &CPU, ram: &mut RAM, opop: OpcodeOperand) {
    match opop {
        OpcodeOperand::Implied => unreachable!(),
        OpcodeOperand::Immediate(val) => unreachable!(),
        OpcodeOperand::Address(adr) => ram.data[adr as usize] = cpu.acc,
    }

    // no flags
}

// rom stuff
pub fn get_byte(rom: &ROM, adr: u16) -> u8 { 0 }
pub fn get_word(rom: &ROM, adr: u16) -> u16 { 0 }

pub enum OpcodeOperand {
    Implied,
    Immediate(u8),
    Address(u16),
}

fn main() {
    let mut cpu = CPU { acc: 0, x: 0, y: 0, p: 0 };
    let mut ram = RAM { data: vec![0x15, 0x25] };
    let mut rom = ROM { data: vec![0xA5, 0x01] };
    
    let mut it: u16 = 0;
    let opcode = get_byte(&rom, it);
    it += 1;
    let out = match opcode {
        0xA1 => {
            let data = get_byte(&rom, it);
            it += 1;
            let opop = izx(&ram, data, cpu.x);
            lda(&mut cpu, &ram, opop);
            6
        },
        0xB1 => {
            let data = get_byte(&rom, it);
            it += 1;
            let (opop, add_cycle) = izy(&ram, data, cpu.y);
            lda(&mut cpu, &ram, opop);
            5 + add_cycle as u8
        }
        _ => 3,
    };
}