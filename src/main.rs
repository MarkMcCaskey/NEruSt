mod args;
//mod cpu;
//mod window;

mod cartridge;
mod cpu;
mod header;
mod nes;
mod ppu;

use crate::args::Settings;
use crate::cartridge::Cartridge;
use crate::nes::{Controller, Nes};

fn main() {
    let settings = Settings::new();

    let cart = Cartridge::load_from_file(&settings.rom_file);
    let mut nes = Nes::new(cart);

    for it in 1.. {
        nes.step();
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;

    struct Emulator {
        nes: Nes,
        cpu_cyc: usize,
        ppu_cyc: usize,
        /// The screen pixels in 8bit RGB.
        screen: Vec<u8>,
    }

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
        let buf_reader = std::io::BufReader::new(rom_bytes);
        let cartridge = Cartridge::load_from_bytes(buf_reader);
        let nes = Nes::new(cartridge);

        //nes.cpu.pc = 0xC000;
        let mut cpu_cyc = 7;
        let mut ppu_cyc = 0;

        Emulator {
            nes,
            cpu_cyc,
            ppu_cyc,
            screen: vec![0; 256 * 240 * 3],
        }
    }

    /// controller state is a bitmap of all the buttons in this order
    /// RIGHT LEFT DOWN UP START SELECT B A
    ///
    /// With the first 8 bits being player 1 and the second 8 bits being player 2's
    /// input with bytes in little endian order.
    #[no_mangle]
    extern "C" fn run_frame(emulator: &mut Emulator, controller_state: u16) {
        let p1_bits = controller_state as u8;
        let p2_bits = (controller_state >> 8) as u8;
        emulator.nes.set_controller_bits(Controller::One, p1_bits);
        emulator.nes.set_controller_bits(Controller::Two, p2_bits);

        for i in 0..10000 {
            emulator.nes.step();
        }

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
