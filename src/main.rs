#[macro_use]
extern crate log;

mod args;
//mod cpu;
//mod window;

mod cartridge;
mod cpu;
mod header;
mod logging;
mod nes;
mod ppu;
#[cfg(target_arch = "wasm32")]
mod wasm;

use crate::args::Settings;
use crate::cartridge::Cartridge;
use crate::nes::{Controller, Nes};

fn main() {
    logging::attach_logger(::log::LevelFilter::Trace);
    let settings = Settings::new();

    let cart = Cartridge::load_from_file(&settings.rom_file);
    let mut nes = Nes::new(cart);

    for it in 0.. {
        nes.step();
    }
}

#[cfg(target_arch = "wasm32")]
use wasm::*;
