use std::{collections::HashMap};

use bytebuffer::ByteBuffer;

use crate::instructions::Instructions;

pub fn create_memory(size_in_bytes: usize) -> ByteBuffer {
    let mut bytes = Vec::with_capacity(size_in_bytes);
    bytes.fill(0);
    let buf = ByteBuffer::from_vec(bytes);
    return buf;
}

#[derive(Debug)]
pub enum CPUError {
    RegisterNotFound(&'static str),
    ReadRegisterError,
    UnknownInstruction(u8),
}

pub struct CPU {
    memory: ByteBuffer,
    register_names: Vec<&'static str>,
    registers: ByteBuffer,
    /// register name to byte offset in memory
    register_map: HashMap<String, usize>,
}

impl CPU {
    pub fn new(memory: ByteBuffer) -> Self {
        let register_names = vec!["ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8"];
        // each register takes 2 bytes (16 bits)
        let registers = create_memory(register_names.len() * 2);

        let mut register_map = HashMap::new();
        for (idx, name) in register_names.iter().enumerate() {
            register_map.insert(String::from(*name), idx * 2);
        }

        CPU {
            memory,
            register_names,
            registers,
            register_map,
        }
    }

    pub fn get_register(&mut self, name: &'static str) -> Option<u16> {
        if !self.register_map.contains_key(name) {
            return None;
        }

        let Some(offset) =self.register_map.get(&String::from(name)) else {
            return None
        };

        self.registers.set_rpos(*offset);
        let Ok(reg_val) = self.registers.read_u16() else {
            return None
        };

        return Some(reg_val);
    }

    pub fn set_register(&mut self, name: &'static str, value: u16) -> Result<(), CPUError> {
        if !self.register_map.contains_key(name) {
            return Err(CPUError::RegisterNotFound(name));
        }

        let Some(offset) =self.register_map.get(&String::from(name)) else {
            return Err(CPUError::RegisterNotFound(name));
        };

        self.registers.set_wpos(*offset);
        self.registers.write_u16(value);

        return Ok(());
    }

    pub fn fetch(&mut self) -> Result<u8, CPUError> {
        let Some(next_instruction_address) = self.get_register("ip") else {
            return Err(CPUError::ReadRegisterError);
        };

        self.memory.set_rpos(next_instruction_address.into());
        let Ok(instruction) = self.memory.read_u8() else {
            return Err(CPUError::ReadRegisterError);
        };

        // move the IP by one byte
        let result = self.set_register("ip", next_instruction_address + 1);
        match result {
            Err(err) => return Err(err),
            Ok(_) => (),
        }

        return Ok(instruction);
    }

    fn fetch_16(&mut self) -> Result<u16, CPUError> {
        let Some(next_instruction_address) = self.get_register("ip") else {
            return Err(CPUError::RegisterNotFound("ip"));
        };

        self.memory.set_rpos(next_instruction_address.into());
        let Ok(instruction) = self.memory.read_u16() else {
            return Err(CPUError::ReadRegisterError)
        };

        let res = self.set_register("ip", next_instruction_address + 2);
        match res {
            Err(err) => return Err(err),
            Ok(_) => (),
        };

        return Ok(instruction);
    }

    pub fn execute(&mut self, instruction: u8) -> Result<(), CPUError> {
        let res = match instruction {
            // move literal into r1 register
            Instructions::MOV_LIT_R1 => {
                // get the literal value (next 2 bytes in memory pointed to by the IP)
                let Ok(literal) = self.fetch_16() else {
                    return Err(CPUError::ReadRegisterError)
                };

                self.set_register("r1", literal)
            }
            // move literal into r2 register
            Instructions::MOV_LIT_R2 => {
                // get the literal value (next 2 bytes in memory pointed to by the IP)
                let Ok(literal) = self.fetch_16() else {
                    return Err(CPUError::ReadRegisterError)
                };

                self.set_register("r2", literal)
            }
            // Add register to register
            Instructions::ADD_REG_REG => {
                let r1 = self.fetch().unwrap();
                let r2 = self.fetch().unwrap();

                self.registers.set_rpos((r1 * 2).into());
                let register_value_1 = self.registers.read_u16().unwrap();
                self.registers.set_rpos((r2 * 2).into());
                let register_value_2 = self.registers.read_u16().unwrap();
                let res = self.set_register("acc", register_value_1 + register_value_2);
                return match res {
                    Err(err) => Err(err),
                    Ok(_) => Ok(()),
                };
            }
            other => Err(CPUError::UnknownInstruction(other)),
        };

        return res;
    }

    pub fn step(&mut self) -> Result<(), CPUError> {
        let instruction = self.fetch()?;
        return self.execute(instruction);
    }

    pub fn debug(&mut self) {
        let reg_names = self.register_names.clone();
        for name in reg_names {
            let register = self.get_register(&name).unwrap();
            println!("{}: {:#06x}", &name, register);
        }
        println!();
    }
}
