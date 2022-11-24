use bytebuffer::ByteBuffer;
use instructions::Instructions;
use std::{collections::HashMap, io::{BufWriter, Write}};

mod cpu;
mod instructions;

fn main() {
    let mut memory = cpu::create_memory(256);
    // let mut writer = BufWriter::new(memory);

    let mut bytes: Vec<u8> = vec![0; 256];

    bytes[0] = Instructions::MOV_LIT_R1;
    bytes[1] = 0x12;
    bytes[2] = 0x34;

    bytes[3] = Instructions::MOV_LIT_R2;
    bytes[4] = 0xAB;
    bytes[5] = 0xCD;

    bytes[6] = Instructions::ADD_REG_REG;
    bytes[7] = 2;
    bytes[8] = 3;
    memory.write_all(&bytes).unwrap();

    let mut cpu = cpu::CPU::new(memory);

    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();
    
    cpu.step();
    cpu.debug();
}
