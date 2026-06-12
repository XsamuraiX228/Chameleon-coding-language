use crate::frontend::lexer::SpannedToken;
use crate::frontend::token::{KeyWordType};
use crate::diagnostic::diagnostic::{ErrorHandler, ErrorKind};
use crate::frontend::token::Token;
use super::token::{CmpOp, OpType, Literal};
use crate::dialect::SyntaxDict;
use std::vec::IntoIter;
use std::iter::Peekable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Number,
    Text,
}

pub struct StringInfo<'a> {
    name: &'a str,
    data_type: DataType,
}

pub struct Bparser<'a> {
    lexer: Peekable<IntoIter<SpannedToken<'a>>>,
    bytecode: Vec<u8>,
    constants: Vec<i64>,       
    variables: Vec<StringInfo<'a>>,
    string_pool: Vec<String>,
    current_line: usize,
    dialect: &'a SyntaxDict
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Memory
    LoadConst  = 0x01,  // const <- stack
    LoadVar    = 0x02,  // var <- stack
    LoadString = 0x03,
    StoreVar   = 0x04,  // var -> stack

    // Math 
    Add        = 0x05,
    Sub        = 0x06,
    Mul        = 0x07,
    Div        = 0x08,
    Mod        = 0x09,
    Pow        = 0x0A,
    Negate     = 0x0B,

    // Compare 
    Equal      = 0x0C,
    NotEqual   = 0x0D,
    Less       = 0x0E,
    LessEq     = 0x0F,
    Greater    = 0x10,
    GreaterEq  = 0x11,

    // Control flow
    Jump          = 0x12,
    JumpIfFalse   = 0x13,

    // IO
    PrintNum     = 0x14,
    PrintStr     = 0x15,
    Input     = 0x16,

    // Bool tokens
    And       = 0x17,
    Or        = 0x18,

    // Stop
    Stop       = 0x00,
}

