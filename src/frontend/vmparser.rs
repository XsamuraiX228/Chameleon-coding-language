use crate::frontend::lexer::SpannedToken;
use crate::frontend::token::KeyWordType;
use crate::diagnostic::diagnostic::{ErrorHandler, ErrorKind};
use crate::frontend::token::Token;
use super::token::{CmpOp, OpType, Literal};
use crate::dialect::SyntaxDict;
use std::vec::IntoIter;
use std::iter::Peekable;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    // === Syste, ===
    Stop        = 0x00,

    // === Memory ===
    LoadConst   = 0x01,
    LoadVar     = 0x02,
    StoreVar    = 0x03,

    // === Math ===
    Add         = 0x04,
    Sub         = 0x05,
    Mul         = 0x06,
    Div         = 0x07,
    Mod         = 0x08,
    Pow         = 0x09,

    // === Unary Operations ===
    Negate      = 0x0A, 

    // === Compare operators ===
    Equal       = 0x0B,
    NotEqual    = 0x0C,
    Greater     = 0x0D,
    Less        = 0x0E,
    GreaterEq   = 0x0F,
    LessEq      = 0x10,

    // === Opcodes which use for controlling the programm ===
    Jump        = 0x11,
    JumpIfFalse = 0x12,

    // === Input/Output ===
    Input       = 0x13,
    Print       = 0x14,

    // === Superopcodes for optimizations ===
    IncVar      = 0x15,
    MovVar      = 0x16,
    JVLC        = 0x17,
    JVGC        = 0x18,
}

pub struct ByteParser<'a> {
    lexer: Peekable<IntoIter<SpannedToken<'a>>>,
    bytecode: Vec<u16>,
    pub constants: Vec<i64>,       
    pub variables: Vec<&'a str>,
    current_line: usize,
    dialect: &'a SyntaxDict
}

impl<'a> ByteParser<'a>  {
    pub fn new(tokens: Vec<SpannedToken<'a>>, dialect: &'a SyntaxDict ) -> Self {
        Self {
            lexer: tokens.into_iter().peekable(), 
            bytecode: Vec::new(), 
            constants: Vec::new(), 
            variables: Vec::new(), 
            current_line: 1,
            dialect
        }
    }

