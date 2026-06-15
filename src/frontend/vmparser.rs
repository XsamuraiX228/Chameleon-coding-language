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
    Text,
    Int,
    Float,
    Bool,
}

pub struct VarInfo<'a> {
    name: &'a str,
    data_type: DataType,
}

#[derive(PartialEq, Debug)]
pub enum Constants {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

pub struct Bparser<'a> {
    lexer: Peekable<IntoIter<SpannedToken<'a>>>,
    bytecode: Vec<u8>,
    constants: Vec<Constants>,       
    variables: Vec<VarInfo<'a>>,
    current_line: usize,
    dialect: &'a SyntaxDict
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Stop 
    Stop        = 0x00,

    // Memory
    LoadConst   = 0x01,  
    LoadVar     = 0x02,  
    StoreVar    = 0x03,  
    // Math Int (i64)
    IAdd        = 0x04,
    ISub        = 0x05,
    IMul        = 0x06,
    IDiv        = 0x07,
    IMod        = 0x08,
    IPow        = 0x09,
    INegate     = 0x0A,

    // Math Float (f64)
    FAdd        = 0x0B,
    FSub        = 0x0C,
    FMul        = 0x0D,
    FDiv        = 0x0E,
    FMod        = 0x0F,
    FPow        = 0x10,
    FNegate     = 0x11,

    // Compare Int
    IEqual      = 0x12,
    INotEqual   = 0x13,
    ILess       = 0x14,
    ILessEq     = 0x15,
    IGreater    = 0x16,
    IGreaterEq  = 0x17,

    // Compare Float
    FEqual      = 0x18,
    FNotEqual   = 0x19,
    FLess       = 0x1A,
    FLessEq     = 0x1B,
    FGreater    = 0x1C,
    FGreaterEq  = 0x1D,

    // Control flow
    Jump        = 0x1E,
    JumpIfFalse = 0x1F,

    // Logic / Bool
    And         = 0x2A, 
    Or          = 0x2B,
    Not         = 0x2C,

    // IO
    Print       = 0x2D, 
    Input       = 0x2E,
}

impl<'a> Bparser<'a>  {
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

