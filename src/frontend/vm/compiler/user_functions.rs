use crate::{
    diagnostic::diagnostic::{ErrorHandler, easy_error},
    frontend::{
        token::{FuncOp, OpType, Token},
        vm::compiler::{bparser::Bparser, opcodes::Opcode, types::DataType},
    },
};

use super::types::VarInfo;

#[allow(dead_code)]

pub struct UserFunction<'a> {
    pub name: String,
    pub function_bc: Vec<u8>,
    pub local_vars: Vec<VarInfo<'a>>,
    pub args_amout: usize,
    pub return_type: DataType,
}

impl<'a> Bparser<'a> {
    pub(super) fn parse_user_fc(&mut self) -> Result<DataType, ErrorHandler> {
        self.next_token(); // Skip FN

        let f_name = self.get_name()?; // Get the name of our functions

        let global_bytecode = std::mem::take(&mut self.bytecode);
        let global_variables = std::mem::take(&mut self.variables);

        if !matches!(self.peek_token(), Some(Token::OpType(OpType::LParen))) {
            return Err(easy_error(
                "Expected '(' after function name".to_string(),
                self.current_line,
            ));
        }
        self.next_token(); // Skip (
        let mut agrs_count = 0;

        while !matches!(self.peek_token(), Some(Token::OpType(OpType::RParen))) {
            let var_name = self.get_name()?; // Get var name

            if !matches!(self.peek_token(), Some(Token::UserFunc(FuncOp::Colon))) {
                return Err(easy_error(
                    "Expected ':' after variable name".to_string(),
                    self.current_line,
                ));
            }
            self.next_token(); // Skip :

            let var_type = self.get_type()?;
            self.add_variable(var_name, var_type);
            agrs_count += 1;

            if matches!(self.peek_token(), Some(Token::Comma)) {
                self.next_token(); // Skip ,

                if matches!(self.peek_token(), Some(Token::OpType(OpType::RParen))) {
                    return Err(easy_error(
                        "Trailing comma is not allowed".to_string(),
                        self.current_line,
                    ));
                }
            }
        }
        self.next_token(); // Skip )

        if !matches!(self.peek_token(), Some(Token::UserFunc(FuncOp::Arrow))) {
            return Err(easy_error(
                "Expected '->' to specify return type".to_string(),
                self.current_line,
            ));
        }
        self.next_token(); // Skip ->

        let return_type = self.get_type()?;

        if !matches!(self.peek_token(), Some(Token::UserFunc(FuncOp::OpenCurly))) {
            return Err(easy_error(
                "Expected '{' before function body".to_string(),
                self.current_line,
            ));
        }
        self.next_token(); // Skip {

        
        let old_return_type = self.return_type;
        let old_has_return = self.has_return;


        self.return_type = return_type;
        self.has_return = false; 

        while !matches!(self.peek_token(), Some(Token::UserFunc(FuncOp::CloseCurly))) {
            self.byteparse_block()?;
        }

        self.next_token(); // Skip }

        
        if !self.has_return {
            return Err(easy_error(
                format!("Function '{}' is missing a RETURN statement", f_name),
                self.current_line,
            ));
        }

        let function_bc = std::mem::take(&mut self.bytecode);
        let local_vars = std::mem::take(&mut self.variables);


        let current_func_return_type = self.return_type; // сохраняем для структуры
        self.return_type = old_return_type;
        self.has_return = old_has_return;


        let user_func = UserFunction {
            name: f_name.to_string(),
            function_bc,
            local_vars,
            args_amout: agrs_count,
            return_type: current_func_return_type,
        };

        self.bytecode = global_bytecode;
        self.variables = global_variables;

        self.register_function(user_func)?;

        Ok(return_type)
    }

    pub(super) fn parse_return(&mut self) -> Result<bool, ErrorHandler> {
        self.next_token(); // Skip RETURN

        let result_type = self.expr_bp(0)?;
        let expected = self.return_type;

        if result_type != expected {
            return Err(easy_error(
                format!(
                    "Mismatched return type. Expected {:?}, found {:?}",
                    expected, result_type
                ),
                self.current_line,
            ));
        }

        self.to_u8(Opcode::Return as u8);
        self.has_return = true;

        Ok(true)
    }
}
