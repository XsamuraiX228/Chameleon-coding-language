use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl VirtualMachine {
    #[inline(always)]
    pub fn execute_math(&mut self, opcode: u8) -> Result<(), String> {
        match opcode {
            // I64 Math
            0x04 => {
                // IAdd
                let (a, b) = self.get_a_b_i64()?;
                self.push_u64((a + b) as u64)?;
                self.pc += 1;
            }
            0x05 => {
                // ISub
                let (a, b) = self.get_a_b_i64()?;
                self.push_u64((a - b) as u64)?;
                self.pc += 1;
            }
            0x06 => {
                // IMul
                let (a, b) = self.get_a_b_i64()?;
                self.push_u64((a * b) as u64)?;
                self.pc += 1;
            }
            0x07 => {
                // IDiv
                let (a, b) = self.get_a_b_i64()?;
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                self.push_u64((a / b) as u64)?;
                self.pc += 1;
            }
            0x08 => {
                // IMod
                let (a, b) = self.get_a_b_i64()?;
                self.push_u64((a % b) as u64)?;
                self.pc += 1;
            }
            0x09 => {
                // IPow
                let (a, b) = self.get_a_b_i64()?;
                self.push_u64(a.pow(b as u32) as u64)?;
                self.pc += 1;
            }
            0x0A => {
                // INegate
                let a = self.get_a_i64()?;
                self.push_u64((-a) as u64)?;
                self.pc += 1;
            }

            // F64 Math
            0x0B => {
                // FAdd
                let (a, b) = self.get_a_b_f64()?;
                self.push_u64((a + b).to_bits())?;
                self.pc += 1;
            }
            0x0C => {
                // FSub
                let (a, b) = self.get_a_b_f64()?;
                self.push_u64((a - b).to_bits())?;
                self.pc += 1;
            }
            0x0D => {
                // FMul
                let (a, b) = self.get_a_b_f64()?;
                self.push_u64((a * b).to_bits())?;
                self.pc += 1;
            }
            0x0E => {
                // FDiv
                let (a, b) = self.get_a_b_f64()?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                self.push_u64((a / b).to_bits())?;
                self.pc += 1;
            }
            0x0F => {
                // FMod
                let (a, b) = self.get_a_b_f64()?;
                self.push_u64((a % b).to_bits())?;
                self.pc += 1;
            }
            0x10 => {
                //  FPow
                let (a, b) = self.get_a_b_f64()?;
                self.push_u64(a.powf(b).to_bits())?;
                self.pc += 1;
            }
            0x11 => {
                // FNegate
                let a = self.get_a_f64()?;
                self.push_u64((-a).to_bits())?;
                self.pc += 1;
            }
            _ => {
                println!("Error in execution math opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
