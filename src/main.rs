pub mod cpu;
pub mod ram;
pub mod rom;

use cpu::addressing_modes::*;
use cpu::cpu::*;
use cpu::opcode_logic::*;
use ram::ram::*;
use rom::rom::*;

fn main() {
    let mut cpu = Cpu::new();
    let mut ram = Ram {
        data: vec![0x15, 0x25],
    };
    let rom = Rom {
        data: vec![0xA5, 0x01],
    };
    cpu.run_instruction(&mut ram, &rom);
}
