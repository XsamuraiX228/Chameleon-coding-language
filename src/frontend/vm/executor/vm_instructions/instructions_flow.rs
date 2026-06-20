use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl VirtualMachine {
    #[inline(always)]
    pub fn execute_flow(&mut self, opcode: u8) -> Result<(), String> {
        match opcode {
            // JUMP
            0x1E => {
                let arg = self.get_arg();
                self.pc = arg as usize;
            }

            // JUMP_IF_FALSE
            0x1F => {
                let arg = self.get_arg();
                if self.pop_u64()? == 0 {
                    self.pc = arg as usize;
                } else {
                    self.pc += 3;
                }
            }
            _ => {
                println!("Error in execution flow opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
