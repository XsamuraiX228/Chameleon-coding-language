use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl VirtualMachine {
    #[inline(always)]
    pub fn execute_compare(&mut self, opcode: u8) -> Result<(), String> {
        match opcode {
            // i64 Compare
            0x12 => {
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push((a == b) as u64);
                self.pc += 1;
            }
            0x13 => {
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push((a != b) as u64);
                self.pc += 1;
            }
            0x14 => {
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push((a < b) as u64);
                self.pc += 1;
            }
            0x15 => {
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push((a <= b) as u64);
                self.pc += 1;
            }
            0x16 => {
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push((a > b) as u64);
                self.pc += 1;
            }
            0x17 => {
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push((a >= b) as u64);
                self.pc += 1;
            }

            // f64 Compare
            0x18 => {
                let (a, b) = self.get_a_b_f64()?;
                self.stack.push((a == b) as u64);
                self.pc += 1;
            }
            0x19 => {
                let (a, b) = self.get_a_b_f64()?;
                self.stack.push((a != b) as u64);
                self.pc += 1;
            }
            0x1A => {
                let (a, b) = self.get_a_b_f64()?;
                self.stack.push((a < b) as u64);
                self.pc += 1;
            }
            0x1B => {
                let (a, b) = self.get_a_b_f64()?;
                self.stack.push((a <= b) as u64);
                self.pc += 1;
            }
            0x1C => {
                let (a, b) = self.get_a_b_f64()?;
                self.stack.push((a > b) as u64);
                self.pc += 1;
            }
            0x1D => {
                let (a, b) = self.get_a_b_f64()?;
                self.stack.push((a >= b) as u64);
                self.pc += 1;
            }

            // Combine logic operators
            0x2A => {
                // And
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push(((a != 0) && (b != 0)) as u64);
                self.pc += 1;
            }
            0x2B => {
                // Or
                let (a, b) = self.get_a_b_i64()?;
                self.stack.push(((a != 0) || (b != 0)) as u64);
                self.pc += 1;
            }
            0x2C => {
                // Not
                let cond = self.get_a_i64()?;
                self.stack.push((cond == 0) as u64);
                self.pc += 1;
            }

            _ => {
                println!("Error in execution flow opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
