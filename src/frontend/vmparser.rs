use crate::frontend::lexer::SpannedToken;
use crate::frontend::token::KeyWordType;
use crate::diagnostic::diagnostic::{ErrorHandler, ErrorKind};
use crate::frontend::token::Token;
use super::token::{CmpOp, OpType, Literal};
use crate::dialect::SyntaxDict;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    // === Системные команды ===
    Stop        = 0x00,

    // === Работа с памятью ===
    LoadConst   = 0x01,
    LoadVar     = 0x02,
    StoreVar    = 0x03,

    // === Бинарная математика ===
    Add         = 0x04,
    Sub         = 0x05,
    Mul         = 0x06,
    Div         = 0x07,
    Mod         = 0x08,
    Pow         = 0x09,

    // === Унарные операции ===
    Negate      = 0x0B,

    // === Сравнения ===
    Equal       = 0x0C,
    NotEqual    = 0x0D,
    Greater     = 0x0E,
    Less        = 0x0F,
    GreaterEq   = 0x10,
    LessEq      = 0x11,

    // === Управление потоком ===
    Jump        = 0x12,
    JumpIfFalse = 0x13,

    // === Ввод/Вывод ===
    Print       = 0x14,
}

pub struct ByteParser<'a> {
    lexer: Vec<SpannedToken<'a>>,
    bytecode: Vec<u16>,
    pub constants: Vec<i64>,           // ← i64
    pub variables: Vec<&'a str>,
    current_line: usize,
    dialect: &'a SyntaxDict
}

impl<'a> ByteParser<'a>  {
    pub fn new(lexer: Vec<SpannedToken<'a>>, dialect: &'a SyntaxDict ) -> Self {
        Self {
            lexer, 
            bytecode: Vec::new(), 
            constants: Vec::new(), 
            variables: Vec::new(), 
            current_line: 1,
            dialect
        }
    }

