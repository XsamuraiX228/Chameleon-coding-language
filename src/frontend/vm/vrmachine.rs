#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

pub struct VirtualMachine {
    bytecode: Vec<u8>,
    constants: Vec<Value>,
    stack: Vec<Value>,
    globals: Vec<Value>,
    pc: usize,
}

impl<'a> VirtualMachine {
    pub fn new(raw_code: Vec<u8>) -> Self {
        let (bytecode, constants, var_count) = Self::deserialize(raw_code);
        Self {
            bytecode,
            constants,
            stack: Vec::new(),
            globals: vec![Value::Int(0); var_count],
            pc: 0,
        }
    }

    #[inline(always)]
    fn get_arg(&self) -> u16 {
        let low = self.bytecode[self.pc + 1] as u16;
        let high = self.bytecode[self.pc + 2] as u16;
        (high << 8) | low
    }

    fn deserialize(data: Vec<u8>) -> (Vec<u8>, Vec<Value>, usize) {
        // ptr to vec
        let mut pos = 0;

        // amount of number constants
        let const_count = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
        pos += 2;

        let mut constants = Vec::with_capacity(const_count);
        for _ in 0..const_count {
            let tag = data[pos];
            pos += 1;

            match tag {
                0x01 => {
                    // Int
                    let bytes: [u8; 8] = data[pos..pos + 8].try_into().unwrap();
                    constants.push(Value::Int(i64::from_le_bytes(bytes)));
                    pos += 8;
                }
                0x02 => {
                    // Float
                    let bytes: [u8; 8] = data[pos..pos + 8].try_into().unwrap();
                    constants.push(Value::Float(f64::from_le_bytes(bytes)));
                    pos += 8;
                }
                0x03 => {
                    // Bool
                    let val = data[pos] != 0;
                    constants.push(Value::Bool(val));
                    pos += 1;
                }
                0x04 => {
                    // Text (String)
                    let length = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
                    pos += 2;

                    let bytes = &data[pos..pos + length];
                    pos += length;

                    let string = String::from_utf8(bytes.to_vec())
                        .unwrap_or_else(|_| "Error in string converting".to_string());
                    constants.push(Value::Text(string));
                }
                _ => panic!(
                    "VM Critical Error: Unknown constant tag 0x{:02X} at pos {}",
                    tag,
                    pos - 1
                ),
            }
        }

        let var_count = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
        pos += 2;

        let bytecode = data[pos..].to_vec();
        (bytecode, constants, var_count)
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
                    self.stack.push(self.constants[arg as usize].clone());
                    self.pc += 3;
                }

                // 0x02: LOAD_VAR - Pushes the value of a global variable onto the stack from the globals array using a 2-byte index arg
                0x02 => {
                    let arg = self.get_arg();
                    self.stack.push(self.globals[arg as usize].clone());
                    self.pc += 3;
                }

                // 0x04: STORE_VAR - Pops the top value from the stack and stores it in the globals array at the 2-byte index arg
                0x03 => {
                    let arg = self.get_arg();
                    let value = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow during StoreVar".to_string())?;
                    self.globals[arg as usize] = value;
                    self.pc += 3;
                }

                // 0x04 IAdd
                0x04 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IADD (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IADD (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Int(val_a + val_b));
                    } else {
                        return Err("VM Runtime Error: Expected Int on stack for IADD".to_string());
                    }
                    self.pc += 1;
                }

                // 0x05 ISub
                0x05 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in ISUB (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in ISUB (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Int(val_a - val_b));
                    } else {
                        return Err("VM Runtime Error: Expected Int on stack for ISUB".to_string());
                    }
                    self.pc += 1;
                }

                // 0x06 IMul
                0x06 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IMUL (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IMUL (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Int(val_a * val_b));
                    } else {
                        return Err("VM Runtime Error: Expected Int on stack for IMUL".to_string());
                    }
                    self.pc += 1;
                }

                // 0x07 IDiv
                0x07 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IDIV (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IDIV (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        if val_b == 0 {
                            return Err("VM Runtime Error: Division by zero".to_string());
                        }
                        self.stack.push(Value::Int(val_a / val_b));
                    } else {
                        return Err("VM Runtime Error: Expected Int on stack for IDIV".to_string());
                    }
                    self.pc += 1;
                }

                // 0x08 IMod
                0x08 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IMOD (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IMOD (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        if val_b == 0 {
                            return Err("VM Runtime Error: Division by zero in modulo".to_string());
                        }
                        self.stack.push(Value::Int(val_a % val_b));
                    } else {
                        return Err("VM Runtime Error: Expected Int on stack for IMOD".to_string());
                    }
                    self.pc += 1;
                }

                // 0x09 IPow
                0x09 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IPOW (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IPOW (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        // Кастим степень в u32, так как pow() в Rust требует u32 для целых чисел
                        self.stack.push(Value::Int(val_a.pow(val_b as u32)));
                    } else {
                        return Err("VM Runtime Error: Expected Int on stack for IPOW".to_string());
                    }
                    self.pc += 1;
                }

                // 0x0A INegate (Унарный минус)
                0x0A => {
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in INEGATE".to_string())?;
                    if let Value::Int(val_a) = a {
                        self.stack.push(Value::Int(-val_a));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Int on stack for INEGATE".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x0B FAdd
                0x0B => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FADD (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FADD (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Float(val_a + val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FADD".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x0C FSub
                0x0C => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FSUB (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FSUB (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Float(val_a - val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FSUB".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x0D FMul
                0x0D => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FMUL (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FMUL (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Float(val_a * val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FMUL".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x0E FDiv
                0x0E => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FDIV (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FDIV (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Float(val_a / val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FDIV".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x0F FMod
                0x0F => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FMOD (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FMOD (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Float(val_a % val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FMOD".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x10 FPow
                0x10 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FPOW (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FPOW (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Float(val_a.powf(val_b))); // powf для f64
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FPOW".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x11 FNegate (Унарный минус)
                0x11 => {
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FNEGATE".to_string())?;
                    if let Value::Float(val_a) = a {
                        self.stack.push(Value::Float(-val_a));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FNEGATE".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x12 IEqual (==)
                0x12 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IEQUAL (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IEQUAL (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a == val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Int on stack for IEQUAL".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x13 INotEqual (!=)
                0x13 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in INOTEQUAL (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in INOTEQUAL (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a != val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Int on stack for INOTEQUAL".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x14 ILess (<)
                0x14 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in ILESS (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in ILESS (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a < val_b));
                    } else {
                        return Err("VM Runtime Error: Expected Int on stack for ILESS".to_string());
                    }
                    self.pc += 1;
                }

                // 0x15 ILessEq (<=)
                0x15 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in ILESSEQ (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in ILESSEQ (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a <= val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Int on stack for ILESSEQ".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x16 IGreater (>)
                0x16 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IGREATER (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IGREATER (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a > val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Int on stack for IGREATER".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x17 IGreaterEq (>=)
                0x17 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IGREATEREQ (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in IGREATEREQ (A)".to_string())?;
                    if let (Value::Int(val_a), Value::Int(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a >= val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Int on stack for IGREATEREQ".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x18 FEqual (==)
                0x18 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FEQUAL (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FEQUAL (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a == val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FEQUAL".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x19 FNotEqual (!=)
                0x19 => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FNOTEQUAL (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FNOTEQUAL (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a != val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FNOTEQUAL".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x1A FLess (<)
                0x1A => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FLESS (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FLESS (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a < val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FLESS".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x1B FLessEq (<=)
                0x1B => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FLESSEQ (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FLESSEQ (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a <= val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FLESSEQ".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x1C FGreater (>)
                0x1C => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FGREATER (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FGREATER (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a > val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FGREATER".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x1D FGreaterEq (>=)
                0x1D => {
                    let b = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FGREATEREQ (B)".to_string())?;
                    let a = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in FGREATEREQ (A)".to_string())?;
                    if let (Value::Float(val_a), Value::Float(val_b)) = (a, b) {
                        self.stack.push(Value::Bool(val_a >= val_b));
                    } else {
                        return Err(
                            "VM Runtime Error: Expected Float on stack for FGREATEREQ".to_string()
                        );
                    }
                    self.pc += 1;
                }

                // 0x1E: JUMP - Unconditionally updates the PC to the 2-byte absolute address arg
                0x1E => {
                    let arg = self.get_arg();
                    self.pc = arg as usize;
                }

                // 0x1F: JUMP_IF_FALSE - Pops a condition value; if it's 0 (false), jumps to the 2-byte address arg, otherwise skips the argument (+3 bytes)
                0x1F => {
                    let arg = self.get_arg();
                    let condition = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in JumpIfFalse".to_string())?;
                    if let Value::Bool(b) = condition {
                        if !b {
                            self.pc = arg as usize;
                        } else {
                            self.pc += 3;
                        }
                    } else {
                        return Err("Expected Bool for JumpIfFalse".to_string());
                    }
                }
                // 0x2D - Print
                0x2D => {
                    let value = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Runtime Error: Nothing to print".to_string())?;

                    match value {
                        Value::Int(n) => println!("{}", n),
                        Value::Float(f) => println!("{}", f),
                        Value::Bool(b) => println!("{}", b),
                        Value::Text(t) => println!("{}", t),
                    }

                    self.pc += 1;
                }

                // 0x16: INPUT - Reads an integer line from stdin and saves it directly into the globals array at the 2-byte slot index arg
                0x2E => {
                    let arg = self.get_arg();
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .map_err(|e| e.to_string())?;
                    let trimmed = input.trim();
                    if let Ok(num) = trimmed.parse::<i64>() {
                        self.globals[arg as usize] = Value::Int(num);
                    } else if let Ok(num) = trimmed.parse::<f64>() {
                        self.globals[arg as usize] = Value::Float(num);
                    } else if trimmed == "true" || trimmed == "false" {
                        self.globals[arg as usize] = Value::Bool(trimmed == "true");
                    } else {
                        self.globals[arg as usize] = Value::Text(trimmed.to_string());
                    }
                    self.pc += 3;
                }

                // 0x2A And
                0x2A => {
                    let cond_1 = self.stack.pop().ok_or_else(|| {
                        "VM Error: Stack underflow in AND block first condition".to_string()
                    })?;
                    let cond_2 = self.stack.pop().ok_or_else(|| {
                        "VM Error: Stack underflow in AND block second condition".to_string()
                    })?;
                    if let (Value::Bool(cd_1), Value::Bool(cd_2)) = (cond_1, cond_2) {
                        self.stack.push(Value::Bool(cd_1 && cd_2));
                    } else {
                        return Err("VM Runtime Error: Expected Boolean values on stack for AND"
                            .to_string());
                    }
                    self.pc += 1
                }

                // 0x2B Or
                0x2B => {
                    let cond_1 = self.stack.pop().ok_or_else(|| {
                        "VM Error: Stack underflow in AND block first condition".to_string()
                    })?;
                    let cond_2 = self.stack.pop().ok_or_else(|| {
                        "VM Error: Stack underflow in AND block second condition".to_string()
                    })?;
                    if let (Value::Bool(cd_1), Value::Bool(cd_2)) = (cond_1, cond_2) {
                        self.stack.push(Value::Bool(cd_1 || cd_2));
                    } else {
                        return Err("VM Runtime Error: Expected Boolean values on stack for AND"
                            .to_string());
                    }
                    self.pc += 1
                }

                // 0x2C Not
                0x2C => {
                    let cond = self
                        .stack
                        .pop()
                        .ok_or_else(|| "VM Error: Stack underflow in NOT block".to_string())?;
                    let result = match cond {
                        Value::Bool(b) => Value::Bool(!b),
                        Value::Int(0) => Value::Bool(true), // 0 → false → NOT → true
                        Value::Int(_) => Value::Bool(false), // любое ненулевое → true → NOT → false
                        _ => {
                            return Err("VM Runtime Error: Expected Bool or Int for NOT".to_string());
                        }
                    };
                    self.stack.push(result);
                    self.pc += 1
                }

                // Fallback for handling compiled bytecode corruption or parser bugs
                _ => {
                    return Err(format!(
                        "Unknown opcode: 0x{:02X} at PC: {}",
                        opcode, self.pc
                    ));
                }
            }
        }
        Ok(())
    }
}
