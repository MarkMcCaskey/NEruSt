pub mod cpu;
pub mod ram;
pub mod rom;

use rom::*;
use cpu::opcodes::lda::*;
use cpu::opcodes::ldx::*;
use cpu::opcodes::ldy::*;
use cpu::opcodes::sta::*;
use cpu::opcodes::stx::*;
use cpu::opcodes::sty::*;
use cpu::opcodes::register::*;
use cpu::opcodes::asl::*;
use cpu::opcodes::and::*;
use cpu::opcodes::ora::*;
use cpu::opcodes::rol::*;
use cpu::opcodes::ror::*;
use cpu::opcodes::adc::*;

fn main() {
    let mut cpu = cpu::CPU::new();
    let mut ram = ram::RAM { data: vec![0x15, 0x25] };
    let mut rom = rom::ROM { data: vec![0xA5, 0x01] };

    let mut it: usize = 0;
    let op = get_byte(&rom, &mut it);
    let cyc = match op {
        // Register
        0x18 => clc(&mut cpu),
        0x38 => sec(&mut cpu),
        0x58 => cli(&mut cpu),
        0x78 => sei(&mut cpu),
        0xB8 => clv(&mut cpu),
        0xD8 => cld(&mut cpu),
        0xF8 => sed(&mut cpu),

        0xAA => tax(&mut cpu),
        0x8A => txa(&mut cpu),
        0xCA => dex(&mut cpu),
        0xE8 => inx(&mut cpu),

        0xA8 => tay(&mut cpu),
        0x98 => tya(&mut cpu),
        0x88 => dey(&mut cpu),
        0xC8 => iny(&mut cpu),
            
        
        // LDA
        0x49 => lda_imm(&mut cpu, get_byte(&rom, &mut it)),
        0xA5 => lda_zp(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xB5 => lda_zpx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xA1 => lda_izx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xB1 => lda_izy(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xAD => lda_abs(&mut cpu, &ram, get_word(&rom, &mut it)),
        0xBD => lda_abx(&mut cpu, &ram, get_word(&rom, &mut it)),
        0xB9 => lda_aby(&mut cpu, &ram, get_word(&rom, &mut it)),

        // LDX
        0xA2 => ldx_imm(&mut cpu, get_byte(&rom, &mut it)),
        0xA6 => ldx_zp(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xB6 => ldx_zpy(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xAE => ldx_abs(&mut cpu, &ram, get_word(&rom, &mut it)),
        0xBE => ldx_aby(&mut cpu, &ram, get_word(&rom, &mut it)),

        // LDY
        0xA0 => ldy_imm(&mut cpu, get_byte(&rom, &mut it)),
        0xA4 => ldy_zp(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xB4 => ldy_zpx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0xAC => ldy_abs(&mut cpu, &ram, get_word(&rom, &mut it)),
        0xBC => ldy_abx(&mut cpu, &ram, get_word(&rom, &mut it)),

        // STA
        0x85 => sta_zp(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x95 => sta_zpx(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x81 => sta_izx(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x91 => sta_izy(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x7D => sta_abs(&cpu, &mut ram, get_word(&rom, &mut it)),
        0x9D => sta_abx(&cpu, &mut ram, get_word(&rom, &mut it)),
        0x99 => sta_aby(&cpu, &mut ram, get_word(&rom, &mut it)),

        // STX
        0x86 => stx_zp(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x96 => stx_zpy(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x8E => stx_abs(&cpu, &mut ram, get_word(&rom, &mut it)),

        // STY
        0x84 => sty_zp(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x94 => sty_zpx(&cpu, &mut ram, get_byte(&rom, &mut it)),
        0x8C => sty_abs(&cpu, &mut ram, get_word(&rom, &mut it)),

        // ASL
        0x0A => asl_acc(&mut cpu),
        0x06 => asl_zp(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x16 => asl_zpx(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x0E => asl_abs(&mut cpu, &mut ram, get_word(&rom, &mut it)),
        0x1E => asl_abx(&mut cpu, &mut ram, get_word(&rom, &mut it)),

        // AND
        0x29 => and_imm(&mut cpu, get_byte(&rom, &mut it)),
        0x25 => and_zp(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x35 => and_zpx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x21 => and_izx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x31 => and_izy(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x2D => and_abs(&mut cpu, &ram, get_word(&rom, &mut it)),
        0x3D => and_abx(&mut cpu, &ram, get_word(&rom, &mut it)),
        0x39 => and_aby(&mut cpu, &ram, get_word(&rom, &mut it)),

        // ORA
        0x09 => ora_imm(&mut cpu, get_byte(&rom, &mut it)),
        0x05 => ora_zp(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x15 => ora_zpx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x01 => ora_izx(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x11 => ora_izy(&mut cpu, &ram, get_byte(&rom, &mut it)),
        0x0D => ora_abs(&mut cpu, &ram, get_word(&rom, &mut it)),
        0x1D => ora_abx(&mut cpu, &ram, get_word(&rom, &mut it)),
        0x19 => ora_aby(&mut cpu, &ram, get_word(&rom, &mut it)),
               
        // ROL
        0x2A => rol_acc(&mut cpu),
        0x26 => rol_zp(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x36 => rol_zpx(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x2E => rol_abs(&mut cpu, &mut ram, get_word(&rom, &mut it)),
        0x3E => rol_abx(&mut cpu, &mut ram, get_word(&rom, &mut it)),

        // ROR
        0x6A => ror_acc(&mut cpu),
        0x66 => ror_zp(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x76 => ror_zpx(&mut cpu, &mut ram, get_byte(&rom, &mut it)),
        0x6E => ror_abs(&mut cpu, &mut ram, get_word(&rom, &mut it)),
        0x7E => ror_abx(&mut cpu, &mut ram, get_word(&rom, &mut it)),

        // NOP
        // Uuuuuuuuuuhghuguhguhgg

        // KIL

	   // Shouldn't ever happen. If it does... well, yuh dun fuck'd son
        // NOTE: can use unreachable!() to tell the compiler this ^
        _ => 255,
    };
}


