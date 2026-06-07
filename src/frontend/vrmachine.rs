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
    pub fn read_bytes(&mut self) -> usize {
        let bytes:[u8; 2] = [
            self.bytecode[self.pc],
            self.bytecode[self.pc + 1],
        ];

        self.pc += 2;
        u16::from_le_bytes(bytes) as usize
    }
    
    pub fn run_bytecode(&mut self) -> Result<(), String> {
        let len = self.bytecode.len();

        while self.pc < len {
            let opcode = self.bytecode[self.pc];
            self.pc += 1; 
            
            match opcode {
                // Opcode Stop (0x00) 
                0x00 => break,

                // Opcode LoadConst (0x01)
                0x01 => {
                    let idx = self.read_bytes();
                    let value = self.constants[idx];
                    self.stack.push(value);
                }

                // Opcode LoadVar (0x02)
                0x02 => {
                    let idx = self.read_bytes();
                    let value = self.globals[idx];
                    self.stack.push(value);
                }

                // Opcode StoreVar (0x03)
                0x03 => {
                    let idx = self.read_bytes();
                    let value = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow during StoreVar".to_string())?;
                    self.globals[idx] = value;
                }

                // Opcode Add (0x04)
                0x04 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ADD (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ADD (arg A)".to_string())?;
                    self.stack.push(a + b);
                }

                // Opcode Sub (0x05)
                0x05 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in SUB (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in SUB (arg A)".to_string())?;
                    self.stack.push(a - b);
                }

                // Opcode Mul (0x06)
                0x06 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MUL (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MUL (arg A)".to_string())?;
                    if b == 2 {
                        self.stack.push(a << 1);
                    } else {
                        self.stack.push(a * b);
                    }
                }

                // Opcode Div (0x07)
                0x07 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg A)".to_string())?;
                    if b == 0 {
                        return Err("VM Runtime Error: Division by zero!".to_string());
                    } else if b == 2 {
                        self.stack.push(a >> 1);
                    } else {
                        self.stack.push(a / b);
                    }
                }

                // Opcode Mod (0x08)
                0x08 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MOD (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in MOD (arg A)".to_string())?;
                    if b == 0 {
                        return Err("VM Runtime Error: Division by zero in modulo!".to_string());
                    }
                    self.stack.push(a % b);
                }

                // Opcode Pow (0x09)
                0x09 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in POW (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in POW (arg A)".to_string())?;
                    if b < 0 {
                        return Err("VM Runtime Error: Negative exponent not supported for integers!".to_string());
                    }
                    self.stack.push(a.pow(b as u32));
                }

                // Opcode Negate (0x0A)
                0x0A => {
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in Negate".to_string())?;
                    self.stack.push(-a);
                }

                // Opcode Equal (0x0B)
                0x0B => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                    self.stack.push(if a == b { 1 } else { 0 });
                }

                // Opcode NotEqual (0x0C)
                0x0C => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                    self.stack.push(if a != b { 1 } else { 0 });
                }

                // Opcode Greater (0x0D)
                0x0D => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                    self.stack.push(if a > b { 1 } else { 0 });
                }

                // Opcode Less (0x0E)
                0x0E => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                    self.stack.push(if a < b { 1 } else { 0 });
                }

                // Opcode GreaterEq (0x0F)
                0x0F => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                    self.stack.push(if a >= b { 1 } else { 0 });
                }

                // Opcode LessEq (0x10)
                0x10 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                    self.stack.push(if a <= b { 1 } else { 0 });
                }

                // Opcode Jump (0x11)
                0x11 => {
                    let target = self.read_bytes();
                    self.pc = target; 
                }

                // Opcode JumpIfFalse (0x12)
                0x12 => {
                    let condition = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in JumpIfFalse".to_string())?;
                    let target = self.read_bytes();
                    if condition == 0 {
                        self.pc = target; 
                    }
                }

                // Input (0x13)
                0x13 => {
                    let idx = self.read_bytes();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read line");
                    let value = input.trim().parse::<i64>().unwrap_or(0);
                    self.globals[idx] = value;
                }

                // Opcode Print (0x14)
                0x14 => {
                    let value = self.stack.pop().ok_or_else(|| "VM Runtime Error: Nothing to print".to_string())?;
                    println!("{}", value);
                }

                // Increment var (0x15)
                0x15 => {
                    let var_idx = self.read_bytes();
                    let inc_idx = self.read_bytes();
                    self.globals[var_idx] += self.constants[inc_idx];
                }

                // MovVar (0x16)
                0x16 => {
                    let var_2_idx = self.read_bytes();
                    let var_1_idx = self.read_bytes();
                    self.globals[var_2_idx] = self.globals[var_1_idx];
                }

                // JVLC -> Jump Variable LESS Constant
                0x17 => {
                    let var_idx = self.read_bytes();
                    let const_idx = self.read_bytes();
                    let target = self.read_bytes();
                    if self.globals[var_idx] <= self.constants[const_idx] {
                        self.pc = target;
                    }
                }

                // JVGC -> Jump Variable GREATER Constant
                0x18 => {
                    let var_idx = self.read_bytes();
                    let const_idx = self.read_bytes();
                    let target = self.read_bytes();
                    if self.globals[var_idx] >= self.constants[const_idx] {
                        self.pc = target;
                    }
                }
                _ => return Err(format!("Unknown opcode: 0x{:02X} at PC: {}", opcode, self.pc - 1)),
            }
        }
        Ok(())
    }
}