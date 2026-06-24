use std::collections::HashMap;
use crate::frontend::vm::compiler::user_functions::UserFunction;

use super::loader::deserialize;

pub struct CallFrame {
    pub bytecode: Vec<u8>,
    pub locals: Vec<u64>,
    pub pc: usize,
}
pub struct VmState {
    pub constants: Vec<u64>,
    pub strings: Vec<String>,
    pub stack: Vec<u64>,
    pub string_pool: HashMap<String, usize>,
    pub tos: Option<u64>,
    pub stos: Option<u64>,
}

pub struct VirtualMachine<'a> {
    pub(super) user_functions: Vec<UserFunction<'a>>, 
    pub state: VmState,
    pub(super) frames: Vec<CallFrame>,
}

use std::fmt;
impl fmt::Display for CallFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CallFrame {{")?;
        writeln!(f, "    pc: {}", self.pc)?;
        writeln!(f, "    locals: {:?}", self.locals)?;
        writeln!(f, "    bytecode_len: {}", self.bytecode.len())?;
        write!(f, "}}")
    }
}

impl<'a> fmt::Display for VirtualMachine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Virtual Machine ===")?;
        writeln!(f, "Frames count: {}", self.frames.len())?;

        for (idx, frame) in self.frames.iter().enumerate() {
            writeln!(f, "\nFrame #{}:", idx)?;
            writeln!(f, "{}", frame)?;
        }

        Ok(())
    }
}

impl<'a> VirtualMachine<'a> {
    pub fn new(raw_code: Vec<u8>, user_functions: Vec<UserFunction<'a>>) -> Self {
        let (bytecode, constants, strings, string_pool, var_count) = deserialize(raw_code);

        // Creating the main frame
        let main_frame = CallFrame {
            bytecode,
            locals: vec![0; var_count],
            pc: 0
        };

        let new_vmstate = VmState {
            constants,
            strings,
            stack: Vec::new(),
            string_pool,
            tos: Some(0),
            stos: Some(0),
        };

        Self {
            user_functions,
            state: new_vmstate,
            frames: vec![main_frame], // Push main frame to CallStack
        }
    }
    pub fn run_bytecode(&mut self) -> Result<(), String> {
        while let Some(frame) = self.frames.last_mut() {
            if frame.pc > frame.bytecode.len() {
                self.frames.pop();
                continue;
            }

            let opcode = frame.bytecode[frame.pc];
            match opcode {
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
                0x2D..=0x37 => self.execute_io(opcode)?,

                0x40 => {
                    let func_id = frame.bytecode[frame.pc + 1];
                    self.execute_buildin(func_id)?;
                }
                
                0x41..=0x42 => {
                    self.execute_flow(opcode)?;
                    // println!("Frames: {}", self.frames.len());
                    // for frame in self.frames.iter() {
                    //     println!("{:}", frame)
                    // }
                    
                }

                _ => {
                    return Err(format!(
                        "Unknown opcode: 0x{:02X} at PC: {}",
                        opcode, frame.pc
                    ));
                }
            }
        }
        Ok(())
    }
}