impl<'a> Bparser<'a>  {
    pub fn new(tokens: Vec<SpannedToken<'a>>, dialect: &'a SyntaxDict ) -> Self {
        Self {
            lexer: tokens.into_iter().peekable(), 
            bytecode: Vec::new(), 
            constants: Vec::new(), 
            variables: Vec::new(), 
            string_pool: Vec::new(),
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

    fn add_constant(&mut self, value: i64) -> u16 {        
        if let Some(index) = self.constants.iter().position(|&c| c == value) {
            return index as u16;
        }
        self.constants.push(value);
        (self.constants.len() - 1) as u16
    }

    fn add_variable(&mut self, value: &'a str, data_type: DataType) -> u16 {
        if let Some(index) = self.variables.iter().position(|c| c.name == value) {
            self.variables[index].data_type = data_type;
            return index as u16;
        }
        self.variables.push(StringInfo { name: value, data_type });
        (self.variables.len() - 1) as u16
    }

    fn find_variable(&self, name: &str) -> Result<u16, ErrorHandler> {
        if let Some(index) = self.variables.iter().position(|v| v.name == name) {
            Ok(index as u16)
        } else {
            Err(self.easy_error(format!("Undeclared variable: '{}'", name)))
        }
    }

    fn get_var_type(&self, var_id: u16) -> DataType {
        self.variables[var_id as usize].data_type
    }

    fn add_string_const(&mut self, value: String) -> u16 {
        if let Some(index) = self.string_pool.iter().position(|s| *s == value) {
            return index as u16;
        }
        self.string_pool.push(value.to_string());
        (self.string_pool.len() - 1) as u16
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

    fn to_u8_with_args(&mut self, opcode: u8, arg: u16) {
        self.bytecode.push(opcode);
        self.bytecode.push((arg & 0xFF) as u8);
        self.bytecode.push(((arg >> 8) & 0xFF) as u8);
    } 

    fn to_u8(&mut self, opcode: u8) {
        self.bytecode.push(opcode);
    }

    pub fn debug_dump(&self) {
        println!("\n=== PARSER DEBUG INFO ===");
        
        // Константы
        println!("\n[Constants] ({} items):", self.constants.len());
        for (i, &c) in self.constants.iter().enumerate() {
            println!("  [{}] = {}", i, c);
        }
        
        // Переменные
        println!("\n[Variables] ({} items):", self.variables.len());
        for (i, v) in self.variables.iter().enumerate() {
            println!("  [{}] = {}", i, v.name);
        }
        
        // String pool
        println!("\n[String Pool] ({} items):", self.string_pool.len());
        for (i, s) in self.string_pool.iter().enumerate() {
            println!("  [{}] = \"{}\"", i, s);
        }
        
        // Байткод (сырой)
        println!("\n[Raw Bytecode] ({} bytes):", self.bytecode.len());
        for (i, &b) in self.bytecode.iter().enumerate() {
            print!("{:02X} ", b);
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
        println!();
        
        println!("=======================\n");
    }


    fn serialized(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        // Amount of number constants
        output.push((self.constants.len() & 0xFF) as u8);
        output.push((self.constants.len() >> 8) as u8);

        // Constants (i64)
        for &c in &self.constants {
            output.extend_from_slice(&c.to_le_bytes());
        }

        // Amount of variables
        output.push((self.variables.len() & 0xFF) as u8);
        output.push((self.variables.len() >> 8) as u8);

        // Amout of string constansts
        output.push((self.string_pool.len() & 0xFF) as u8);
        output.push((self.string_pool.len() >> 8) as u8);

        for text in self.string_pool.iter()  {
            let text_bytes = text.as_bytes();
            let text_len = text_bytes.len() as u16;

            output.extend_from_slice(&text_len.to_le_bytes());
            output.extend_from_slice(text_bytes);
        }

        output.extend_from_slice(&self.bytecode);
        output
    }

    pub fn start_byteparsing(&mut self) -> Result<Vec<u8>, ErrorHandler> {
        while let Some(_) = self.peek_token() {
            self.byteparse_block()?;
        }
        self.to_u8(Opcode::Stop as u8);
        let b_code = self.serialized();
        Ok(b_code)
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
            Some(t) => {
                let token_clone = t.clone(); 
                Err(self.easy_error(format!("Unexpected token: {:?}", token_clone)))
            }
        }
    }

    fn parse_let(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var_name = self.get_name()?;
        self.expect(Token::CmpOp(CmpOp::Equal))?;
        let data_type = self.expr_bp(0)?;
        let var_id = self.add_variable(var_name, data_type);
        self.to_u8_with_args(Opcode::StoreVar as u8, var_id);
        Ok(())
    }

    fn parse_input(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var_name = self.get_name()?;
        let var_id = self.find_variable(var_name)?;
        self.to_u8_with_args(Opcode::Input as u8, var_id);
        Ok(())
    }

    fn parse_print(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let expr_type = self.expr_bp(0)?; 
        match expr_type {
            DataType::Number => self.to_u8(Opcode::PrintNum as u8),
            DataType::Text => self.to_u8(Opcode::PrintStr as u8),
        }
        Ok(()) 
    }

    fn patch_address(&mut self, instr_pos: usize, target_instruction_idx: u16) {
        let low_byte = (target_instruction_idx & 0xFF) as u8;
        let high_byte = ((target_instruction_idx >> 8) & 0xFF) as u8;
        
        
        self.bytecode[instr_pos + 1] = low_byte;
        self.bytecode[instr_pos + 2] = high_byte;
    }

    fn parse_if(&mut self) -> Result<(), ErrorHandler> {
        self.next_token(); // Skip If
        self.expr_bp(0)?; // Get result 1 or 0
        self.hard_expect(KeyWordType::Then, KeyWordType::If)?; // Check if THEN keyword was written

        let start_if_condition = self.bytecode.len();
        self.to_u8_with_args(Opcode::JumpIfFalse as u8, 0); // send a Check for conditional

        while let Some(token) = self.peek_token() {
            match token {
                Token::KeyWord(KeyWordType::Else) => break,
                Token::KeyWord(KeyWordType::End) => break,
                _ => { self.byteparse_block()?; }
            }
        }

        if let Some(Token::KeyWord(KeyWordType::Else)) = self.peek_token() {
            self.parse_else(start_if_condition)?;
        } else {
            self.hard_expect(KeyWordType::End, KeyWordType::If)?;
            let target = self.bytecode.len() as u16;
            self.patch_address(start_if_condition, target);
        }

        Ok(())
    }

    fn parse_else(&mut self,  jump_if_false_pos: usize) -> Result<(),  ErrorHandler> {
        self.next_token();
        let jump_pos = self.bytecode.len();
        self.to_u8_with_args(Opcode::Jump as u8, 0);

        let start_else_condition = self.bytecode.len() as u16;
        self.patch_address(jump_if_false_pos, start_else_condition);

        while let Some(token) = self.peek_token() {
            match token {
                Token::KeyWord(KeyWordType::End) => break,
                _ => { self.byteparse_block()?; }
            }
        }

        self.hard_expect(KeyWordType::End, KeyWordType::If)?;

        let end_idx = self.bytecode.len() as u16;
        self.patch_address(jump_pos, end_idx);
        Ok(())
    }

    fn parse_while(&mut self) -> Result<(),  ErrorHandler> {
        self.next_token(); // Skip WHILE
        let start_loop = self.bytecode.len() as u16;
        self.expr_bp(0)?;
        self.hard_expect(KeyWordType::Then, KeyWordType::While)?;

        let while_start = self.bytecode.len();
        self.to_u8_with_args(Opcode::JumpIfFalse as u8, 0);
        
        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Wend) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }

        self.to_u8_with_args(Opcode::Jump as u8, start_loop);
        
        let end_idx = self.bytecode.len() as u16;
        self.patch_address(while_start, end_idx);
        Ok(())
    }

    fn parse_for(&mut self) -> Result<(),  ErrorHandler> {
        self.next_token(); // Skip FOR

        // Read var name
        let var = self.get_name()?;
        let var_id = self.add_variable(var, DataType::Number);
        self.expect(Token::CmpOp(CmpOp::Equal))?;
        self.expr_bp(0)?;

        self.to_u8_with_args(Opcode::StoreVar as u8, var_id);
        self.hard_expect(KeyWordType::To, KeyWordType::For)?;

        let limit = self.get_num()?;
        let limit_id = self.add_constant(limit);

        let mut step_value = 1;
        if let Some(Token::KeyWord(KeyWordType::Step)) = self.peek_token() {
            self.next_token();
            step_value = self.get_num()?;
        }
        let step_idx = self.add_constant(step_value);

        let loop_start = self.bytecode.len() as u16;

        self.to_u8_with_args(Opcode::LoadVar as u8, var_id);
        self.to_u8_with_args(Opcode::LoadConst as u8, limit_id);

        if step_value > 0 {
            self.to_u8(Opcode::LessEq as u8);
        } else {
            self.to_u8(Opcode::GreaterEq as u8);
        }

        let for_jump_pos = self.bytecode.len();
        self.to_u8_with_args(Opcode::JumpIfFalse as u8, 0);

        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Next) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }

