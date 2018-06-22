pub mod cpu;
pub mod ram;
pub mod rom;

use cpu::addressing_modes::*;
use cpu::cpu::*;
use cpu::opcode_logic::*;
use ram::ram::*;
use rom::rom::*;

fn main() {
    let mut cpu = CPU::new();
    let mut ram = RAM {
        data: vec![0x15, 0x25],
    };
    let rom = ROM {
        data: vec![0xA5, 0x01],
    };

    let mut it: usize = 0;
    let op = get_byte(&rom, &mut it);
    let cyc = match op {
        // LDA
        0x49 => {
            let data = get_byte(&rom, &mut it);
            let opop = imm(data);
            lda(&mut cpu, &ram, opop);
            2
        }

        0xA5 => {
            let data = get_byte(&rom, &mut it);
            let opop = zp(data);
            lda(&mut cpu, &ram, opop);
            3
        }

        0xB5 => {
            let data = get_byte(&rom, &mut it);
            let opop = zpx(data, cpu.x);
            lda(&mut cpu, &ram, opop);
            4
        }

        0xA1 => {
            let data = get_byte(&rom, &mut it);
            let opop = izx(&ram, data, cpu.x);
            lda(&mut cpu, &ram, opop);
            6
        }

        0xB1 => {
            let data = get_byte(&rom, &mut it);
            let (opop, add_cycle) = izy(&ram, data, cpu.y);
            lda(&mut cpu, &ram, opop);
            5 + add_cycle as u8
        }

        0xAD => {
            let data = get_word(&rom, &mut it);
            let opop = abs(data);
            lda(&mut cpu, &ram, opop);
            4
        }

        0xBD => {
            let data = get_word(&rom, &mut it);
            let (opop, add_cycle) = abx(data, cpu.x);
            lda(&mut cpu, &ram, opop);
            4 + add_cycle as u8
        }

        0xB9 => {
            let data = get_word(&rom, &mut it);
            let (opop, add_cycle) = aby(data, cpu.y);
            lda(&mut cpu, &ram, opop);
            4 + add_cycle as u8
        }

        // STA
        0x81 => {
            let data = get_byte(&rom, &mut it);
            let opop = izx(&ram, data, cpu.x);
            sta(&cpu, &mut ram, opop);
            6
        }

        // Shouldn't ever happen. If it does... well, yuh dun fuck'd son
        // NOTE: can use unreachable!() to tell the compiler this ^
        _ => {
            unreachable!();
            255
        }
    };
}
