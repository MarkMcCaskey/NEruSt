/* The iNES header is formatted like so:
NOTE: These fields will be filled as the header type is implemented
    bytes 0-3: constant representing 'nes' + EOF byte
    byte 4: size of PRG rom in 16384 ($4000) byte units
    byte 5: size of CHR rom in 8192 ($2000) byte units
    byte 6: -----t--
        t: contains trainer
    byte 7:
    byte 8: size of PRG ram in 8192 byte units
    byte 9:
    byte 10:
    byte 11:
    byte 12:
    byte 13: 
    byte 14:
    byte 15:    
*/
pub struct INESHeader {
    data: [u8; 16],
}

impl INESHeader {
    pub fn from(data: [u8; 16]) -> Self {
        Self { data }
    }

    pub fn get_prg_rom_size(&self) -> usize {
        self.data[4] as usize * 16384
    }

    pub fn get_chr_rom_size(&self) -> usize {
        self.data[5] as usize * 8192
    }

    pub fn get_prg_ram_size(&self) -> usize {
        self.data[8] as usize * 1892
    }

    pub fn contains_trainer(&self) -> bool {
        // if the trainer bit isn't 0
        (self.data[6] & 0b0000_0100) != 0
    }

    pub fn get_mapper_id(&self) -> u8 {
        // the mapper is retrieved by combining the first 4 bits of
        // flag 7 with the first 4 bits of flag 6.
        (self.data[7] % 0b1111_0000) + (self.data[6] >> 4)
    }
}
