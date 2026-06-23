use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl<'a> VirtualMachine<'a>{
    #[inline(always)]
    pub fn execute_memory(&mut self, opcode: u8) -> Result<(), String> {
        let frame = self
            .frames
            .last_mut()
            .ok_or("VM Error: No active call frame")?;
        match opcode {
            // 0x01: LOAD_CONST
            0x01 => {
                let arg = frame.get_arg();
                let value = self.state.constants[arg as usize];
                self.state.push_u64(value)?;
                frame.pc += 3
            }

            // 0x02: LOAD_VAR
            0x02 => {
                let arg = frame.get_arg();
                let value = frame.locals[arg as usize];
                self.state.push_u64(value)?;
                frame.pc += 3;
            }

            // 0x03: STORE_VAR
            0x03 => {
                let arg = frame.get_arg();
                let value = self.state.pop_u64()?;
                frame.locals[arg as usize] = value;
                frame.pc += 3;
            }

            _ => {
                println!("Error in execution memory opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
