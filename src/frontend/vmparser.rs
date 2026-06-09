use crate::frontend::lexer::SpannedToken;
use crate::frontend::token::{KeyWordType};
use crate::diagnostic::diagnostic::{ErrorHandler, ErrorKind};
use crate::frontend::token::Token;
use super::token::{CmpOp, OpType, Literal};
use crate::dialect::SyntaxDict;
use std::vec::IntoIter;
use std::iter::Peekable;


pub struct Bparser<'a> {
    lexer: Peekable<IntoIter<SpannedToken<'a>>>,
    bytecode: Vec<u8>,
    pub constants: Vec<i64>,       
    pub variables: Vec<&'a str>,
    current_line: usize,
    dialect: &'a SyntaxDict
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcodes {
    // Memory
    LoadConst  = 0x01,  // стек ← константа
    LoadVar    = 0x02,  // стек ← переменная  
    StoreVar   = 0x03,  // переменная ← стек

    // Math 
    Add        = 0x04,
    Sub        = 0x05,
    Mul        = 0x06,
    Div        = 0x07,
    Mod        = 0x08,
    Pow        = 0x09,
    Negate     = 0x0A,

    // Compare 
    Equal      = 0x0B,
    NotEqual   = 0x0C,
    Less       = 0x0D,
    LessEq     = 0x0E,
    Greater    = 0x0F,
    GreaterEq  = 0x10,

    // Control flow
    Jump          = 0x11,
    JumpIfFalse   = 0x12,

    // IO
    Print     = 0x13,
    Input     = 0x14,

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

    fn add_variable(&mut self, value: &'a str) -> u16 {
        if let Some(index) = self.variables.iter().position(|&c| c == value) {
            return index as u16;
        }
        self.variables.push(value);
        (self.variables.len() - 1) as u16
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
        self.bytecode.push(0x00);
        self.bytecode.push(0x00);
    } 

    pub fn start_byteparsing(&mut self) -> Result<Vec<u8>, ErrorHandler> {
        while let Some(_) = self.peek_token() {
            self.byteparse_block()?;
        }
        self.to_u8(Opcodes::Stop as u8);
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
        self.expr_bp(0)?;
        let var_id = self.add_variable(var_name);
        self.to_u8_with_args(Opcodes::StoreVar as u8,var_id);
        Ok(())
    }

    fn parse_input(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var = self.get_name()?;
        let var_id = self.add_variable(var);
        self.to_u8_with_args(Opcodes::Input as u8, var_id);
        Ok(()) 
    }

    fn parse_print(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        self.expr_bp(0)?;
        self.to_u8(Opcodes::Print as u8);
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
        self.to_u8_with_args(Opcodes::JumpIfFalse as u8, 0); // send a Check for conditional

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
            let target = (self.bytecode.len() / 3) as u16;
            self.patch_address(start_if_condition, target);
        }

        Ok(())
    }

    fn parse_else(&mut self,  jump_if_false_pos: usize) -> Result<(),  ErrorHandler> {
        self.next_token();
        let jump_pos = self.bytecode.len();
        self.to_u8_with_args(Opcodes::Jump as u8, 0);

        let start_else_condition = (self.bytecode.len() / 3 ) as u16;
        self.patch_address(jump_if_false_pos, start_else_condition);

        while let Some(token) = self.peek_token() {
            match token {
                Token::KeyWord(KeyWordType::End) => break,
                _ => { self.byteparse_block()?; }
            }
        }

        self.hard_expect(KeyWordType::End, KeyWordType::If)?;

        let end_idx = (self.bytecode.len() / 3) as u16;
        self.patch_address(jump_pos, end_idx);
        Ok(())
    }

    fn parse_while(&mut self) -> Result<(),  ErrorHandler> {
        self.next_token(); // Skip WHILE
        let start_loop = (self.bytecode.len() / 3) as u16;
        self.expr_bp(0)?;
        self.hard_expect(KeyWordType::Then, KeyWordType::While)?;

        let while_start = self.bytecode.len();
        self.to_u8_with_args(Opcodes::JumpIfFalse as u8, 0);
        
        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Wend) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }

        self.to_u8_with_args(Opcodes::Jump as u8, start_loop);
        
        let end_idx = (self.bytecode.len() / 3) as u16;
        self.patch_address(while_start, end_idx);
        Ok(())
    }

    fn parse_for(&mut self) -> Result<(),  ErrorHandler> {
        self.next_token(); // Skip FOR

        // Read var name
        let var = self.get_name()?;
        let var_id = self.add_variable(var);
        self.expect(Token::CmpOp(CmpOp::Equal))?;
        self.expr_bp(0)?;

        self.to_u8_with_args(Opcodes::StoreVar as u8, var_id);
        self.hard_expect(KeyWordType::To, KeyWordType::For)?;

        let limit = self.get_num()?;
        let limit_id = self.add_constant(limit);

        let mut step_value = 1;
        if let Some(Token::KeyWord(KeyWordType::Step)) = self.peek_token() {
            self.next_token();
            step_value = self.get_num()?;
        }
        let step_idx = self.add_constant(step_value);

        let loop_start = (self.bytecode.len() / 3) as u16;

        self.to_u8_with_args(Opcodes::LoadVar as u8, var_id);
        self.to_u8_with_args(Opcodes::LoadConst as u8, limit_id);

        if step_value > 0 {
            self.to_u8(Opcodes::LessEq as u8);
        } else {
            self.to_u8(Opcodes::GreaterEq as u8);
        }

        let for_jump_pos = self.bytecode.len();
        self.to_u8_with_args(Opcodes::JumpIfFalse as u8, 0);

        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Next) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }

        self.to_u8_with_args(Opcodes::LoadVar as u8, var_id);    
        self.to_u8_with_args(Opcodes::LoadConst as u8, step_idx); 
        self.to_u8(Opcodes::Add as u8);                           
        self.to_u8_with_args(Opcodes::StoreVar as u8, var_id);   

        
        self.to_u8_with_args(Opcodes::Jump as u8, loop_start);

        let end_idx = (self.bytecode.len() / 3) as u16;
        self.patch_address(for_jump_pos, end_idx);


        Ok(())
    }


    pub fn expr_bp(&mut self, min_bp: u16) -> Result<(), ErrorHandler> {
        match self.next_token() {
            Some(Token::Literal(Literal::Number(num))) => {
                let idx = self.add_constant(num);
                self.to_u8_with_args(Opcodes::LoadConst as u8, idx);
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
                        let prefix_bp = 5;
                        self.expr_bp(prefix_bp as u16)?;
                        if op_type == OpType::Minus {
                            self.to_u8(Opcodes::Negate as u8);
                        }
                    }
                    _ => return Err(self.easy_error("Unexpected operator".to_string())),
                }
            }
            Some(Token::Literal(Literal::Ident(name))) => {
                let idx = self.add_variable(name);
                self.to_u8_with_args(Opcodes::LoadVar as u8, idx);
            },
            Some(Token::Literal(Literal::Text(_))) => {
                return Err(self.easy_error("Strings not allowed in expressions".to_string()));
            }
            _ => return Err(self.easy_error("Expected number or variable".to_string())),
        };

        while let Some(next_t) = self.peek_token() {
            let (l_bp, r_bp) = match next_t {
                Token::CmpOp(_) => (1, 2),
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

            if l_bp < min_bp { break; }

            let op_token = self.next_token().unwrap();

            self.expr_bp(r_bp)?;
            match op_token {
                Token::OpType(token) => self.to_u8(token.to_opcode() as u8),
                Token::CmpOp(token) => self.to_u8(token.to_opcode() as u8),
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}