    pub fn peek_token(&self) -> Option<&Token<'a>> {
        self.lexer.last().map(|token| &token.token)
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        let next_token = self.lexer.pop()?;
        self.current_line = next_token.line; 
        Some(next_token.token)
    }

    fn add_constant(&mut self, value: i64) -> u8 {          // ← i64
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

    fn get_num(&mut self) -> Result<i64, ErrorHandler> {    // ← i64
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

    fn expect_cmptoken(&mut self, expected: Token<'a>) -> Result<(), ErrorHandler> {
        if let Some(token) = self.next_token() {
            if token == expected {
                return Ok(());
            }
        }
        Err(self.easy_error(format!("Expected {:?}", expected)))
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

    pub fn byteparse(&mut self) -> Result<Vec<u16>, ErrorHandler> {
        while self.peek_token().is_some() && self.peek_token() != Some(&Token::EOF) {
            self.byteparse_block()?;
        }
        self.bytecode.push(OpCode::Stop as u16);
        Ok(self.bytecode.clone())
    }

    pub fn debug(&self) {
        println!("=== BYTECODE DUMP (16-bit) ===");
        for (index, value) in self.bytecode.iter().enumerate() {
            println!("{:3}: {:04X}", index, value);
            if (index + 1) % 8 == 0 { println!(); }
        }
        if self.bytecode.len() % 8 != 0 { println!(); }
        println!("==============================");
        println!("Constants: {:?}", self.constants);
        println!("Variables: {:?}", self.variables);
    }

    pub fn byteparse_block(&mut self) -> Result<(), ErrorHandler> {
        match &self.peek_token() {
            Some(Token::KeyWord(KeyWordType::Print)) => self.parse_print(),
            Some(Token::KeyWord(KeyWordType::Let)) => self.parse_let(),
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
            Some(t) => Err(self.easy_error(format!("Unexpected token: {:?}", t))),
            None => Ok(()),
        }
    }

    fn parse_let(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var_name = self.get_name()?;
        self.expect_cmptoken(Token::CmpOp(CmpOp::Equal))?;
        self.expr_bp(0)?;
        let var_id = self.add_variable(var_name);
        self.bytecode.push(OpCode::StoreVar as u16);
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
        self.next_token();
        self.expr_bp(0)?;
        self.hard_expect(KeyWordType::Then, KeyWordType::If)?;
        
        self.bytecode.push(OpCode::JumpIfFalse as u16);
        let jump_patch = self.bytecode.len();
        self.bytecode.push(0);

        while let Some(token) = self.peek_token() {
            match token {
                Token::KeyWord(KeyWordType::Else) => break,
                Token::KeyWord(KeyWordType::End) => break,
                _ => { self.byteparse_block()?; }
            }
        }

        if let Some(Token::KeyWord(KeyWordType::Else)) = self.peek_token() {
            self.next_token();
            
            self.bytecode.push(OpCode::Jump as u16);
            let else_jump_patch = self.bytecode.len();
            self.bytecode.push(0);
            
            let jump_target = self.bytecode.len();
            self.bytecode[jump_patch] = jump_target as u16;
            
            while let Some(token) = self.peek_token() {
                if let Token::KeyWord(KeyWordType::End) = token { break; }
                self.byteparse_block()?;
            }
            
            let else_target = self.bytecode.len();
            self.bytecode[else_jump_patch] = else_target as u16;
        } else {
            let jump_target = self.bytecode.len();
            self.bytecode[jump_patch] = jump_target as u16;
        }
        
        self.hard_expect(KeyWordType::End, KeyWordType::If)?;
        Ok(())
    }

    fn parse_while(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let loop_start = self.bytecode.len();
        self.expr_bp(0)?;
        self.hard_expect(KeyWordType::Then, KeyWordType::While)?;
        
        self.bytecode.push(OpCode::JumpIfFalse as u16);
        let exit_patch = self.bytecode.len();
        self.bytecode.push(0);
        
        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Wend) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }
        
        self.bytecode.push(OpCode::Jump as u16);
        self.bytecode.push(loop_start as u16);
        
        let loop_end = self.bytecode.len();
        self.bytecode[exit_patch] = loop_end as u16;
        Ok(())
    }

    fn parse_for(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var = self.get_name()?;
        self.expect(Token::CmpOp(CmpOp::Equal))?;
        self.expr_bp(0)?;
        
        let var_idx = self.add_variable(var);
        self.bytecode.push(OpCode::StoreVar as u16);
        self.bytecode.push(var_idx as u16);
        
        self.hard_expect(KeyWordType::To, KeyWordType::For)?;
        let num = self.get_num()?;
        let num_idx = self.add_constant(num);
        
        let mut step_value = 1;
        if let Some(Token::KeyWord(KeyWordType::Step)) = self.peek_token() {
            self.next_token();
            step_value = self.get_num()?;
        }
        
        let loop_start = self.bytecode.len();
        
        self.bytecode.push(OpCode::LoadVar as u16);
        self.bytecode.push(var_idx as u16);
        self.bytecode.push(OpCode::LoadConst as u16);
        self.bytecode.push(num_idx as u16);
        
        if step_value > 0 {
            self.bytecode.push(OpCode::LessEq as u16);
        } else {
            self.bytecode.push(OpCode::GreaterEq as u16);
        }
        
        self.bytecode.push(OpCode::JumpIfFalse as u16);
        let end_patch = self.bytecode.len();
        self.bytecode.push(0);
        
        while let Some(token) = self.peek_token() {
            if *token == Token::KeyWord(KeyWordType::Next) {
                self.next_token();
                break;
            }
            self.byteparse_block()?;
        }
        
        self.bytecode.push(OpCode::LoadVar as u16);
        self.bytecode.push(var_idx as u16);
        
        let step_idx = self.add_constant(step_value);
        self.bytecode.push(OpCode::LoadConst as u16);
        self.bytecode.push(step_idx as u16);
        self.bytecode.push(OpCode::Add as u16);
        self.bytecode.push(OpCode::StoreVar as u16);
        self.bytecode.push(var_idx as u16);
        self.bytecode.push(OpCode::Jump as u16);
        self.bytecode.push(loop_start as u16);
        
        let loop_end = self.bytecode.len();
        self.bytecode[end_patch] = loop_end as u16;
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

            if l_bp < min_bp as u8 { break; }

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