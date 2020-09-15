mod args;
//mod cpu;
//mod window;

mod cartridge;
mod cpu;
mod cpu_map;
mod getset;
mod header;
mod io;
mod memory;
mod ppu;
mod ppu_map;

//mod nes;

use crate::args::Settings;
use crate::cartridge::Cartridge;
use crate::cpu::cpu::Cpu;
use crate::cpu_map::CpuMap;
use crate::getset::GetSet;
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::ppu_map::PpuMap;

fn main() {
    let settings = Settings::new();

    let mut cpu_ram = Memory::new(0x800);
    let mut ppu_memory = Memory::new(0x4000);
    let mut io_memory = Memory::new(0x0020); // this will be... something... someday
    let mut cart = Cartridge::load_from_file(&settings.rom_file);

    // cpu
    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();

    /*use crate::io::{Input, Output};
    let mut io = io::native::NativeRenderer::new(400, 600).unwrap();*/

    cpu.pc = 0xC000;
    let mut cpu_cyc = 7;
    let mut ppu_cyc = 0;
    for it in 1..=3000 {
        {
            let mut cpu_map = CpuMap {
                ram: &mut cpu_ram,
                ppu: &mut ppu,
                io: &mut io_memory,
                cart: &mut cart.cpu_view(),
            };
            let inst = cpu_map.get(cpu.pc);
            println!(
            "{:05}: {:04X} {:2X}    A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:3},{:3} CYC:{}",
            it, cpu.pc, inst, cpu.acc, cpu.x, cpu.y, cpu.p, cpu.s, 0, 0, cpu_cyc,
            );
            //println!("*($0180) == {:02X}", cpu_map.ram.get(0x0180));
            // run an instruction
            cpu_cyc += cpu.run_instruction(&mut cpu_map) as usize;
        }
        let mut ppu_map = PpuMap {
            // should ppu_memory go in here?
            cart: &mut cart.ppu_view(),
        };
        ppu_cyc += ppu.run_instruction(&mut ppu_memory, &mut ppu_map) as usize;

        /*while let Some(ie) = io.get_next_input() {
            dbg!(ie);
        }*/
    }
}
