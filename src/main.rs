extern crate clap;
extern crate ezgl;
extern crate gl;
//extern crate sdl2;

mod args;
//mod cpu;
//mod window;

mod canvas;
mod cartridge;
mod cpu;
mod cpu_map;
mod emscripten;
mod getset;
mod header;
mod memory;
mod system;

//mod nes;

use args::Settings;
use cartridge::Cartridge;
use cpu::cpu::Cpu;
use cpu_map::CpuMap;
use memory::Memory;
use system::System;

extern "C" fn main_loop(sys: *mut std::os::raw::c_void) {
    unsafe {
        let mut sys = &mut *(sys as *mut System);
        sys.step();
    }
}

fn main() {
    unsafe {
        let mut attributes: emscripten::EmscriptenWebGLContextAttributes =
            std::mem::uninitialized();
        emscripten::emscripten_webgl_init_context_attributes(&mut attributes);
        attributes.majorVersion = 2;
        let handle = emscripten::emscripten_webgl_create_context(std::ptr::null(), &attributes);
        emscripten::emscripten_webgl_make_context_current(handle);
        gl::load_with(|name| {
            let name_ffi = std::ffi::CString::new(name).unwrap();
            emscripten::emscripten_GetProcAddress(name.as_ptr() as *const _) as *const _
        });
        let mut ctx = System::new();
        let ptr = &mut ctx as *mut _ as *mut std::os::raw::c_void;
        emscripten::emscripten_set_main_loop_arg(Some(main_loop), ptr, 0, 1);
    }
}

#[cfg(feature = "native")]
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
