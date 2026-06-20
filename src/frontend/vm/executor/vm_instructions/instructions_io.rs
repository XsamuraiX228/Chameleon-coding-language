use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl VirtualMachine {
    #[inline(always)]
    pub fn execute_io(&mut self, opcode: u8) -> Result<(), String> {
        match opcode {
            // Print int
            0x2D => {
                println!("{}", self.get_a_i64()?);
                self.pc += 1;
            }
            // Print float
            0x2E => {
                println!("{}", self.get_a_f64()?);
                self.pc += 1;
            }
            // Print bool
            0x2F => {
                println!(
                    "{}",
                    if self.pop_u64()? == 0 {
                        "false"
                    } else {
                        "true"
                    }
                );
                self.pc += 1;
            }
            // Print text
            0x30 => {
                let id = self.pop_u64()? as usize;
                if let Some(text) = self.strings.get(id) {
                    println!("{}", text);
                } else {
                    return Err("VM Error: Invalid string ID".to_string());
                }
                self.pc += 1;
            }

            // Input int
            0x31 => {
                let arg = self.get_arg();
                let mut txt = String::new();
                std::io::stdin()
                    .read_line(&mut txt)
                    .map_err(|e| e.to_string())?;
                let val = txt.trim().parse::<i64>().map_err(|e| e.to_string())?;

                self.globals[arg as usize] = val as u64;
                self.pc += 3;
            }
            // Input float
            0x32 => {
                let arg = self.get_arg();
                let mut txt = String::new();
                std::io::stdin()
                    .read_line(&mut txt)
                    .map_err(|e| e.to_string())?;
                let val = txt.trim().parse::<f64>().map_err(|e| e.to_string())?;

                self.globals[arg as usize] = val.to_bits();
                self.pc += 3;
            }
            // Input text
            0x33 => {
                let arg = self.get_arg();
                let mut txt = String::new();
                std::io::stdin()
                    .read_line(&mut txt)
                    .map_err(|e| e.to_string())?;
                let txt = txt.trim().to_string();

                let id = if let Some(&text_id) = self.string_pool.get(&txt) {
                    text_id
                } else {
                    let new_id = self.strings.len();
                    self.strings.push(txt.clone());
                    self.string_pool.insert(txt, new_id);
                    new_id
                };

                self.globals[arg as usize] = id as u64;
                self.pc += 3;
            }
            _ => {
                println!("Error in execution io opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