    pub fn peek_token(&mut self) -> Option<&Token<'a>> {
        self.lexer.peek().map(|spanned| &spanned.token)
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        let next_token = self.lexer.next()?;
        self.current_line = next_token.line; 
        Some(next_token.token)
    }

    fn add_constant(&mut self, value: i64) -> u8 {        
        if let Some(index) = self.constants.iter().position(|&c| c == value) {
            return index as u8;
        }
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }

    fn add_variable(&mut self, value: &'a str) -> u8 {
        if let Some(index) = self.variables.iter().position(|&c| c == value) {
            return index as u8;
        }
        self.variables.push(value);
        (self.variables.len() - 1) as u8
    }

    fn get_num(&mut self) -> Result<i64, ErrorHandler> {    
        match self.next_token() {
            Some(Token::Literal(Literal::Number(num))) => Ok(num),
            Some(Token::OpType(OpType::Minus)) => {
                match self.next_token() {
                    Some(Token::Literal(Literal::Number(num))) => Ok(-num),
                    _ => Err(self.easy_error("Expected number after '-'".to_string())),
                }
            }
            _ => Err(self.easy_error("Expected number".to_string())),
        }
    }

    fn get_name(&mut self) -> Result<&'a str, ErrorHandler> {
        match self.next_token() {
            Some(Token::Literal(Literal::Ident(name))) => Ok(name),
            _ => Err(self.easy_error("Expected variable name".to_string())),
        }
    }

    fn hard_error(&self, error_kw: String, kw_type: String) -> ErrorHandler{
        let message = format!("Expected '{}' after {} block", error_kw, kw_type);
        ErrorHandler::new(ErrorKind::Syntax, message, self.current_line)
    }

    fn easy_error(&self, context: String) -> ErrorHandler {
        ErrorHandler::new(ErrorKind::Syntax, context, self.current_line)
    }

    fn expect(&mut self, expected: Token<'a>) -> Result<(), ErrorHandler> {
        if let Some(token) = self.next_token() {
            if token == expected {
                return Ok(());
            }
        }
        Err(self.easy_error(format!("Expected {:?}", expected)))
    }

    fn hard_expect(&mut self, expected: KeyWordType, context: KeyWordType) -> Result<(), ErrorHandler> {
        let curr_token = self.next_token();
        if curr_token == Some(Token::KeyWord(expected)) {
            return Ok(());
        }
        let error_kw_str = self.dialect.get_kw_word(expected);
        let context_kw_str = self.dialect.get_kw_word(context);
        Err(self.hard_error(error_kw_str, context_kw_str))
    }

    pub fn debug(bytecode: &Vec<u8>) {
        println!("=== BYTECODE DUMP (8-bit packed) ===");
        for (index, value) in bytecode.iter().enumerate() {
            println!("{:3}: {:02X}", index, value);
            if (index + 1) % 4 == 0 { println!(); }
        }
        if bytecode.len() % 4 != 0 { println!(); }
        println!("==============================");
    }

    pub fn optimize_and_map_addresses(bytecode: &[u16]) -> (Vec<u16>, Vec<usize>) {
        let mut optimized = Vec::with_capacity(bytecode.len());
        
        // Addres map
        // We use this map to set our old_idx to new byte_idx after optimisation for JumpIfFalse and Jump Opcodes
        let mut addr_map: Vec<usize> = vec![0; bytecode.len()];
        
        let mut old_idx = 0; // idx in bytecode
        let mut byte_idx = 0; // new_idx after bytecode will be optimized

        while old_idx < bytecode.len() {
            // Every JumpIfFalse and Jump now move to new byte_idx
            addr_map[old_idx] = byte_idx;

            // === 1. 7 opcodes pattern (IncVar) ===
            // The goal of IncVar is to reduce the part of the code
            // where everytime the same while is increase
            // e.g WHILE I < 100 THEN LET I = I + 1 
            if old_idx + 6 < bytecode.len() 
                && bytecode[old_idx] == OpCode::LoadVar as u16 
                && bytecode[old_idx + 2] == OpCode::LoadConst as u16
                && bytecode[old_idx + 4] == OpCode::Add as u16
                && bytecode[old_idx + 5] == OpCode::StoreVar as u16
                && bytecode[old_idx + 1] == bytecode[old_idx + 6] 
            {
                
                for offset in 1..7 {
                    if old_idx + offset < bytecode.len() {
                        addr_map[old_idx + offset] = byte_idx;
                    }
                }

                optimized.push(OpCode::IncVar as u16);
                optimized.push(bytecode[old_idx + 1]); // var_idx
                optimized.push(bytecode[old_idx + 3]); // const_idx

                // Calculate the size of IncVar in u8:
                // 1 byte for op (IncVar) + 2 bytes for var_idx + 2 bytes for const_idx = 5 bytes
                byte_idx += 5; 
                old_idx += 7;
                continue;
            }

            // === 2. 4 opcodes pattern (MovVar) ===
            // The goal of MovVar is to reduce the part of the code
            // where where a variable value is copied into another variable
            // e.g LET X = 5 LET Y = X
            if old_idx + 3 < bytecode.len()
                && bytecode[old_idx] == OpCode::LoadVar as u16
                && bytecode[old_idx + 2] == OpCode::StoreVar as u16
            {
                for offset in 1..4 {
                    if old_idx + offset < bytecode.len() {
                        addr_map[old_idx + offset] = byte_idx;
                    }
                }

                optimized.push(OpCode::MovVar as u16);
                optimized.push(bytecode[old_idx + 3]); // To (s_idx)
                optimized.push(bytecode[old_idx + 1]); // From (l_idx)

                // Calculate the size of MovVar in u8:
                // 1 byte for op (MovVar) + 2 bytes for To arg + 2 bytes for From arg = 5
                byte_idx += 5;
                old_idx += 4;
                continue;
            }

            // === 3. Simple instructions ===
            let op = bytecode[old_idx];
            optimized.push(op);
            // We match op to know how many bytes it's weight
            match op {
                // Wihtout any args 1 byte
                0x00 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0A | 
                0x0B | 0x0C | 0x0D | 0x0E | 0x0F | 0x10 | 0x14 => {
                    byte_idx += 1;
                    old_idx += 1;
                }
                // 1 op + 1 arg = 3 bytes
                0x01 | 0x02 | 0x03 | 0x11 | 0x12 | 0x13 => {
                    if old_idx + 1 < bytecode.len() {
                        addr_map[old_idx + 1] = byte_idx;
                    }
                    
                    if old_idx + 1 < bytecode.len() {
                        optimized.push(bytecode[old_idx + 1]);
                    }

                    byte_idx += 3;
                    old_idx += 2;
                }
                _ => {
                    byte_idx += 1;
                    old_idx += 1;
                }
            }
        }

        (optimized, addr_map)
    }

    // This function change points where JumpIfFalse and Jump would jump in bytecode, but not in optimized one
    pub fn patch_addresses(mut optimized: Vec<u16>, addr_map: &[usize]) -> Vec<u16> {
        let mut i = 0;
        while i < optimized.len() {
            let op = optimized[i];
            
            match op {
                // Simple jumps, in u16 the addres locate in the second part, so we do i + 1
                0x11 | 0x12 => {
                    let old_addr = optimized[i + 1] as usize;
                    optimized[i + 1] = addr_map[old_addr] as u16; 
                    i += 2;
                }
                // For super opcodes JVLC / JVGC: addres locate in the third part  (i + 3)
                0x17 | 0x18 => {
                    let old_addr = optimized[i + 3] as usize;
                    optimized[i + 3] = addr_map[old_addr] as u16;
                    i += 4;
                }
                // Simple instructions skipped, because they don't jump
                0x01 | 0x02 | 0x03 | 0x13 => i += 2,
                // Super opcodes which don't jump with 2 args
                0x15 | 0x16 => i += 3,
                // Other without any args
                _ => i += 1,
            }
        }
        optimized
    }

    pub fn finalize_to_u8_simple(patched_bytecode: &[u16]) -> Vec<u8> {
        let mut bin = Vec::with_capacity(patched_bytecode.len());
        let mut i = 0;

        while i < patched_bytecode.len() {
            let op = patched_bytecode[i];
            bin.push(op as u8); 
            i += 1;

            match op {
                // 0 args
                0x00 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0A | 
                0x0B | 0x0C | 0x0D | 0x0E | 0x0F | 0x10 | 0x14 => {}

                // 1 arg
                0x01 | 0x02 | 0x03 | 0x11 | 0x12 | 0x13 => {
                    let bytes = patched_bytecode[i].to_le_bytes();
                    bin.push(bytes[0]); bin.push(bytes[1]);
                    i += 1;
                }

                // 2 args
                0x15 | 0x16 => {
                    for _ in 0..2 {
                        let bytes = patched_bytecode[i].to_le_bytes();
                        bin.push(bytes[0]); bin.push(bytes[1]);
                        i += 1;
                    }
                }

                // 3 args
                0x17 | 0x18 => {
                    for _ in 0..3 {
                        let bytes = patched_bytecode[i].to_le_bytes();
                        bin.push(bytes[0]); bin.push(bytes[1]);
                        i += 1;
                    }
                }
                _ => {}
            }
        }
        bin
    }

    pub fn byteparse(&mut self) -> Result<Vec<u16>, ErrorHandler> {
        while let Some(_) = self.peek_token() {
            self.byteparse_block()?;
        }
        self.bytecode.push(OpCode::Stop as u16);
        Ok(self.bytecode.clone())
    }

    pub fn byteparse_block(&mut self) -> Result<(), ErrorHandler> {
        match self.peek_token() {
            Some(Token::KeyWord(KeyWordType::Print)) => self.parse_print(),
            Some(Token::KeyWord(KeyWordType::Let)) => self.parse_let(),
            Some(Token::KeyWord(KeyWordType::Input)) => self.parse_input(),
            Some(Token::KeyWord(KeyWordType::If)) => self.parse_if(),
            Some(Token::KeyWord(KeyWordType::While)) => self.parse_while(),
            Some(Token::KeyWord(KeyWordType::For)) => self.parse_for(),
            Some(Token::Newline) => {
                self.next_token();
                Ok(())
            }
            Some(Token::EOF) => {
                self.next_token();
                Ok(())
            }
            None => Ok(()),
            _ => Err(self.easy_error("Unexpected token".to_string()))
        }
    }

    fn parse_let(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var_name = self.get_name()?;
        self.expect(Token::CmpOp(CmpOp::Equal))?;
        self.expr_bp(0)?;
        let var_id = self.add_variable(var_name);
        self.bytecode.push(OpCode::StoreVar as u16);
        self.bytecode.push(var_id as u16);
        Ok(())
    }

    fn parse_input(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var = self.get_name()?;
        let var_id = self.add_variable(var);
        self.bytecode.push(OpCode::Input as u16);
        self.bytecode.push(var_id as u16);
        Ok(()) 
    }

    fn parse_print(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        self.expr_bp(0)?;
        self.bytecode.push(OpCode::Print as u16);
        Ok(())
    }

    fn parse_if(&mut self) -> Result<(), ErrorHandler> {
        // Skip IF and parse the expression (we will get 1.0 or 0.0)
        self.next_token(); 
        self.expr_bp(0)?;  
        self.hard_expect(KeyWordType::Then, KeyWordType::If)?;
        
        // We push JumpIfFalse which will immediatly bring our self.expr_bp(0)? and check the condition
        self.bytecode.push(OpCode::JumpIfFalse as u16);

        // We store the idx where the loop is started to return here again
        let jump_patch = self.bytecode.len();
        // Push a temporary placeholder (0). The real destination address will overwrite this later.
        self.bytecode.push(0); 

        // Parse until we meet ELSE or END
        while let Some(token) = self.peek_token() {
            match token {
                Token::KeyWord(KeyWordType::Else) => break,
                Token::KeyWord(KeyWordType::End) => break,
                _ => { self.byteparse_block()?; }
            }
        }

        // If we met ELSE
        if let Some(Token::KeyWord(KeyWordType::Else)) = self.peek_token() {
            self.next_token(); // Consume ELSE
            
            // If the THEN block executed successfully, the VM must skip the ELSE block.
            // So we emit an unconditional Jump at the very end of the THEN block.
            self.bytecode.push(OpCode::Jump as u16);
            let else_jump_patch = self.bytecode.len();
            self.bytecode.push(0);
            
            // If the IF condition was false, the VM must jump HERE (the beginning of the ELSE block).
            // The current bytecode length perfectly indicates the first instruction of the ELSE block.
            let jump_target = self.bytecode.len();
            self.bytecode[jump_patch] = jump_target as u16; // Overwrite the conditional jump placeholder.
            
            // Parse all instructions inside the ELSE block until we hit "END".
            while let Some(token) = self.peek_token() {
                if let Token::KeyWord(KeyWordType::End) = token { break; }
                self.byteparse_block()?;
            }
            
            // !!! ИСПРАВЛЕНИЕ №2 !!!
            // End of the entire IF-ELSE structure. This is where the VM should land after
            // executing the THEN block to safely bypass the ELSE code.
            let else_target = self.bytecode.len();
            self.bytecode[else_jump_patch] = else_target as u16;
            
        } else {
            // If the condition is false, we jump straight to the exit (right after the END token).
            // Get the current index and overwrite our very first placeholder.
            let jump_target = self.bytecode.len();
            self.bytecode[jump_patch] = jump_target as u16;
        }
        
        self.hard_expect(KeyWordType::End, KeyWordType::If)?;
        Ok(())
    }

    fn parse_while(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();

        let start_loop = self.bytecode.len() as u16;
        self.expr_bp(0)?;
        self.hard_expect(KeyWordType::Then, KeyWordType::While)?;

        self.bytecode.push(OpCode::JumpIfFalse as u16);
        let end_patch = self.bytecode.len();
        self.bytecode.push(0);

        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Wend) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }

        self.bytecode.push(OpCode::Jump as u16);
        self.bytecode.push(start_loop);

        let end_loop = self.bytecode.len() as u16;
        self.bytecode[end_patch] = end_loop;
        
        Ok(())
    }

    fn parse_for(&mut self) -> Result<(), ErrorHandler> {
        self.next_token(); //  Consum FOR

        // Load the var token
        let var = self.get_name()?;
        self.expect(Token::CmpOp(CmpOp::Equal))?;
        self.expr_bp(0)?;
        
        // Store var in stack
        let var_idx = self.add_variable(var);
        self.bytecode.push(OpCode::StoreVar as u16);
        self.bytecode.push(var_idx as u16);
        
        self.hard_expect(KeyWordType::To, KeyWordType::For)?;
        let num = self.get_num()?;
        let const_idx = self.add_constant(num);
        
        // Get step value
        let mut step_value = 1;
        if let Some(Token::KeyWord(KeyWordType::Step)) = self.peek_token() {
            self.next_token();
            step_value = self.get_num()?;
        }
        let step_idx = self.add_constant(step_value);
        // From this line the loop starts
        let loop_start = self.bytecode.len();

        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Next) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }

        self.bytecode.push(OpCode::IncVar as u16); 
        self.bytecode.push(var_idx as u16);               
        self.bytecode.push(step_idx as u16);
        
        if step_value > 0 {
            self.bytecode.push(OpCode::JVLC as u16);
            self.bytecode.push(var_idx as u16);
            self.bytecode.push(const_idx as u16);
            self.bytecode.push(loop_start as u16);
        } else {
            self.bytecode.push(OpCode::JVGC as u16);
            self.bytecode.push(var_idx as u16);
            self.bytecode.push(const_idx as u16);
            self.bytecode.push(loop_start as u16);
        }
        Ok(())
    }

    pub fn expr_bp(&mut self, min_bp: u16) -> Result<(), ErrorHandler> {
        match self.next_token() {
            Some(Token::Literal(Literal::Number(num))) => {
                let idx = self.add_constant(num);
                self.bytecode.push(OpCode::LoadConst as u16);
                self.bytecode.push(idx as u16);
            },
            Some(Token::OpType(OpType::LParen)) => {
                self.expr_bp(0)?;
                if let Some(token) = self.next_token() {
                    if token != Token::OpType(OpType::RParen) {
                        return Err(self.easy_error("Expected matching ')'".to_string()));
                    }
                }
            }
            Some(Token::OpType(op_type)) => {
                match op_type {
                    OpType::Plus | OpType::Minus => {
                        let prefix_bp = self.prefix_bind_operator(op_type)
                            .map_err(|e| self.easy_error(e.to_string()))?;
                        self.expr_bp(prefix_bp as u16)?;
                        if op_type == OpType::Minus {
                            self.bytecode.push(OpCode::Negate as u16);
                        }
                    }
                    _ => return Err(self.easy_error("Unexpected operator".to_string())),
                }
            }
            Some(Token::Literal(Literal::Ident(name))) => {
                let idx = self.add_variable(name);
                self.bytecode.push(OpCode::LoadVar as u16);
                self.bytecode.push(idx as u16);
            },
            Some(Token::Literal(Literal::Text(_))) => {
                return Err(self.easy_error("Strings not allowed in expressions".to_string()));
            }
            _ => return Err(self.easy_error("Expected number or variable".to_string())),
        };

        while let Some(next_t) = self.peek_token() {
            let (l_bp, r_bp) = match next_t {
                Token::CmpOp(_) => (0, 1),
                Token::OpType(op_type) => {
                    match op_type {
                        OpType::Plus | OpType::Minus => (3, 4),
                        OpType::Multiply | OpType::Divide | OpType::Mod => (5, 6),
                        OpType::Power => (9, 8),
                        _ => break,
                    }
                },
                _ => break,
            };

            if l_bp < min_bp as u16 { break; }

            let op_token = self.next_token().unwrap();

            self.expr_bp(r_bp as u16)?;
            match op_token {
                Token::OpType(OpType::Plus) => self.bytecode.push(OpCode::Add as u16),
                Token::OpType(OpType::Minus) => self.bytecode.push(OpCode::Sub as u16),
                Token::OpType(OpType::Multiply) => self.bytecode.push(OpCode::Mul as u16),
                Token::OpType(OpType::Divide) => self.bytecode.push(OpCode::Div as u16),
                Token::OpType(OpType::Mod) => self.bytecode.push(OpCode::Mod as u16),
                Token::OpType(OpType::Power) => self.bytecode.push(OpCode::Pow as u16),
                Token::CmpOp(CmpOp::DoubleEqual) => self.bytecode.push(OpCode::Equal as u16),
                Token::CmpOp(CmpOp::Less) => self.bytecode.push(OpCode::Less as u16),
                Token::CmpOp(CmpOp::Greater) => self.bytecode.push(OpCode::Greater as u16),
                Token::CmpOp(CmpOp::NonEqual) => self.bytecode.push(OpCode::NotEqual as u16),
                Token::CmpOp(CmpOp::GreaterEqual) => self.bytecode.push(OpCode::GreaterEq as u16),
                Token::CmpOp(CmpOp::LessEqual) => self.bytecode.push(OpCode::LessEq as u16),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn prefix_bind_operator(&self, op_type: OpType) -> Result<u8, &'static str> {
        match op_type {
            OpType::Plus | OpType::Minus => Ok(5),
            _ => Err("Wrong prefix operator"),
        }
    }
}
