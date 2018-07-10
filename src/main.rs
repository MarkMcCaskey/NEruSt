extern crate clap;

pub mod args;
pub mod cpu;
pub mod ram;
pub mod rom;

pub mod cartridge;
pub mod header;
pub mod memory;
pub mod nes;

pub mod mapper000board;

use args::Settings;
use cpu::cpu::*;
use ram::ram::*;
use rom::rom::*;

fn main() {
    let settings = Settings::new();

    let mut cpu = Cpu::new();
    let mut ram = Ram {
        data: vec![0u8; 0x10000],
    };

    let rom = Rom::from_file(settings.rom_file).expect("Could not read ROM from file");
    while !cpu.halt {
        cpu.run_instruction(&mut ram, &rom);
    }
}