    fn add_constant(&mut self, value: Constants) -> u16 {        
        if let Some(index) = self.constants.iter().position(|c| *c == value) {
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
        self.variables.push(VarInfo { name: value, data_type });
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

    fn get_num(&mut self) -> Result<i64, ErrorHandler> {    
        match self.next_token() {
            Some(Token::Literal(Literal::Int(num))) => Ok(num),
            Some(Token::OpType(OpType::Minus)) => {
                match self.next_token() {
                    Some(Token::Literal(Literal::Int(num))) => Ok(-num),
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
        for (i, c) in self.constants.iter().enumerate() {
            println!("  [{}] = {:?}", i, c);
        }
        
        // Переменные
        println!("\n[Variables] ({} items):", self.variables.len());
        for (i, v) in self.variables.iter().enumerate() {
            println!("  [{}] = {}", i, v.name);
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

        // We push every type of constant to our Vec<u8>
        // To separate them, we first push the unique code for every type
        for constant in &self.constants {
            match constant {
                Constants::Int(num) => {
                    output.push(0x01);
                    output.extend_from_slice(&num.to_le_bytes());
                },
                Constants::Float(num) => {
                    output.push(0x02);
                    output.extend_from_slice(&num.to_le_bytes());
                },
                Constants::Bool(b) => {
                    output.push(0x03);
                    output.push(if *b { 1 } else { 0 });
                },
                Constants::Text(text) => {
                    output.push(0x04);
                    let text_bytes = text.as_bytes();
                    let text_len = text_bytes.len() as u16;

                    output.extend_from_slice(&text_len.to_le_bytes());
                    output.extend_from_slice(text_bytes);
                }
            }
        }

        output.push((self.variables.len() & 0xFF) as u8);
        output.push((self.variables.len() >> 8) as u8);

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
            },
            Some(Token::EOF) => {
                self.next_token();
                Ok(())
            },
            Some(Token::Literal(Literal::Ident(_))) => {
                let variable = self.next_token().unwrap();
                let is_inc_dec = match self.peek_token() {
                    Some(Token::OpType(OpType::Increment)) => true,
                    Some(Token::OpType(OpType::Decrement)) => true,
                    _ => false
                };
                if is_inc_dec {
                    if let Token::Literal(Literal::Ident(name)) = variable {
                        self.increment_decrement(name)?;
                    }
                }
                Ok(())
            },
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
        self.expr_bp(0)?; 
        self.to_u8(Opcode::Print as u8);
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
        let var_id = self.add_variable(var, DataType::Int);
        self.expect(Token::CmpOp(CmpOp::Equal))?;
        self.expr_bp(0)?;

        self.to_u8_with_args(Opcode::StoreVar as u8, var_id);
        self.hard_expect(KeyWordType::To, KeyWordType::For)?;

        let limit = self.get_num()?;
        let limit_id = self.add_constant(Constants::Int(limit));

        let mut step_value = 1;
        if let Some(Token::KeyWord(KeyWordType::Step)) = self.peek_token() {
            self.next_token();
            step_value = self.get_num()?;
        }
        let step_idx = self.add_constant(Constants::Int(step_value));

        let loop_start = self.bytecode.len() as u16;

        self.to_u8_with_args(Opcode::LoadVar as u8, var_id);
        self.to_u8_with_args(Opcode::LoadConst as u8, limit_id);

        if step_value > 0 {
            self.to_u8(Opcode::ILessEq as u8);
        } else {
            self.to_u8(Opcode::IGreaterEq as u8);
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
        self.to_u8(Opcode::IAdd as u8);                           
        self.to_u8_with_args(Opcode::StoreVar as u8, var_id);   

        
        self.to_u8_with_args(Opcode::Jump as u8, loop_start);

        let end_idx = self.bytecode.len() as u16;
        self.patch_address(for_jump_pos, end_idx);


        Ok(())
    }

    fn increment_decrement(&mut self, var_name: &'a str) -> Result<(), ErrorHandler> {
        let var_idx = self.find_variable(var_name)?;
        let token = self.peek_token();
        match token {
            Some(Token::OpType(OpType::Increment)) => {
                self.next_token();
                self.to_u8_with_args(Opcode::LoadVar as u8, var_idx);
                let var_type = self.get_var_type(var_idx);
                match var_type {
                    DataType::Int => {
                        let const_idx = self.add_constant(Constants::Int(1));
                        self.to_u8_with_args(Opcode::LoadConst as u8, const_idx);
                        self.to_u8(Opcode::IAdd as u8);
                        self.to_u8_with_args(Opcode::StoreVar as u8, var_idx);
                    }
                    DataType::Float => {
                        let const_idx = self.add_constant(Constants::Float(1.0));
                        self.to_u8_with_args(Opcode::LoadConst as u8, const_idx);
                        self.to_u8(Opcode::FAdd as u8);
                        self.to_u8_with_args(Opcode::StoreVar as u8, var_idx);
                    }
                    _ => return Err(self.easy_error("Increment can only be applied to Int or Float".to_string()))
                }
            },
            Some(Token::OpType(OpType::Decrement)) => {
                self.next_token();
                self.to_u8_with_args(Opcode::LoadVar as u8, var_idx);
                let var_type = self.get_var_type(var_idx);
                match var_type {
                    DataType::Int => {
                        let const_idx = self.add_constant(Constants::Int(-1));
                        self.to_u8_with_args(Opcode::LoadConst as u8, const_idx);
                        self.to_u8(Opcode::IAdd as u8);
                        self.to_u8_with_args(Opcode::StoreVar as u8, var_idx);
                    }
                    DataType::Float => {
                        let const_idx = self.add_constant(Constants::Float(-1.0));
                        self.to_u8_with_args(Opcode::LoadConst as u8, const_idx);
                        self.to_u8(Opcode::FAdd as u8);
                        self.to_u8_with_args(Opcode::StoreVar as u8, var_idx);
                    }
                    _ => return Err(self.easy_error("Decrement can only be applied to Int or Float".to_string()))
                }
            },
            _ => return Err(self.easy_error("Expected '++' or '--' after variable name".to_string()))
        }
        Ok(())
    }

    pub fn expr_bp(&mut self, min_bp: u16) -> Result<DataType, ErrorHandler> {
        let mut current_type = match self.next_token() {
            Some(Token::Literal(Literal::Int(num))) => {
                let int_idx = self.add_constant(Constants::Int(num));
                self.to_u8_with_args(Opcode::LoadConst as u8, int_idx);
                DataType::Int
            },
            Some(Token::Literal(Literal::Float(num))) => {
                let float_idx = self.add_constant(Constants::Float(num));
                self.to_u8_with_args(Opcode::LoadConst as u8, float_idx);
                DataType::Float
            },
            Some(Token::Literal(Literal::Bool(b))) => {
                let bool_idx = self.add_constant(Constants::Bool(b));
                self.to_u8_with_args(Opcode::LoadConst as u8, bool_idx);
                DataType::Bool
            },
            Some(Token::Literal(Literal::Ident(name))) => {
                let var_idx = self.find_variable(name)?;
                self.to_u8_with_args(Opcode::LoadVar as u8, var_idx);
                let data_type = self.get_var_type(var_idx);
                data_type
            },
            Some(Token::Literal(Literal::Text(t))) => {
                let str_idx= self.add_constant(Constants::Text(t.to_string()));
                self.to_u8_with_args(Opcode::LoadConst as u8, str_idx);
                DataType::Text
            }
            Some(Token::OpType(OpType::LParen)) => {
                let inner_type = self.expr_bp(0)?;
                if let Some(token) = self.next_token() {
                    if token != Token::OpType(OpType::RParen) {
                        return Err(self.easy_error("Expected matching ')'".to_string()));
                    }
                }
                inner_type
            },
            Some(Token::KeyWord(KeyWordType::Not)) => {
                self.expr_bp(0)?;
                self.to_u8(Opcode::Not as u8);
                DataType::Bool
            },
            Some(Token::OpType(op_type)) => {
                match op_type {
                    OpType::Plus | OpType::Minus => {
                        let num_type = self.expr_bp(11)?;
                        if num_type != DataType::Int && num_type != DataType::Float {
                            return Err(self.easy_error("Unary operator can only be applied to Int or Float".to_string()));
                        }
                        if op_type == OpType::Minus {
                            match num_type {
                                DataType::Int => self.to_u8(Opcode::INegate as u8),
                                DataType::Float => self.to_u8(Opcode::FNegate as u8),
                                _ => return Err(self.easy_error("Unexpected tokens in Negate function".to_string()))
                            }
                        }
                        num_type
                    }
                    _ => return Err(self.easy_error("Unexpected operator".to_string())),
                }
            },
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

            let result_type = match op_token {
                Token::KeyWord(KeyWordType::And) | Token::KeyWord(KeyWordType::Or) => {
                    if current_type != DataType::Bool || right_type != DataType::Bool {
                        return Err(self.easy_error("Logical operators expect Boolean expressions".to_string()));
                    }
                    DataType::Bool
                },
                Token::CmpOp(_) => {
                    if current_type != right_type {
                        return Err(self.easy_error("Cannot compare different types".to_string()));
                    }
                    DataType::Bool 
                },
                Token::OpType(_) => {
                    if current_type != right_type {
                        println!("{:?}", current_type);
                        println!("{:?}", right_type);
                        return Err(self.easy_error("Type mismatch in math expression".to_string()));
                    }
                    current_type 
                },
                _ => unreachable!(),
            };

            match op_token {
                Token::KeyWord(KeyWordType::And) => self.to_u8(Opcode::And as u8),
                Token::KeyWord(KeyWordType::Or) => self.to_u8(Opcode::Or as u8),
                Token::CmpOp(token) => {
                    match current_type {
                        DataType::Int => self.to_u8(token.to_opcode_int() as u8),
                        DataType::Float => self.to_u8(token.to_opcode_float() as u8),
                        _ => return Err(self.easy_error("Invalid operation for this data type".to_string())),
                    }
                } 
                Token::OpType(op_type) => {
                    match current_type {
                        DataType::Int => self.to_u8(op_type.to_opcode_int() as u8),
                        DataType::Float => self.to_u8(op_type.to_opcode_float() as u8),
                        _ => return Err(self.easy_error("Invalid operation for this data type".to_string())),
                    }
                },
                _ => unreachable!(),
            }

            current_type = result_type
        }
        Ok(current_type)
    }
}
