extern crate clap;

pub mod args;
pub mod cpu;
pub mod ram;
pub mod rom;

use args::Settings;
use cpu::addressing_modes::*;
use cpu::cpu::*;
use cpu::opcode_logic::*;
use ram::ram::*;
use rom::rom::*;

fn main() {
    let settings = Settings::new();

    let mut cpu = Cpu::new();
    let mut ram = Ram {
        data: vec![0u8; 0xFFFF],
    };

    let rom = Rom::from_file(settings.rom_file).expect("Could not read ROM from file");
    while !cpu.halt {
        cpu.run_instruction(&mut ram, &rom);
    }
}
