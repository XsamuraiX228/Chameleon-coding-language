use crate::frontend::vm::executor::vrmachine::{CallFrame, VirtualMachine};

impl<'a> VirtualMachine<'a> {
    #[inline(always)]
    pub fn execute_flow(&mut self, opcode: u8) -> Result<(), String> {
        let frame = self
            .frames
            .last_mut()
            .ok_or("VM Error: No active call frame")?;

        match opcode {
            // JUMP
            0x1E => {
                let arg = frame.get_arg();
                frame.pc = arg as usize;
            }

            // JUMP_IF_FALSE
            0x1F => {
                let arg = frame.get_arg();
                if self.state.pop_u64()? == 0 {
                    frame.pc = arg as usize;
                } else {
                    frame.pc += 3;
                }
            }

            // Call opcode, which executes, when the functions is being called
            // e.g LET X = Foo(5)
            0x41 => {
                if let Some(current_frame) = self.frames.last_mut() {
                    let func_id = current_frame.get_arg() as usize;

                    current_frame.pc += 3;

                    if let Some(func_data) = self.user_functions.get(func_id) {
                        
                        let mut locals = vec![0; func_data.local_vars.len()];

                        let args_count = func_data.args_amout;
                        for i in (0..args_count).rev() {
                            locals[i] = self.state.pop_u64()?;
                        }

                        let new_frame = CallFrame {
                            bytecode: func_data.function_bc.clone(),
                            locals, 
                            pc: 0,
                        };

                        self.frames.push(new_frame);
                    } else {
                        return Err(format!(
                            "VM Error: User function with ID {} not found",
                            func_id
                        ));
                    }
                } else {
                    return Err("VM Error: No active frame to execute CALL".to_string());
                }
            }
            0x42 => {
                let return_value = self.state.pop_u64()?;

                self.frames.pop();

                if let Some(_) = self.frames.last_mut() {
                    self.state.push_u64(return_value)?;
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
