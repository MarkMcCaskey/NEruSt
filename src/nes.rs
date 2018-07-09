use memory::*;

pub struct Nes {
    ram: Memory,
    /// 0x4016 and 0x4017
    controllers_state: [u8; 2],
}

#[repr(u8)]
#[derive(Debug)]
pub enum Button {
    Right = 0x01,
    Left = 0x02,
    Down = 0x04,
    Up = 0x08,
    Start = 0x10,
    Select = 0x20,
    B = 0x40,
    A = 0x80,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            ram: Memory::new(0x0800), // 2kb of system ram
            controllers_state: [0; 2],
        }
    }

    pub fn set_button(&mut self, button: Button, player_2: bool, state: bool) {
        let player_select = player_2 as usize;
        let button_val = button as u8;
        if state {
            self.controllers_state[player_select] |= button_val;
        } else {
            self.controllers_state[player_select] &= !button_val;
        }
    }
}
