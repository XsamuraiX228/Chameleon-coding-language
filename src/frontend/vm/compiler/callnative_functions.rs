use super::bparser::Bparser;
use super::types::CallType;
use crate::{
    diagnostic::diagnostic::{ErrorHandler, easy_error},
    frontend::{
        token::{FuncKeyWord, OpType, Token},
        vm::compiler::{opcodes::Opcode, types::DataType},
    },
};

impl<'a> Bparser<'a> {
    pub(super) fn parse_func(&mut self, fw_type: FuncKeyWord) -> Result<DataType, ErrorHandler> {
        if !matches!(self.peek_token(), Some(Token::OpType(OpType::LParen))) {
            return Err(easy_error(
                "Expected '(' after functions definitions".to_string(),
                self.current_line,
            ));
        }

        self.next_token();

        let final_call_type = match fw_type {
            // Only Float
            FuncKeyWord::Sin => {
                let t = self.expr_bp(0)?;
                if t != DataType::Float {
                    return Err(easy_error(
                        "Sin expects Float".to_string(),
                        self.current_line,
                    ));
                }
                CallType::Sin
            }
            FuncKeyWord::Cos => {
                let t = self.expr_bp(0)?;
                if t != DataType::Float {
                    return Err(easy_error(
                        "Cos expects Float".to_string(),
                        self.current_line,
                    ));
                }
                CallType::Cos
            }
            FuncKeyWord::Sqrt => {
                let t = self.expr_bp(0)?;
                if t != DataType::Float {
                    return Err(easy_error(
                        "Sqrt expects Float".to_string(),
                        self.current_line,
                    ));
                }
                CallType::Sqrt
            }

            // Int or Float
            FuncKeyWord::Abs => {
                let t = self.expr_bp(0)?;
                match t {
                    DataType::Int => CallType::AbsInt,
                    DataType::Float => CallType::AbsFloat,
                    _ => {
                        return Err(easy_error(
                            "Abs expects Int or Float".to_string(),
                            self.current_line,
                        ));
                    }
                }
            }

            // Binary functions
            FuncKeyWord::Max | FuncKeyWord::Min | FuncKeyWord::Random => {
                let val_1_type = self.expr_bp(0)?;

                if !matches!(self.peek_token(), Some(Token::Comma)) {
                    return Err(easy_error(
                        "Expected ',' after first argument".to_string(),
                        self.current_line,
                    ));
                }

                self.next_token();

                let val_2_type = self.expr_bp(0)?;

                if val_1_type != val_2_type {
                    return Err(easy_error(
                        "Type mismatch in function".to_string(),
                        self.current_line,
                    ));
                }

                match fw_type {
                    FuncKeyWord::Max => {
                        if val_1_type == DataType::Int {
                            CallType::MaxInt
                        } else {
                            CallType::MaxFloat
                        }
                    }
                    FuncKeyWord::Min => {
                        if val_1_type == DataType::Int {
                            CallType::MinInt
                        } else {
                            CallType::MinFloat
                        }
                    }
                    FuncKeyWord::Random => {
                        if val_1_type == DataType::Int {
                            CallType::RandomInt
                        } else {
                            CallType::RandomFloat
                        }
                    }
                    _ => unreachable!(),
                }
            }
        };

        let return_type = match final_call_type {
            CallType::Sin
            | CallType::Cos
            | CallType::Sqrt
            | CallType::AbsFloat
            | CallType::MinFloat
            | CallType::MaxFloat
            | CallType::RandomFloat => DataType::Float,
            CallType::AbsInt | CallType::MinInt | CallType::MaxInt | CallType::RandomInt => {
                DataType::Int
            }
        };

        if !matches!(self.peek_token(), Some(Token::OpType(OpType::RParen))) {
            return Err(easy_error(
                "Expected ')' to close function".to_string(),
                self.current_line,
            ));
        }

        self.next_token();

        self.to_u8(Opcode::CallNative as u8);
        self.to_u8(final_call_type as u8);
        Ok(return_type)
    }
}
