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
use cpu::opcodes::sta::*;
use cpu::opcodes::register::*;

fn main() {
    let mut cpu = cpu::CPU::new();
    let mut ram = ram::RAM { data: vec![0x15, 0x25] };
    let mut rom = rom::ROM { data: vec![0xA5, 0x01] };

    let mut it: usize = 0;
    let op = get_byte(&rom, &mut it);
    match op {
        // Register
        0x18 => clc(&mut cpu),
        0x38 => sec(&mut cpu),
        0x58 => cli(&mut cpu),
        0x78 => sei(&mut cpu),
        0xB8 => clv(&mut cpu),
        0xD8 => cld(&mut cpu),
        0xF8 => sed(&mut cpu),
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
        0x85 => sta_zp(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x95 => sta_zpx(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x81 => sta_izx(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x91 => sta_izy(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x7D => sta_abs(&cpu, &mut ram, get_word(&rom, &mut it)),
        0x9D => sta_abx(&cpu, &mut ram, get_word(&rom, &mut it)),
        0x99 => sta_aby(&cpu, &mut ram, get_word(&rom, &mut it)),

        // NOP

        // KIL

	   // Shouldn't ever happen. If it does... well, yuh dun fuck'd son
        _ => println!("uh-oh"),
    }

    println!("{acc}", acc = cpu.acc);
}


