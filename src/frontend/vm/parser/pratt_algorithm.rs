use crate::diagnostic::diagnostic::{ErrorHandler, easy_error};
use crate::frontend::token::{KeyWordType, Literal, OpType, Token};

use super::bparser::Bparser;
use super::opcodes::Opcode;
use super::types::{Constants, DataType};

impl<'a> Bparser<'a> {
    pub(super) fn handle_assignment_op(
        &mut self,
        var_name: &'a str,
        int_op: Opcode,
        float_op: Opcode,
        is_inc_or_dec: bool,
    ) -> Result<(), ErrorHandler> {
        // Get the variable index to get the type, call LoadVar and StoreVar
        let var_idx = self.find_variable(var_name)?;
        self.to_u8_with_args(Opcode::LoadVar as u8, var_idx);
        let var_type = self.get_var_type(var_idx);

        // True than we have tokens ++ or --
        if is_inc_or_dec {
            match var_type {
                DataType::Int => {
                    let const_idx = self.add_constant(Constants::Int(1));
                    self.to_u8_with_args(Opcode::LoadConst as u8, const_idx);
                }
                DataType::Float => {
                    let const_idx = self.add_constant(Constants::Float(1.0));
                    self.to_u8_with_args(Opcode::LoadConst as u8, const_idx);
                }
                _ => {
                    return Err(easy_error(
                        "Modification can only be applied to Int or Float".to_string(),
                        self.current_line,
                    ));
                }
            }
        // False than we have tokens += or  -=
        } else {
            let right_type = self.expr_bp(0)?;
            if var_type != right_type {
                return Err(easy_error(
                    format!(
                        "Type mismatch: cannot apply operation to {:?} and {:?}",
                        var_type, right_type
                    ),
                    self.current_line,
                ));
            }
        }

        // We use our int_op and float_op to send the correct opcode
        match var_type {
            DataType::Int => self.to_u8(int_op as u8),
            DataType::Float => self.to_u8(float_op as u8),
            _ => {
                return Err(easy_error(
                    "Operation can only be applied to Int or Float".to_string(),
                    self.current_line,
                ));
            }
        }

        // Call StoreVar to rewrite the new value
        self.to_u8_with_args(Opcode::StoreVar as u8, var_idx);

        Ok(())
    }

