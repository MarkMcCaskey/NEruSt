mod args;
//mod cpu;
//mod window;

mod cartridge;
mod cpu;
mod cpu_map;
mod getset;
mod header;
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
    let mut io = Memory::new(0x0020); // this will be... something... someday
    let mut cart = Cartridge::load_from_file(&settings.rom_file);

    // cpu
    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();

    cpu.pc = 0xC000;
    let mut cpu_cyc = 7;
    let mut ppu_cyc = 0;
    for it in 1.. {
        {
            let mut cpu_map = CpuMap {
                ram: &mut cpu_ram,
                ppu: &mut ppu,
                io: &mut io,
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
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;

    struct Emulator {
        cpu_ram: Memory,
        ppu_memory: Memory,
        io: Memory,
        cartridge: Cartridge,
        cpu: Cpu,
        ppu: Ppu,
        cpu_cyc: usize,
        ppu_cyc: usize,
        /// The screen pixels in 8bit RGB.
        screen: Vec<u8>,
    }

    //#[cfg(target = "wasm32-unknown-unknown")]
    //mod wasm {
    extern "C" {
        fn draw_screen(ptr: usize);
    }

    #[no_mangle]
    unsafe extern "C" fn create_emulator(rom_bytes: *mut u8, num_bytes: usize) -> Box<Emulator> {
        let rom_bytes: &[u8] = std::slice::from_raw_parts(rom_bytes, num_bytes);
        let emu = create_emulator_inner(rom_bytes);

        Box::new(emu)
    }

    fn create_emulator_inner(rom_bytes: &[u8]) -> Emulator {
        let cpu_ram = Memory::new(0x800);
        let ppu_memory = Memory::new(0x4000);
        let io = Memory::new(0x0020); // this will be... something... someday
        let buf_reader = std::io::BufReader::new(rom_bytes);
        let cartridge = Cartridge::load_from_bytes(buf_reader);

        // cpu
        let mut cpu = Cpu::new();
        let ppu = Ppu::new();

        cpu.pc = 0xC000;
        let mut cpu_cyc = 7;
        let mut ppu_cyc = 0;

        Emulator {
            cpu_ram,
            ppu_memory,
            io,
            cartridge,
            cpu,
            ppu,
            cpu_cyc,
            ppu_cyc,
            screen: vec![0; 256 * 240 * 3],
        }
    }

    /// controller state is a bitmap of all the buttons in this order
    /// LEFT, UP, RIGHT, DOWN, A, B, START, SELECT
    #[no_mangle]
    extern "C" fn run_frame(emulator: &mut Emulator, _controller_state: u8) {
        // TODO: convert and pass controller state
        /*for it in 1..2 {
            {
                let mut cpu_map = CpuMap {
                    ram: &mut emulator.cpu_ram,
                    ppu: &mut emulator.ppu,
                    io: &mut emulator.io,
                    cart: &mut emulator.cartridge.cpu_view(),
                };
                let inst = cpu_map.get(emulator.cpu.pc);
                emulator.cpu_cyc += emulator.cpu.run_instruction(&mut cpu_map) as usize;
            }
            let mut ppu_map = PpuMap {
                cart: &mut emulator.cartridge.ppu_view(),
            };
            emulator.ppu_cyc += emulator
                .ppu
                .run_instruction(&mut emulator.ppu_memory, &mut ppu_map)
                as usize;
        }*/

        for y in 0..240 {
            for x in 0..256 {
                let idx = ((y * 256) + x) * 3;
                //rustfmt: ignore
                emulator.screen[idx + 0] = if x < 100 { 255 } else { 0 };
                emulator.screen[idx + 1] = if x >= 100 && x < 200 { 255 } else { 0 };
                emulator.screen[idx + 2] = if x >= 200 && x < 300 { 255 } else { 0 };
            }
        }

        unsafe {
            draw_screen(emulator.screen.as_mut_ptr() as usize);
        }
    }

    #[no_mangle]
    extern "C" fn allocate_bytes(num_bytes: usize) -> *mut u8 {
        let mut bytes = vec![0; num_bytes];
        let mut byte_slice: Box<[u8]> = bytes.into_boxed_slice();
        byte_slice.as_mut_ptr()
    }

    #[no_mangle]
    unsafe extern "C" fn free_bytes(ptr: *mut u8, num_bytes: usize) {
        let _bytes: Vec<u8> = Vec::from_raw_parts(ptr, num_bytes, num_bytes);
    }
}
