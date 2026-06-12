pub struct VirtualMachine {
    bytecode: Vec<u8>,
    constants: Vec<i64>,        
    stack: Vec<i64>,             
    globals: Vec<i64>,       
    string_pool: Vec<String>,    
    pc: usize,
}

impl<'a> VirtualMachine {
    pub fn new(raw_code: Vec<u8>) -> Self {
        let (bytecode, constants, var_count, strings) = Self::deserialize(raw_code);
        Self {
            bytecode,
            constants,
            stack: Vec::new(),
            globals: vec![0; var_count], 
            string_pool: strings,
            pc: 0,
        }
    }
    
    #[inline(always)]
    fn get_arg(&mut self) -> u16 {
        let low = self.bytecode[self.pc + 1] as u16;
        let high = self.bytecode[self.pc + 2] as u16;
        (high << 8) | low
    }

    pub fn deserialize(data: Vec<u8>) -> (Vec<u8>, Vec<i64>, usize, Vec<String>) {
        // ptr to vec
        let mut pos = 0;

        // amount of number constants
        let const_count = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
        pos += 2;

        // number constants
        let mut constants = Vec::with_capacity(const_count);
        for _ in 0..const_count {
            let bytes: [u8; 8] = data[pos..pos+8].try_into().unwrap();
            constants.push(i64::from_le_bytes(bytes));
            pos += 8;
        }

        // amount of variables
        let var_count = (data[pos] as usize) | ((data[pos  + 1] as usize) << 8);
        pos += 2;

        let string_count = (data[pos] as usize) | ((data[pos  + 1] as usize) << 8);
        pos += 2;

        let mut strings: Vec<String> = Vec::with_capacity(string_count);
        for _ in 0..string_count {
            let length = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
            pos += 2;
            let bytes = &data[pos..pos+length];
            pos += length;
            let string = String::from_utf8(bytes.to_vec()).unwrap_or("Error in string converting".to_string());
            strings.push(string);
        }

        let bytecode = data[pos..].to_vec();
        (bytecode, constants, var_count, strings)
    }

    

    
    pub fn run_bytecode(&mut self) -> Result<(), String> {
        let len = self.bytecode.len();

        
        while self.pc < len {
            let opcode = self.bytecode[self.pc];

            match opcode {
                // 0x00: STOP - Halts execution of the virtual machine
                0x00 => break,
                
                // 0x01: LOAD_CONST - Pushes a constant onto the stack from the constants pool using a 2-byte index arg
                0x01 => {
                    let arg = self.get_arg();
                    self.stack.push(self.constants[arg as usize]);
                    self.pc += 3;
                }

                // 0x02: LOAD_VAR - Pushes the value of a global variable onto the stack from the globals array using a 2-byte index arg
                0x02 => {
                    let arg = self.get_arg();
                    self.stack.push(self.globals[arg as usize]);
                    self.pc += 3;
                }

                // 0x03 LOAD_STRING
                0x03 => {
                    let arg = self.get_arg();
                    self.pc += 3;
                    self.stack.push(arg as i64);
                }

                // 0x04: STORE_VAR - Pops the top value from the stack and stores it in the globals array at the 2-byte index arg
                0x04 => {
                    let arg = self.get_arg();
                    let value = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow during StoreVar".to_string())?;
                    self.globals[arg as usize] = value;
                    self.pc += 3;
                }

                // 0x05: ADD - Pops two values, adds them (a + b), and pushes the result back onto the stack
                0x05 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ADD (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ADD (arg A)".to_string())?;
                    self.stack.push(a + b);
                    self.pc += 1;
                }

                // 0x06: SUB - Pops two values, subtracts the first popped from the second (a - b), and pushes the result
                0x06 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in SUB (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in SUB (arg A)".to_string())?;
                    self.stack.push(a - b);
                    self.pc += 1;
                }

                // 0x07: MUL - Pops two values, multiplies them (a * b), and pushes the result back onto the stack
                0x07 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MUL (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MUL (arg A)".to_string())?;
                    self.stack.push(a * b);
                    self.pc += 1;
                }

                // 0x08: DIV - Pops two values, performs integer division (a / b), and pushes the result. Checks for division by zero.
                0x08 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg A)".to_string())?;
                    if b == 0 {
                        return Err("VM Runtime Error: Division by zero!".to_string());
                    }
                    self.stack.push(a / b);
                    self.pc += 1;
                }

                // 0x09: MOD - Pops two values, calculates the remainder (a % b), and pushes it. Checks for division by zero.
                0x09 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MOD (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MOD (arg A)".to_string())?;
                    if b == 0 {
                        return Err("VM Runtime Error: Division by zero in modulo!".to_string());
                    }
                    self.stack.push(a % b);
                    self.pc += 1;
                }

                // 0x0A: POW - Pops two values, raises 'a' to the power of 'b' (a^b), and pushes the result. Negative exponents are forbidden.
                0x0A => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in POW (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in POW (arg A)".to_string())?;
                    if b < 0 {
                        return Err("VM Runtime Error: Negative exponent not supported for integers!".to_string());
                    }
                    self.stack.push(a.pow(b as u32));
                    self.pc += 1;
                }

                // 0x0B: NEGATE - Pops the top value and pushes its arithmetic negation (-a) back onto the stack
                0x0B => {
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in Negate".to_string())?;
                    self.stack.push(-a);
                    self.pc += 1;
                }

                // 0x0C: EQUAL - Pops two values, checks if a == b, and pushes 1 (true) or 0 (false)
                0x0C => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                    self.stack.push(if a == b { 1 } else { 0 });
                    self.pc += 1;
                }

                // 0x0D: NOT_EQUAL - Pops two values, checks if a != b, and pushes 1 (true) or 0 (false)
                0x0D => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                    self.stack.push(if a != b { 1 } else { 0 });
                    self.pc += 1;
                }

                // 0x0E: LESS - Pops two values, checks if a < b, and pushes 1 (true) or 0 (false)
                0x0E => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                    self.stack.push(if a < b { 1 } else { 0 });
                    self.pc += 1;
                }

                // 0x0F: LESS_EQ - Pops two values, checks if a <= b, and pushes 1 (true) or 0 (false)
                0x0F => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                    self.stack.push(if a <= b { 1 } else { 0 });
                    self.pc += 1;
                }

                // 0x10: GREATER - Pops two values, checks if a > b, and pushes 1 (true) or 0 (false)
                0x10 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                    self.stack.push(if a > b { 1 } else { 0 });
                    self.pc += 1;
                }

                // 0x11: GREATER_EQ - Pops two values, checks if a >= b, and pushes 1 (true) or 0 (false)
                0x11 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                    self.stack.push(if a >= b { 1 } else { 0 });
                    self.pc += 1;
                }

                // 0x12: JUMP - Unconditionally updates the PC to the 2-byte absolute address arg
                0x12 => {
                    let arg = self.get_arg();
                    self.pc = arg as usize; 
                }

                // 0x13: JUMP_IF_FALSE - Pops a condition value; if it's 0 (false), jumps to the 2-byte address arg, otherwise skips the argument (+3 bytes)
                0x13 => {
                    let arg = self.get_arg();
                    let condition = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in JumpIfFalse".to_string())?;
                    if condition == 0 {
                        self.pc = arg as usize;
                    } else {
                        self.pc += 3;
                    }
                }

                // 0x14: PRINT NUM - Pops the top value from the stack and prints it to the standard output
                0x14 => {
                    let value = self.stack.pop().ok_or_else(|| "VM Runtime Error: Nothing to print".to_string())?;
                    println!("{}", value);
                    self.pc += 1;
                }

                // 0x15 PRINT STR - Pops the top index from the same stack and before printing, get the string from string_pool
                0x15 => {
                    let idx = self.stack.pop().ok_or_else(|| "VM Runtime Error: Nothing to print".to_string())?;
                    println!("{}", self.string_pool[idx as usize]);
                    self.pc += 1;
                }

                // 0x16: INPUT - Reads an integer line from stdin and saves it directly into the globals array at the 2-byte slot index arg
                0x16 => {
                    let arg = self.get_arg();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
                    let value = input.trim().parse::<i64>().unwrap_or(0);
                    self.globals[arg as usize] = value;
                    self.pc += 3;
                }

                // 0x17 And
                0x17 => {
                    let cond_1 = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in AND block first condition".to_string())?;
                    let cond_2 = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in AND block second condition".to_string())?;
                    let res = if cond_1 != 0 && cond_2 != 0 {1} else {0};
                    self.stack.push(res);
                    self.pc += 1
                }

                // 0x18 Or
                0x18 => {
                    let cond_1 = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in AND block first condition".to_string())?;
                    let cond_2 = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in AND block second condition".to_string())?;
                    let res = if cond_1 != 0 || cond_2 != 0 {1} else {0};
                    self.stack.push(res);
                    self.pc += 1
                }

                // Fallback for handling compiled bytecode corruption or parser bugs
                _ => return Err(format!("Unknown opcode: 0x{:02X} at PC: {}", opcode, self.pc)),
            }
        }
        Ok(())
    }
    // Добавь это в vrmachine.rs внутри impl VirtualMachine
    #[cfg(test)]
    pub fn get_stack(&self) -> &[i64] {
        &self.stack
    }

    #[cfg(test)]
    pub fn get_globals(&self) -> &[i64] {
        &self.globals
    }
}

