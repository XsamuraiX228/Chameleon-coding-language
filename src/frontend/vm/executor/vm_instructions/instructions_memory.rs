use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl VirtualMachine {
    #[inline(always)]
    pub fn execute_memory(&mut self, opcode: u8) -> Result<(), String> {
        match opcode {
            // 0x01: LOAD_CONST
            0x01 => {
                let arg = self.get_arg();
                self.stack.push(self.constants[arg as usize]);
                self.pc += 3;
            }

            // 0x02: LOAD_VAR
            0x02 => {
                let arg = self.get_arg();
                self.stack.push(self.globals[arg as usize]);
                self.pc += 3;
            }

            // 0x03: STORE_VAR
            0x03 => {
                let arg = self.get_arg();
                let value = self.pop_u64()?;
                self.globals[arg as usize] = value;
                self.pc += 3;
            }

            _ => {
                println!("Error in execution memory opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
