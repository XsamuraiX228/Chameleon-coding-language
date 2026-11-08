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
                // Get the current frame
                if let Some(current_frame) = self.frames.last_mut() {
                    // Get the id
                    let func_id = current_frame.get_arg() as usize;
                    current_frame.pc += 3; // Skip 0x41 + 2 bytes for argument

                    // Here we check if we can use TCO
                    // We lookahead to check if the pattern 41 00 00 42 is exist
                    if current_frame.pc <= current_frame.bytecode.len() && current_frame.bytecode[current_frame.pc] == 0x42 {
                        if let Some(func_data) = self.user_functions.get(func_id) {
                            // Instead of creating new local vector
                            // We simple reload old values to new one
                            for i in (0..func_data.args_amout).rev() {
                                current_frame.locals[i] = self.state.pop_u64()?;
                            }
                        }
                        current_frame.pc = 0;
                        return Ok(());
                    }

                    // If the pattern TCO hasn't been captured, then we have a simple function
                    if let Some(func_data) = self.user_functions.get(func_id) {
                        let mut locals = vec![0; func_data.local_vars.len()];

                        // We create new vector and load it with values from global stack
                        let args_count = func_data.args_amout;
                        for i in (0..args_count).rev() {
                            locals[i] = self.state.pop_u64()?;
                        }

                        let new_frame = CallFrame {
                            bytecode: func_data.function_bc.clone(),
                            locals, 
                            pc: 0,
                        };

                        // After the frame is ready, it goes to vector frames and will be deleted
                        // When the function end
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
