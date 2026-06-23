use crate::frontend::vm::executor::vrmachine::VirtualMachine;

impl<'a> VirtualMachine<'a> {
    #[inline(always)]
    pub fn execute_io(&mut self, opcode: u8) -> Result<(), String> {
        let frame = self
            .frames
            .last_mut()
            .ok_or("VM Error: No active call frame")?;

        match opcode {
            // --- Print ---

            // 0x2D: Print int
            0x2D => {
                print!("{}", self.state.get_a_i64()?);
                frame.pc += 1;
            }
            // 0x2E: Print float
            0x2E => {
                print!("{}", self.state.get_a_f64()?);
                frame.pc += 1;
            }
            // 0x2F: Print bool
            0x2F => {
                print!(
                    "{}",
                    if self.state.pop_u64()? == 0 {
                        "false"
                    } else {
                        "true"
                    }
                );
                frame.pc += 1;
            }
            // 0x30: Print text
            0x30 => {
                let id = self.state.pop_u64()? as usize;
                if let Some(text) = self.state.strings.get(id) {
                    print!("{}", text);
                } else {
                    return Err("VM Error: Invalid string ID".to_string());
                }
                frame.pc += 1;
            }

            // --- Println ---

            // 0x31: Println int
            0x31 => {
                println!("{}", self.state.get_a_i64()?);
                frame.pc += 1;
            }
            // 0x32: Println float
            0x32 => {
                println!("{}", self.state.get_a_f64()?);
                frame.pc += 1;
            }
            // 0x33: Println bool
            0x33 => {
                println!(
                    "{}",
                    if self.state.pop_u64()? == 0 {
                        "false"
                    } else {
                        "true"
                    }
                );
                frame.pc += 1;
            }
            // 0x34: Println text
            0x34 => {
                let id = self.state.pop_u64()? as usize;
                if let Some(text) = self.state.strings.get(id) {
                    println!("{}", text);
                } else {
                    return Err("VM Error: Invalid string ID".to_string());
                }
                frame.pc += 1;
            }

            // --- Input (Пишем в локальные переменные кадра) ---

            // 0x35: Input int
            0x35 => {
                let arg = frame.get_arg();
                let mut txt = String::new();
                std::io::stdin()
                    .read_line(&mut txt)
                    .map_err(|e| e.to_string())?;
                let val = txt.trim().parse::<i64>().map_err(|e| e.to_string())?;

                frame.locals[arg as usize] = val as u64;
                frame.pc += 3;
            }
            // 0x36: Input float
            0x36 => {
                let arg = frame.get_arg();
                let mut txt = String::new();
                std::io::stdin()
                    .read_line(&mut txt)
                    .map_err(|e| e.to_string())?;
                let val = txt.trim().parse::<f64>().map_err(|e| e.to_string())?;

                frame.locals[arg as usize] = val.to_bits();
                frame.pc += 3;
            }
            // 0x37: Input text
            0x37 => {
                let arg = frame.get_arg();
                let mut txt = String::new();
                std::io::stdin()
                    .read_line(&mut txt)
                    .map_err(|e| e.to_string())?;
                let txt = txt.trim().to_string();

                // Работаем с динамическим пулом строк внутри state
                let id = if let Some(&text_id) = self.state.string_pool.get(&txt) {
                    text_id
                } else {
                    let new_id = self.state.strings.len();
                    self.state.strings.push(txt.clone());
                    self.state.string_pool.insert(txt, new_id);
                    new_id
                };

                frame.locals[arg as usize] = id as u64;
                frame.pc += 3;
            }

            _ => {
                println!("Error in execution io opcodes");
                unreachable!()
            }
        }
        Ok(())
    }
}
