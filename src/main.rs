pub mod cpu;
pub mod ram;
pub mod rom;

/* Ill find a better place to put this. When implementing the "match switch", try
to keep your function calls in order. Look to the LDA parts for guidance.
imm = #$00
zp = $00
zpx = $00,X
zpy = $00,Y
izx = ($00,X)
izy = ($00),Y
abs = $0000
abx = $0000,X
aby = $0000,Y
ind = ($0000)
*/

use rom::*;
use cpu::opcodes::lda::*;
//use cpu::opcodes::lda::*;

fn main() {
    let mut cpu = cpu::CPU::new();
    let mut ram = ram::RAM { data: vec![0x15, 0x25] };
    let mut rom = rom::ROM { data: vec![0xA5, 0x01] };

    let mut it: usize = 0;
    let op = get_byte(&rom, &mut it);
    match op {
        // LDA
        0x49 => lda_imm(&mut cpu, get_byte(&rom, &mut it)),
        0xA5 => lda_zp(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xAD => lda_abs(&mut cpu, &ram, get_word(&rom, &mut it)),
        0xB5 => lda_zpx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xA1 => lda_izx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xB1 => lda_izy(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xBD => lda_abx(&mut cpu, &ram, get_word(&rom, &mut it)),
        0xB9 => lda_aby(&mut cpu, &ram, get_word(&rom, &mut it)),	

        // STA


	// Shouldn't ever happen. If it does... well, yuh dun fuck'd son
        _ => println!("uh-oh"),
    }

    println!("{acc}", acc = cpu.acc);
}
