use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl<'a> VirtualMachine<'a> {
    #[inline(always)]
    pub fn execute_math(&mut self, opcode: u8) -> Result<(), String> {
        let frame = self
            .frames
            .last_mut()
            .ok_or("VM Error: No active call frame")?;
        match opcode {
            // --- I64 Math ---

            // 0x04: IAdd
            0x04 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a + b) as u64)?;
                frame.pc += 1;
            }

            // 0x05: ISub
            0x05 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a - b) as u64)?;
                frame.pc += 1;
            }

            // 0x06: IMul
            0x06 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a * b) as u64)?;
                frame.pc += 1;
            }

            // 0x07: IDiv
            0x07 => {
                let (a, b) = self.state.get_a_b_i64()?;
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                self.state.push_u64((a / b) as u64)?;
                frame.pc += 1;
            }

            // 0x08: IMod
            0x08 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a % b) as u64)?;
                frame.pc += 1;
            }

            // 0x09: IPow
            0x09 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64(a.pow(b as u32) as u64)?;
                frame.pc += 1;
            }

            // 0x0A: INegate
            0x0A => {
                let a = self.state.get_a_i64()?;
                self.state.push_u64((-a) as u64)?;
                frame.pc += 1;
            }

            // --- F64 Math ---

            // 0x0B: FAdd
            0x0B => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a + b).to_bits())?;
                frame.pc += 1;
            }

            // 0x0C: FSub
            0x0C => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a - b).to_bits())?;
                frame.pc += 1;
            }

            // 0x0D: FMul
            0x0D => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a * b).to_bits())?;
                frame.pc += 1;
            }

            // 0x0E: FDiv
            0x0E => {
                let (a, b) = self.state.get_a_b_f64()?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                self.state.push_u64((a / b).to_bits())?;
                frame.pc += 1;
            }

            // 0x0F: FMod
            0x0F => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a % b).to_bits())?;
                frame.pc += 1;
            }

            // 0x10: FPow
            0x10 => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64(a.powf(b).to_bits())?;
                frame.pc += 1;
            }

            // 0x11: FNegate
            0x11 => {
                let a = self.state.get_a_f64()?;
                self.state.push_u64((-a).to_bits())?;
                frame.pc += 1;
            }

            _ => {
                println!("Error in execution math opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
