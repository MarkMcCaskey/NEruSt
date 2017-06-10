pub mod cpu;
pub mod ram;
pub mod rom;

use rom::*;
use cpu::opcodes::lda::*;
use cpu::opcodes::sta::*;
use cpu::opcodes::register::*;
use cpu::opcodes::asl::*;
use cpu::opcodes::and::*;

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

        // ASL
        0x06 => asl_zp(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x0A => asl_acc(&mut cpu),
        0x0E => asl_abs(&mut cpu, &mut ram, get_word(&rom, &mut it)),
        0x16 => asl_zpx(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x1E => asl_abx(&mut cpu, &mut ram, get_word(&rom, &mut it)),

        // AND
        0x21 => and_izx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x25 => and_zp(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x29 => and_imm(&mut cpu, get_byte(&rom, &mut it)),
        0x2D => and_abs(&mut cpu, &ram, get_word(&rom, &mut it)),
        0x31 => and_izy(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x35 => and_zpx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x39 => and_aby(&mut cpu, &ram, get_word(&rom, &mut it)),
        0x3D => and_abx(&mut cpu, &ram, get_word(&rom, &mut it)),
        

        // NOP

        // KIL

	   // Shouldn't ever happen. If it does... well, yuh dun fuck'd son
        _ => println!("uh-oh"),
    }

    println!("{acc}", acc = cpu.acc);
}


