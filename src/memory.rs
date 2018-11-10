use getset::GetSet;

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
        Self { memory }
    }
}

impl GetSet for Memory {
    fn get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn set(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }
}