    pub fn expr_bp(&mut self, min_bp: u16) -> Result<DataType, ErrorHandler> {
        let mut current_type = match self.next_token() {
            Some(Token::Literal(Literal::Int(num))) => {
                let int_idx = self.add_constant(Constants::Int(num));
                self.to_u8_with_args(Opcode::LoadConst as u8, int_idx);
                DataType::Int
            }
            Some(Token::Literal(Literal::Float(num))) => {
                let float_idx = self.add_constant(Constants::Float(num));
                self.to_u8_with_args(Opcode::LoadConst as u8, float_idx);
                DataType::Float
            }
            Some(Token::Literal(Literal::Bool(b))) => {
                let bool_idx = self.add_constant(Constants::Bool(b));
                self.to_u8_with_args(Opcode::LoadConst as u8, bool_idx);
                DataType::Bool
            }
            Some(Token::Literal(Literal::Ident(name))) => {
                let var_idx = self.find_variable(name)?;
                self.to_u8_with_args(Opcode::LoadVar as u8, var_idx);
                let data_type = self.get_var_type(var_idx);
                data_type
            }
            Some(Token::Literal(Literal::Text(t))) => {
                let str_idx = self.add_constant(Constants::Text(t.to_string()));
                self.to_u8_with_args(Opcode::LoadConst as u8, str_idx);
                DataType::Text
            }
            Some(Token::OpType(OpType::LParen)) => {
                let inner_type = self.expr_bp(0)?;
                if let Some(token) = self.next_token() {
                    if token != Token::OpType(OpType::RParen) {
                        return Err(easy_error(
                            "Expected matching ')'".to_string(),
                            self.current_line,
                        ));
                    }
                }
                inner_type
            }
            Some(Token::KeyWord(KeyWordType::Not)) => {
                self.expr_bp(0)?;
                self.to_u8(Opcode::Not as u8);
                DataType::Bool
            }
            Some(Token::OpType(op_type)) => match op_type {
                OpType::Plus | OpType::Minus => {
                    let num_type = self.expr_bp(11)?;
                    if num_type != DataType::Int && num_type != DataType::Float {
                        return Err(easy_error(
                            "Unary operator can only be applied to Int or Float".to_string(),
                            self.current_line,
                        ));
                    }
                    if op_type == OpType::Minus {
                        match num_type {
                            DataType::Int => self.to_u8(Opcode::INegate as u8),
                            DataType::Float => self.to_u8(Opcode::FNegate as u8),
                            _ => {
                                return Err(easy_error(
                                    "Unexpected tokens in Negate function".to_string(),
                                    self.current_line,
                                ));
                            }
                        }
                    }
                    num_type
                }
                _ => {
                    return Err(easy_error(
                        "Unexpected operator".to_string(),
                        self.current_line,
                    ));
                }
            },
            _ => {
                return Err(easy_error(
                    "Expected number or variable".to_string(),
                    self.current_line,
                ));
            }
        };

        while let Some(next_t) = self.peek_token() {
            let (l_bp, r_bp) = match next_t {
                Token::KeyWord(KeyWordType::Or) => (1, 2),
                Token::KeyWord(KeyWordType::And) => (3, 4),
                Token::CmpOp(_) => (5, 6),
                Token::OpType(op_type) => match op_type {
                    OpType::Plus | OpType::Minus => (7, 8),
                    OpType::Multiply | OpType::Divide | OpType::Mod => (9, 10),
                    OpType::Power => (13, 12),
                    _ => break,
                },
                _ => break,
            };

            if l_bp < min_bp {
                break;
            }

            let op_token = self.next_token().unwrap();

            let right_type = self.expr_bp(r_bp)?;

            let result_type = match op_token {
                Token::KeyWord(KeyWordType::And) | Token::KeyWord(KeyWordType::Or) => {
                    if current_type != DataType::Bool || right_type != DataType::Bool {
                        return Err(easy_error(
                            "Logical operators expect Boolean expressions".to_string(),
                            self.current_line,
                        ));
                    }
                    DataType::Bool
                }
                Token::CmpOp(_) => {
                    if current_type != right_type {
                        return Err(easy_error(
                            "Cannot compare different types".to_string(),
                            self.current_line,
                        ));
                    }
                    DataType::Bool
                }
                Token::OpType(_) => {
                    if current_type != right_type {
                        println!("{:?}", current_type);
                        println!("{:?}", right_type);
                        return Err(easy_error(
                            "Type mismatch in math expression".to_string(),
                            self.current_line,
                        ));
                    }
                    current_type
                }
                _ => unreachable!(),
            };

            match op_token {
                Token::KeyWord(KeyWordType::And) => self.to_u8(Opcode::And as u8),
                Token::KeyWord(KeyWordType::Or) => self.to_u8(Opcode::Or as u8),
                Token::CmpOp(token) => match current_type {
                    DataType::Int => self.to_u8(token.to_opcode_int() as u8),
                    DataType::Float => self.to_u8(token.to_opcode_float() as u8),
                    _ => {
                        return Err(easy_error(
                            "Invalid operation for this data type".to_string(),
                            self.current_line,
                        ));
                    }
                },
                Token::OpType(op_type) => match current_type {
                    DataType::Int => self.to_u8(op_type.to_opcode_int() as u8),
                    DataType::Float => self.to_u8(op_type.to_opcode_float() as u8),
                    _ => {
                        return Err(easy_error(
                            "Invalid operation for this data type".to_string(),
                            self.current_line,
                        ));
                    }
                },
                _ => unreachable!(),
            }

            current_type = result_type
        }
        Ok(current_type)
    }
}
