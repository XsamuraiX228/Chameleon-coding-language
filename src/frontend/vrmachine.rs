pub struct VirtualMachine {
    bytecode: Vec<u8>,
    constants: Vec<i64>,        
    stack: Vec<i64>,             
    globals: Vec<i64>,           
    pc: usize,
}

impl<'a> VirtualMachine {
    pub fn new(bytecode: Vec<u8>, constants: Vec<i64>, var_count: usize) -> Self {
        Self {
            bytecode,
            constants,
            stack: Vec::new(),
            globals: vec![0; var_count], 
            pc: 0,
        }
    }
    
    #[inline(always)]
    fn get_arg(&mut self) -> u16 {
        let low = self.bytecode[self.pc + 1] as u16;
        let high = self.bytecode[self.pc + 2] as u16;
        (high << 8) | low
    }
    
    pub fn run_bytecode(&mut self) -> Result<(), String> {
    let len = self.bytecode.len();

    
    while self.pc < len {
        let opcode = self.bytecode[self.pc];

        match opcode {
            0x00 => break,

            // С аргументом (pc += 3)
            0x01 => {
                let arg = self.get_arg();
                self.stack.push(self.constants[arg as usize]);
                self.pc += 3;
            }
            0x02 => {
                let arg = self.get_arg();
                self.stack.push(self.globals[arg as usize]);
                self.pc += 3;
            }
            0x03 => {
                let arg = self.get_arg();
                let value = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow during StoreVar".to_string())?;
                self.globals[arg as usize] = value;
                self.pc += 3;
            }

            // Без аргумента (pc += 1)
            0x04 => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ADD (arg B)".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ADD (arg A)".to_string())?;
                self.stack.push(a + b);
                self.pc += 1;
            }
            0x05 => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in SUB (arg B)".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in SUB (arg A)".to_string())?;
                self.stack.push(a - b);
                self.pc += 1;
            }
            0x06 => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MUL (arg B)".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MUL (arg A)".to_string())?;
                self.stack.push(a * b);
                self.pc += 1;
            }
            0x07 => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg B)".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg A)".to_string())?;
                if b == 0 {
                    return Err("VM Runtime Error: Division by zero!".to_string());
                }
                self.stack.push(a / b);
                self.pc += 1;
            }
            0x08 => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MOD (arg B)".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MOD (arg A)".to_string())?;
                if b == 0 {
                    return Err("VM Runtime Error: Division by zero in modulo!".to_string());
                }
                self.stack.push(a % b);
                self.pc += 1;
            }
            0x09 => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in POW (arg B)".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in POW (arg A)".to_string())?;
                if b < 0 {
                    return Err("VM Runtime Error: Negative exponent not supported for integers!".to_string());
                }
                self.stack.push(a.pow(b as u32));
                self.pc += 1;
            }
            0x0A => {
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in Negate".to_string())?;
                self.stack.push(-a);
                self.pc += 1;
            }
            0x0B => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                self.stack.push(if a == b { 1 } else { 0 });
                self.pc += 1;
            }
            0x0C => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                self.stack.push(if a != b { 1 } else { 0 });
                self.pc += 1;
            }
            0x0D => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                self.stack.push(if a < b { 1 } else { 0 });
                self.pc += 1;
            }
            0x0E => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                self.stack.push(if a <= b { 1 } else { 0 });
                self.pc += 1;
            }
            0x0F => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                self.stack.push(if a > b { 1 } else { 0 });
                self.pc += 1;
            }
            0x10 => {
                let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                self.stack.push(if a >= b { 1 } else { 0 });
                self.pc += 1;
            }
            0x11 => {
                let arg = self.get_arg();
                self.pc = arg as usize; // теперь просто индекс байта, не * 3
            }
            0x12 => {
                let arg = self.get_arg();
                let condition = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in JumpIfFalse".to_string())?;
                if condition == 0 {
                    self.pc = arg as usize;
                } else {
                    self.pc += 3;
                }
            }
            0x13 => {
                let value = self.stack.pop().ok_or_else(|| "VM Runtime Error: Nothing to print".to_string())?;
                println!("{}", value);
                self.pc += 1;
            }
            0x14 => {
                let arg = self.get_arg();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
                let value = input.trim().parse::<i64>().unwrap_or(0);
                self.globals[arg as usize] = value;
                self.pc += 3;
            }
            _ => return Err(format!("Unknown opcode: 0x{:02X} at PC: {}", opcode, self.pc)),
        }
    }
    Ok(())
}
}