use crate::diagnostic::diagnostic::{ErrorHandler, easy_error};
use crate::frontend::token::{CmpOp, KeyWordType, Token};

use super::bparser::Bparser;
use super::opcodes::Opcode;
use super::types::{Constants, DataType};

impl<'a> Bparser<'a> {
    pub(super) fn parse_let(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var_name = self.get_name()?;
        if self.next_token() != Some(Token::CmpOp(CmpOp::Equal)) {
            return Err(easy_error(
                format!("Expected token = after variable name"),
                self.current_line,
            ));
        }
        let data_type = self.expr_bp(0)?;
        let var_id = self.add_variable(var_name, data_type);
        self.to_u8_with_args(Opcode::StoreVar as u8, var_id);
        Ok(())
    }

    pub(super) fn parse_input(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        let var_name = self.get_name()?;
        let var_id = self.find_variable(var_name)?;
        self.to_u8_with_args(Opcode::Input as u8, var_id);
        Ok(())
    }

    pub(super) fn parse_print(&mut self) -> Result<(), ErrorHandler> {
        self.next_token();
        self.expr_bp(0)?;
        self.to_u8(Opcode::Print as u8);
        Ok(())
    }

    pub(super) fn patch_address(&mut self, instr_pos: usize, target_instruction_idx: u16) {
        let low_byte = (target_instruction_idx & 0xFF) as u8;
        let high_byte = ((target_instruction_idx >> 8) & 0xFF) as u8;

        self.bytecode[instr_pos + 1] = low_byte;
        self.bytecode[instr_pos + 2] = high_byte;
    }

    pub(super) fn parse_if(&mut self) -> Result<(), ErrorHandler> {
        self.next_token(); // Skip If
        self.expr_bp(0)?; // Get result 1 or 0
        self.hard_expect(KeyWordType::Then, KeyWordType::If)?; // Check if THEN keyword was written

        let start_if_condition = self.bytecode.len();
        self.to_u8_with_args(Opcode::JumpIfFalse as u8, 0); // send a Check for conditional

        while let Some(token) = self.peek_token() {
            match token {
                Token::KeyWord(KeyWordType::Else) => break,
                Token::KeyWord(KeyWordType::End) => break,
                _ => {
                    self.byteparse_block()?;
                }
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

    pub(super) fn parse_else(&mut self, jump_if_false_pos: usize) -> Result<(), ErrorHandler> {
        self.next_token();
        let jump_pos = self.bytecode.len();
        self.to_u8_with_args(Opcode::Jump as u8, 0);

        let start_else_condition = self.bytecode.len() as u16;
        self.patch_address(jump_if_false_pos, start_else_condition);

        while let Some(token) = self.peek_token() {
            match token {
                Token::KeyWord(KeyWordType::End) => break,
                _ => {
                    self.byteparse_block()?;
                }
            }
        }

        self.hard_expect(KeyWordType::End, KeyWordType::If)?;

        let end_idx = self.bytecode.len() as u16;
        self.patch_address(jump_pos, end_idx);
        Ok(())
    }

    pub(super) fn parse_while(&mut self) -> Result<(), ErrorHandler> {
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

    pub(super) fn parse_for(&mut self) -> Result<(), ErrorHandler> {
        self.next_token(); // Skip FOR

        // Read var name
        let var = self.get_name()?;
        let var_id = self.add_variable(var, DataType::Int);
        if self.next_token() != Some(Token::CmpOp(CmpOp::Equal)) {
            return Err(easy_error(
                format!("Expected token = after variable name"),
                self.current_line,
            ));
        }
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
}
