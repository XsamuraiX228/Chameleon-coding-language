use std::collections::HashMap;

use super::loader::deserialize;
pub struct VirtualMachine {
    pub(super) bytecode: Vec<u8>,
    pub(super) constants: Vec<u64>,
    pub(super) strings: Vec<String>,
    pub(super) stack: Vec<u64>,
    pub(super) globals: Vec<u64>,
    pub(super) string_pool: HashMap<String, usize>,
    pub(super) tos: Option<u64>,
    pub(super) stos: Option<u64>,
    pub(super) pc: usize,
}

impl VirtualMachine {
    pub fn new(raw_code: Vec<u8>) -> Self {
        let (bytecode, constants, strings, string_pool, var_count) = deserialize(raw_code);
        Self {
            bytecode,
            constants,
            strings,
            stack: Vec::with_capacity(1024),
            globals: vec![0; var_count],
            string_pool,
            pc: 0,
            tos: Some(0),
            stos: Some(0),
        }
    }
    #[inline(always)]
    pub(super) fn check_reg(&self) -> u8 {
        match (self.tos, self.stos)  {
            (Some(_), Some(_)) => 2,
            (Some(_), None) => 1,
            (None, None) => 0,
            (None, Some(_)) => unreachable!("VM Error: STOS isn't empty, while TOS is empty!"),
        }
    }

    pub fn run_bytecode(&mut self) -> Result<(), String> {
        let len = self.bytecode.len();

        while self.pc < len {
            let opcode = self.bytecode[self.pc];

            match opcode {
                // 0x00: STOP
                0x00 => break,

                // Memory opcodes
                0x01..=0x03 => self.execute_memory(opcode)?,

                // Math opcodes
                0x04..=0x11 => self.execute_math(opcode)?,

                // Comparative opcodes
                0x12..=0x1D => self.execute_compare(opcode)?,
                // Also comparative opcodes (AND, OR, NOT)
                0x2A..=0x2C => self.execute_compare(opcode)?,

                // Flow opcodes (JUMP and JUMP_IF_FALSE)
                0x1E..=0x1F => self.execute_flow(opcode)?,

                // IO opcodes
                0x2D..=0x33 => self.execute_io(opcode)?,

                0x40 => {
                    let func_id = self.bytecode[self.pc + 1];
                    self.execute_buildin(func_id)?;
                },

                _ => {
                    return Err(format!(
                        "Unknown opcode: 0x{:02X} at PC: {}",
                        opcode, self.pc
                    ));
                }
            }
        }
        Ok(())
    }
}
