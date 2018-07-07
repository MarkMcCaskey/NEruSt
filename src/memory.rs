use std::ops::Index;
use std::ops::IndexMut;

pub struct Memory {
	memory: Box<[u8]>,
}

impl Memory {
	pub fn new(size: usize) -> Self {
		Self {
			memory: vec![0u8; size].into_boxed_slice(),
		}
	}

	pub fn from_boxed_slice(memory: Box<[u8]>) -> Self {
		Self {
			memory
		}
	}
}

impl Index<usize> for Memory {
	type Output = u8;

	fn index(&self, index: usize) -> &u8 {
		&self.memory[index]
	}
}

impl IndexMut<usize> for Memory {
	fn index_mut(&mut self, index: usize) -> &mut u8 {
		&mut self.memory[index]
	}
}

pub fn combine_bytes(high: u8, low: u8) -> u16 {
	(high as u16) << 8 + low
}