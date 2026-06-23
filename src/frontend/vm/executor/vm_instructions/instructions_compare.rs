use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl<'a> VirtualMachine<'a>{
    #[inline(always)]
    pub fn execute_compare(&mut self, opcode: u8) -> Result<(), String> {
        let frame = self
            .frames
            .last_mut()
            .ok_or("VM Error: No active call frame")?;

        match opcode {
            // --- i64 Compare ---

            // 0x12: Equal
            0x12 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a == b) as u64)?;
                frame.pc += 1;
            }
            // 0x13: Not Equal
            0x13 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a != b) as u64)?;
                frame.pc += 1;
            }
            // 0x14: Less Than
            0x14 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a < b) as u64)?;
                frame.pc += 1;
            }
            // 0x15: Less or Equal
            0x15 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a <= b) as u64)?;
                frame.pc += 1;
            }
            // 0x16: Greater Than
            0x16 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a > b) as u64)?;
                frame.pc += 1;
            }
            // 0x17: Greater or Equal
            0x17 => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64((a >= b) as u64)?;
                frame.pc += 1;
            }

            // --- f64 Compare ---

            // 0x18: F-Equal
            0x18 => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a == b) as u64)?;
                frame.pc += 1;
            }
            // 0x19: F-Not Equal
            0x19 => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a != b) as u64)?;
                frame.pc += 1;
            }
            // 0x1A: F-Less Than
            0x1A => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a < b) as u64)?;
                frame.pc += 1;
            }
            // 0x1B: F-Less or Equal
            0x1B => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a <= b) as u64)?;
                frame.pc += 1;
            }
            // 0x1C: F-Greater Than
            0x1C => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a > b) as u64)?;
                frame.pc += 1;
            }
            // 0x1D: F-Greater or Equal
            0x1D => {
                let (a, b) = self.state.get_a_b_f64()?;
                self.state.push_u64((a >= b) as u64)?;
                frame.pc += 1;
            }

            // --- Logical Operators ---

            // 0x2A: And
            0x2A => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64(((a != 0) && (b != 0)) as u64)?;
                frame.pc += 1;
            }
            // 0x2B: Or
            0x2B => {
                let (a, b) = self.state.get_a_b_i64()?;
                self.state.push_u64(((a != 0) || (b != 0)) as u64)?;
                frame.pc += 1;
            }
            // 0x2C: Not
            0x2C => {
                let cond = self.state.get_a_i64()?;
                self.state.push_u64((cond == 0) as u64)?;
                frame.pc += 1;
            }

            _ => {
                println!("Error in execution compare/logic opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
