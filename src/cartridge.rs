pub trait Cartridge {
    fn get(&self, u16) -> u8;
    fn set(&mut self, u16, u8);
}

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use header::*;
use mapper000board::*;

fn load_cartridge(rom_name: String) -> Box<Cartridge> {
    let file = File::open(rom_name).unwrap();
    let mut buf_reader = BufReader::new(file);

    // extract the header
    let mut header_bytes = [0u8; 16];
    buf_reader.read(&mut header_bytes).unwrap();
    let header = INESHeader::from(header_bytes);

    // create the board
    let mapper_board = match header.get_mapper_id() {
        0 => Mapper000Board::new(header, buf_reader),
        _ => panic!("This rom is not supported!"),
    };

    // return the cart
    Box::new(mapper_board)
}