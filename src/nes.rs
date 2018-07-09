use memory::*;

pub struct Nes {
	ram: Memory,
}

impl Nes {
	pub fn new() -> Self {
		Self {
			ram: Memory::new(0x0800), // 2kb of system ram
		}
	}
}