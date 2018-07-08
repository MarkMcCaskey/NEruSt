use clap::{App, Arg};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Settings {
    pub rom_file: PathBuf,
}

impl Settings {
    pub fn new() -> Self {
        let matches = App::new("NEruSt")
            .version("0.0.1")
            .about("An emulator for the Famicom and NES")
            .arg(
                Arg::with_name("rom-file")
                    .short("f")
                    .value_name("FILE")
                    .help("The path to a ROM file")
                    .takes_value(true),
            )
            .get_matches();
        let rom_file_arg = matches
            .value_of("rom-file")
            .expect("Must pass in ROM file to use");

        Self {
            rom_file: PathBuf::from(rom_file_arg),
        }
    }
}
