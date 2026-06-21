use crate::diagnostic::diagnostic::{ErrorHandler, easy_error};
use crate::frontend::token::{Literal, OpType, Token};

use super::bparser::Bparser;
use super::types::{Constants, DataType, VarInfo};

impl<'a> Bparser<'a> {
    pub(super) fn peek_token(&mut self) -> Option<&Token<'a>> {
        self.lexer.peek().map(|spanned| &spanned.token)
    }

    pub(super) fn next_token(&mut self) -> Option<Token<'a>> {
        let next_token = self.lexer.next()?;
        self.current_line = next_token.line;
        Some(next_token.token)
    }

    pub(super) fn add_constant(&mut self, value: Constants) -> u16 {
        if let Some(index) = self.constants.iter().position(|c| *c == value) {
            return index as u16;
        }
        self.constants.push(value);
        (self.constants.len() - 1) as u16
    }

    pub(super) fn add_variable(&mut self, value: &'a str, data_type: DataType) -> u16 {
        if let Some(index) = self.variables.iter().position(|c| c.name == value) {
            self.variables[index].data_type = data_type;
            return index as u16;
        }
        self.variables.push(VarInfo {
            name: value,
            data_type,
        });
        (self.variables.len() - 1) as u16
    }

    pub(super) fn find_variable(&self, name: &str) -> Result<u16, ErrorHandler> {
        if let Some(index) = self.variables.iter().position(|v| v.name == name) {
            Ok(index as u16)
        } else {
            Err(easy_error(
                format!("Undeclared variable: '{}'", name),
                self.current_line,
            ))
        }
    }

    pub(super) fn get_num(&mut self) -> Result<i64, ErrorHandler> {
        match self.next_token() {
            Some(Token::Literal(Literal::Int(num))) => Ok(num),
            Some(Token::OpType(OpType::Minus)) => match self.next_token() {
                Some(Token::Literal(Literal::Int(num))) => Ok(-num),
                _ => Err(easy_error(
                    "Expected number after '-'".to_string(),
                    self.current_line,
                )),
            },
            _ => Err(easy_error("Expected number".to_string(), self.current_line)),
        }
    }

    pub(super) fn get_name(&mut self) -> Result<&'a str, ErrorHandler> {
        match self.next_token() {
            Some(Token::Literal(Literal::Ident(name))) => Ok(name),
            _ => Err(easy_error(
                "Expected variable name".to_string(),
                self.current_line,
            )),
        }
    }

    pub(super) fn get_var_type(&self, var_id: u16) -> DataType {
        self.variables[var_id as usize].data_type
    }

    pub(super) fn to_u8_with_args(&mut self, opcode: u8, arg: u16) {
        self.bytecode.push(opcode);
        self.bytecode.push((arg & 0xFF) as u8);
        self.bytecode.push(((arg >> 8) & 0xFF) as u8);
    }

    pub(super) fn to_u8(&mut self, opcode: u8) {
        self.bytecode.push(opcode);
    }
}