        self.to_u8_with_args(Opcode::LoadVar as u8, var_id);    
        self.to_u8_with_args(Opcode::LoadConst as u8, step_idx); 
        self.to_u8(Opcode::Add as u8);                           
        self.to_u8_with_args(Opcode::StoreVar as u8, var_id);   

        
        self.to_u8_with_args(Opcode::Jump as u8, loop_start);

        let end_idx = self.bytecode.len() as u16;
        self.patch_address(for_jump_pos, end_idx);


        Ok(())
    }


    pub fn expr_bp(&mut self, min_bp: u16) -> Result<DataType, ErrorHandler> {
        let current_type = match self.next_token() {
            Some(Token::Literal(Literal::Number(num))) => {
                let idx = self.add_constant(num);
                self.to_u8_with_args(Opcode::LoadConst as u8, idx);
                DataType::Number
            },
            Some(Token::OpType(OpType::LParen)) => {
                let inner_type = self.expr_bp(0)?;
                if let Some(token) = self.next_token() {
                    if token != Token::OpType(OpType::RParen) {
                        return Err(self.easy_error("Expected matching ')'".to_string()));
                    }
                }
                inner_type
            }
            Some(Token::KeyWord(KeyWordType::Not)) => {
                self.expr_bp(0)?;
                let zero_idx = self.add_constant(0);
                self.to_u8_with_args(Opcode::LoadConst as u8, zero_idx);
                self.to_u8(Opcode::Equal as u8);
                DataType::Number
            }
            Some(Token::OpType(op_type)) => {
                match op_type {
                    OpType::Plus | OpType::Minus => {
                        self.expr_bp(11)?;
                        if op_type == OpType::Minus {
                            self.to_u8(Opcode::Negate as u8);
                        }
                        DataType::Number
                    }
                    _ => return Err(self.easy_error("Unexpected operator".to_string())),
                }
            }
            Some(Token::Literal(Literal::Ident(name))) => {
                let idx = self.find_variable(name)?;
                self.to_u8_with_args(Opcode::LoadVar as u8, idx);
                let data_type = self.get_var_type(idx);
                data_type
            },
            Some(Token::Literal(Literal::Text(t))) => {
                let str_idx= self.add_string_const(t.to_string());
                self.to_u8_with_args(Opcode::LoadString as u8, str_idx);
                DataType::Text
            }
            _ => return Err(self.easy_error("Expected number or variable".to_string())),
        };

        while let Some(next_t) = self.peek_token() {
            let (l_bp, r_bp) = match next_t {
                Token::KeyWord(KeyWordType::Or) => (1, 2),
                Token::KeyWord(KeyWordType::And) => (3, 4),
                Token::CmpOp(_) => (5, 6),
                Token::OpType(op_type) => {
                    match op_type {
                        OpType::Plus | OpType::Minus => (7, 8),
                        OpType::Multiply | OpType::Divide | OpType::Mod => (9, 10),
                        OpType::Power => (13, 12),
                        _ => break,
                    }
                },
                _ => break,
            };

            if l_bp < min_bp { break; }

            let op_token = self.next_token().unwrap();

            let right_type = self.expr_bp(r_bp)?;

            if current_type != right_type {
                return Err(self.easy_error("Type mismatch in expression".to_string()));
            }

            match op_token {
                Token::OpType(token) => self.to_u8(token.to_opcode() as u8),
                Token::CmpOp(token) => self.to_u8(token.to_opcode() as u8),
                Token::KeyWord(KeyWordType::And) => self.to_u8(Opcode::And as u8),
                Token::KeyWord(KeyWordType::Or) => self.to_u8(Opcode::Or as u8),
                _ => unreachable!(),
            }
        }
        Ok(current_type)
    }
}
