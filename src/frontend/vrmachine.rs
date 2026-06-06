pub struct VirtualMachine<'a> {
    bytecode: Vec<u16>,
    constants: &'a [i64],        
    stack: Vec<i64>,             
    globals: Vec<i64>,           
    pc: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(bytecode: Vec<u16>, constants: &'a [i64], var_count: usize) -> Self {
        Self {
            bytecode,
            constants,
            stack: Vec::new(),
            globals: vec![0; var_count], 
            pc: 0,
        }
    }

    
    pub fn run_bytecode(&mut self) -> Result<(), String> {
        let len = self.bytecode.len();

        while self.pc < len {
            let opcode = self.bytecode[self.pc];
            self.pc += 1;
            
            match opcode {
                // Opcode Stop (0x00) -> Мягкий выход из выполнения программы
                0x00 => break,

                // Opcode LoadConst (0x01)
                0x01 => {
                    let idx = self.bytecode[self.pc] as usize;
                    self.pc += 1;
                    let value = self.constants[idx];
                    self.stack.push(value);
                }

                // Opcode LoadVar (0x02)
                0x02 => {
                    let idx = self.bytecode[self.pc] as usize;
                    self.pc += 1;
                    let value = self.globals[idx];
                    self.stack.push(value);
                }

                // Opcode StoreVar (0x03)
                0x03 => {
                    let idx = self.bytecode[self.pc] as usize;
                    self.pc += 1;
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
                    self.stack.push(a * b);
                }

                // Opcode Div (0x07)
                0x07 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg B)".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in DIV (arg A)".to_string())?;
                    if b == 0 {
                        return Err("VM Runtime Error: Division by zero!".to_string());
                    }
                    self.stack.push(a / b);
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

                // Opcode Negate (0x0B)
                0x0B => {
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in Negate".to_string())?;
                    self.stack.push(-a);
                }

                // Opcode Equal (0x0C)
                0x0C => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in ==".to_string())?;
                    self.stack.push(if a == b { 1 } else { 0 });
                }

                // Opcode NotEqual (0x0D)
                0x0D => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in !=".to_string())?;
                    self.stack.push(if a != b { 1 } else { 0 });
                }

                // Opcode Greater (0x0E)
                0x0E => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >".to_string())?;
                    self.stack.push(if a > b { 1 } else { 0 });
                }

                // Opcode Less (0x0F)
                0x0F => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <".to_string())?;
                    self.stack.push(if a < b { 1 } else { 0 });
                }

                // Opcode GreaterEq (0x10)
                0x10 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in >=".to_string())?;
                    self.stack.push(if a >= b { 1 } else { 0 });
                }

                // Opcode LessEq (0x11)
                0x11 => {
                    let b = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                    let a = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in <=".to_string())?;
                    self.stack.push(if a <= b { 1 } else { 0 });
                }

                // Opcode Jump (0x12)
                0x12 => {
                    let target = self.bytecode[self.pc] as usize;
                    self.pc = target;
                }

                // Opcode JumpIfFalse (0x13)
                0x13 => {
                    let condition = self.stack.pop().ok_or_else(|| "VM Error: Stack underflow in JumpIfFalse".to_string())?;
                    let target = self.bytecode[self.pc] as usize;
                    self.pc += 1;
                    if condition == 0 {
                        self.pc = target;
                    }
                }

                // Opcode Print (0x14)
                0x14 => {
                    let value = self.stack.pop().ok_or_else(|| "VM Runtime Error: Nothing to print".to_string())?;
                    println!("{}", value);
                }

                _ => return Err(format!("Unknown opcode: 0x{:02X}", opcode)),
            }
        }
        Ok(())
    }
}