use std::iter::Peekable;
use std::vec::IntoIter;

use crate::diagnostic::diagnostic::{ErrorHandler, easy_error, hard_error};
use crate::dialect::SyntaxDict;
use crate::frontend::lexer::SpannedToken;
use crate::frontend::token::{KeyWordType, Literal, OpType, Token};
use crate::frontend::vm::compiler::opcodes::Opcode::{FAdd, FSub, IAdd, ISub};

use super::opcodes::Opcode;
use super::types::{Constants, VarInfo};

pub struct Bparser<'a> {
    pub(super) lexer: Peekable<IntoIter<SpannedToken<'a>>>,
    pub(super) bytecode: Vec<u8>,
    pub(super) constants: Vec<Constants>,
    pub(super) variables: Vec<VarInfo<'a>>,
    pub(super) current_line: usize,
    pub(super) dialect: &'a SyntaxDict,
}

impl<'a> Bparser<'a> {
    pub fn new(tokens: Vec<SpannedToken<'a>>, dialect: &'a SyntaxDict) -> Self {
        Self {
            lexer: tokens.into_iter().peekable(),
            bytecode: Vec::new(),
            constants: Vec::new(),
            variables: Vec::new(),
            current_line: 1,
            dialect,
        }
    }

    pub(super) fn hard_expect(
        &mut self,
        expected: KeyWordType,
        context: KeyWordType,
    ) -> Result<(), ErrorHandler> {
        let curr_token = self.next_token();
        if curr_token == Some(Token::KeyWord(expected)) {
            return Ok(());
        }
        let error_kw_str = self.dialect.get_kw_word(expected);
        let context_kw_str = self.dialect.get_kw_word(context);
        Err(hard_error(error_kw_str, context_kw_str, self.current_line))
    }

    pub fn debug_dump(&self) {
        println!("\n=== PARSER DEBUG INFO ===");

        println!("\n[Constants] ({} items):", self.constants.len());
        for (i, c) in self.constants.iter().enumerate() {
            println!("  [{}] = {:?}", i, c);
        }

        println!("\n[Variables] ({} items):", self.variables.len());
        for (i, v) in self.variables.iter().enumerate() {
            println!("  [{}] = {}", i, v.name);
        }

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
                }
                Constants::Float(num) => {
                    output.push(0x02);
                    output.extend_from_slice(&num.to_le_bytes());
                }
                Constants::Bool(b) => {
                    output.push(0x03);
                    output.push(if *b { 1 } else { 0 });
                }
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
            }
            Some(Token::EOF) => {
                self.next_token();
                Ok(())
            }
            Some(Token::Literal(Literal::Ident(name))) => {
                let var_name = *name;
                self.next_token();

                match self.peek_token() {
                    Some(Token::OpType(OpType::Increment)) => {
                        self.next_token();
                        self.handle_assignment_op(var_name, IAdd, FAdd, true)?;
                    }
                    Some(Token::OpType(OpType::Decrement)) => {
                        self.next_token();
                        self.handle_assignment_op(var_name, ISub, FSub, true)?;
                    }
                    Some(Token::OpType(OpType::IncEqual)) => {
                        self.next_token();
                        self.handle_assignment_op(var_name, IAdd, FAdd, false)?;
                    }
                    Some(Token::OpType(OpType::DecEqual)) => {
                        self.next_token();
                        self.handle_assignment_op(var_name, ISub, FSub, false)?;
                    }
                    _ => {
                        return Err(easy_error(
                            "Expected assignment or modification operator".to_string(),
                            self.current_line,
                        ));
                    }
                }
                Ok(())
            }
            None => Ok(()),
            Some(t) => {
                let token_clone = t.clone();
                Err(easy_error(
                    format!("Unexpected token: {:?}", token_clone),
                    self.current_line,
                ))
            }
        }
    }
}
