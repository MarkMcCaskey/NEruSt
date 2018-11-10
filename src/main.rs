extern crate clap;
//extern crate sdl2;

mod args;
//mod cpu;
//mod window;

mod cartridge;
mod header;
mod memory;
mod cpu_map;
mod cpu;
mod getset;

//mod nes;

use args::Settings;
use memory::Memory;
use cpu_map::CpuMap;
use cartridge::Cartridge;
use cpu::cpu::Cpu;

fn main() {
    let settings = Settings::new();

    // The fakest blocks of memory
    let mut ram = Memory::new(0x2000);
    let mut ppu = Memory::new(0x0001); // this will be a PPU eventually
    let mut io = Memory::new(0x0001); // this will be... something... someday
    let mut cart = Cartridge::load_from_file(&settings.rom_file);

    // cpu
    let mut cpu = Cpu::new();

    // run 10 instructions
    let mut cpu_map = CpuMap {
        ram: &mut ram,
        ppu: &mut ppu,
        io: &mut io,
        cart: &mut cart,
    };
    for _ in 0..10 {
    	// run an instruction
    	cpu.run_instruction(&mut cpu_map);
    }
}
