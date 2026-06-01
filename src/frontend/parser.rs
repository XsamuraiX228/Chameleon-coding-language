use crate::frontend::ast::Statement;
use crate::frontend::lexer::SpannedToken;
use crate::frontend::token::KeyWordType;
use crate::diagnostic::diagnostic::{ErrorHandler, ErrorKind};
use super::token::{Token, CmpOp, OpType, Literal};
use super::ast::Expression;
use crate::dialect::SyntaxDict;
pub struct Parser<'a> {
    tokens: Vec<SpannedToken<'a>>,
    pub current_line: usize,
    dialect: &'a SyntaxDict
}

impl<'a> Parser<'a> {

    // Create Parser
    pub fn new(tokens: Vec<SpannedToken<'a>>, dialect: &'a SyntaxDict ) -> Self {
        Self {tokens, current_line: 1, dialect}
    }

    // Functions peek is used to look at the token and check it
    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.last().map(|spanned| &spanned.token)
    }

    // Function next is used to get the token and skip it, by using .pop()
    fn next(&mut self) -> Option<Token<'a>> {
        let spanned= self.tokens.pop()?;
        self.current_line = spanned.line;
        Some(spanned.token)
    }

    // Help function to generate error messages for statements like (IF, FOR, WHILE and etc.)
    fn hard_error(&self, error_kw: String, kw_type: String) -> ErrorHandler{
        let message = format!("Expected '{}' after {} block", error_kw, kw_type);
        ErrorHandler::new(ErrorKind::Syntax, message, self.current_line)
    }

    // Help function to generate error messages for simple statements
    fn easy_error(&self, context: String) -> ErrorHandler {
        ErrorHandler::new(ErrorKind::Syntax, context, self.current_line)
    }

    // Name getter
    fn get_name(&mut self) -> Result<&'a str, ErrorHandler> {
        match self.next() {
            Some(Token::Literal(Literal::Ident(name))) => Ok(name),
            _ => Err(self.easy_error("Expected variable name".to_string())),
        }
    }

    // Num getter
    fn get_num(&mut self) -> Result<i64, ErrorHandler> {
        match self.next() {
            Some(Token::Literal(Literal::Number(num))) => Ok(num),
            _ =>Err(self.easy_error("Expected num".to_string())),
        }
    }


    // Parse_block is use to parse a block of code
    // While, for and if use this function to parse commands inside them
    fn parse_block(&mut self, stop_tokens: &[Token<'a>]) -> Result<Vec<Statement<'a>>, ErrorHandler> {
        let mut block_of_commads = Vec::new();
        while let Some(token) = self.peek() {
            if stop_tokens.iter().any(|t| t == token) {
                break;
            }
            match token {
                Token::Newline => {
                    self.next();
                    continue;
                }
                _ => {
                    let cmd = self.parse_command()?;
                    block_of_commads.push(cmd);
                }
            }
        }
        Ok(block_of_commads)
    }

    // Parse the Vec<Token<'a>>
    pub fn parse(&mut self) -> Result<Vec<Statement<'a>>, ErrorHandler> {
        let mut commands = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Newline => {
                    self.next();
                    continue;
                }
                _ => {
                    let cmd = self.parse_command()?;
                    commands.push(cmd);
                }
                
            }
        }
        Ok(commands)
    }

    // Parse one command at time
    fn parse_command(&mut self) -> Result<Statement<'a>, ErrorHandler> {
        let current_token = self.next().ok_or_else(|| self.easy_error("Unexpected end of file, expected command".to_string()))?;
        match current_token {
            Token::KeyWord(KeyWordType::Let) => self.parse_let(),
            Token::KeyWord(KeyWordType::Print) => self.parse_print(),
            Token::KeyWord(KeyWordType::If) => self.parse_if(),
            Token::KeyWord(KeyWordType::Random) => self.parse_random(),
            Token::Mark(name) => Ok(Statement::Label { name }),
            Token::KeyWord(KeyWordType::End) => Ok(Statement::End),
            Token::KeyWord(KeyWordType::Input) => {
                let name = self.get_name()?;
                Ok(Statement::Input { name })
            }
            Token::KeyWord(KeyWordType::Goto) => {
                let label = self.get_name()?;
                Ok(Statement::Goto { label })
            }
            Token::KeyWord(KeyWordType::While) => self.parse_while(),
            Token::KeyWord(KeyWordType::For) => self.parse_for(),
            Token::KeyWord(KeyWordType::Else) => {
                let error_kw = self.dialect.get_kw_word(KeyWordType::Else);
                let kw_type = self.dialect.get_kw_word(KeyWordType::If);
                Err(self.hard_error(error_kw, kw_type))
            }
            _ => Err(self.easy_error("Unexpected command token".to_string())),
        }
    }
    
    fn parse_let(&mut self) -> Result<Statement<'a>, ErrorHandler> {
        // get variable name
        let name = self.get_name()?;

        // check if = exist
        if self.next() != Some(Token::CmpOp(CmpOp::Equal)) {
            let kw_type = self.dialect.get_kw_word(KeyWordType::Let);
            return Err(self.hard_error("=".to_string(), kw_type));
        }

        // get the value, which will be stored in hashmap
        let value = self.expr_bp(0)?;
        Ok(Statement::Assign { name, value })
    }

    fn parse_print(&mut self) -> Result<Statement<'a>, ErrorHandler> {
        let statement = match self.next() {
            Some(Token::Literal(Literal::Text(text))) => {
                if let Some(Token::Semicolon) = self.peek() {
                    self.next(); // Consume ;
                    Statement::PrintStr(text, false) 
                } else {
                    Statement::PrintStr(text, true)
                }
            }
            Some(Token::Literal(Literal::Ident(name))) => {
                if let Some(Token::Semicolon) = self.peek() {
                    self.next(); // Consume ;
                    Statement::PrintVar(name, false)
                } else {
                    Statement::PrintVar(name, true)
                }
            }
            _ => return Err(self.easy_error("Expected string or variable after PRINT".to_string())),
        };
        Ok(statement)
    }

    fn parse_if(&mut self) -> Result<Statement<'a>, ErrorHandler> {
        // get left expression
        let left_value = self.expr_bp(0)?;
        
        // get operator (==, !=, <, >)
        let op_token = self.next();
        let cmp = match op_token {
            Some(Token::CmpOp(CmpOp::DoubleEqual)) => "=",
            Some(Token::CmpOp(CmpOp::NonEqual)) => "!",
            Some(Token::CmpOp(CmpOp::Less)) => "<",
            Some(Token::CmpOp(CmpOp::Greater)) => ">",
            Some(Token::CmpOp(CmpOp::LessEqual)) => "<=",
            Some(Token::CmpOp(CmpOp::GreaterEqual)) => ">=",
            _ => unreachable!(),
        };
        

        // get right expression
        let right_value = self.expr_bp(0)?;
        
        // check if keyword THEN exist
        if self.next() != Some(Token::KeyWord(KeyWordType::Then)) {
            let error_kw = self.dialect.get_kw_word(KeyWordType::Then);
            let kw_type = self.dialect.get_kw_word(KeyWordType::If);
            return Err(self.hard_error(error_kw, kw_type));
        }
       
        // Skip \n
        if let Some(Token::Newline) = self.peek() {
            self.next();
            
        }
        
        // Get the stop keywords to get the then_block before we meet ELSE
        let stop_keywords = vec![
            Token::KeyWord(KeyWordType::Else), 
            Token::KeyWord(KeyWordType::End),
        ];
        let then_block = self.parse_block(&stop_keywords)?;

        // Start parsing ELSE block
        let else_block = if let Some(Token::KeyWord(KeyWordType::Else)) = self.peek() {
            self.next(); // Consume Else

            if let Some(Token::Newline) = self.peek() {
                self.next();
            }

            let stop_keywords = vec![
                Token::KeyWord(KeyWordType::End),
                Token::KeyWord(KeyWordType::Wend),
            ];

            self.parse_block(&stop_keywords)?
        } else {
            Vec::new()
        };

        
        if self.next() != Some(Token::KeyWord(KeyWordType::End)) {
            let error_kw = self.dialect.get_kw_word(KeyWordType::End);
            let kw_type = self.dialect.get_kw_word(KeyWordType::If);
            return Err(self.hard_error(error_kw, kw_type));
        }
        
        
        if let Some(Token::Newline) = self.peek() {
            self.next();
        }


        Ok(Statement::If { 
            left_value, 
            cmp, 
            right_value,
            then_block,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<Statement<'a>, ErrorHandler> {
        // get left expression
        let left_value = self.expr_bp(0)?;

        // get operator (==, !=, <, >)
        let op_token = self.next();
        let cmp = match op_token {
            Some(Token::CmpOp(CmpOp::DoubleEqual)) => "=",
            Some(Token::CmpOp(CmpOp::NonEqual)) => "!",
            Some(Token::CmpOp(CmpOp::Less)) => "<",
            Some(Token::CmpOp(CmpOp::Greater)) => ">",
            Some(Token::CmpOp(CmpOp::LessEqual)) => "<=",
            Some(Token::CmpOp(CmpOp::GreaterEqual)) => ">=",
            _ => return Err(self.easy_error("Expected comparison operator in WHILE condition".to_string())),
        };

        // get right expression
        let right_value = self.expr_bp(0)?;

        if self.next() != Some(Token::KeyWord(KeyWordType::Then)) {
            let error_kw = self.dialect.get_kw_word(KeyWordType::Then);
            let kw_type = self.dialect.get_kw_word(KeyWordType::While);
            return Err(self.hard_error(error_kw, kw_type));
        }

        if let Some(Token::Newline) = self.peek() {
            self.next();
        }
        let stop_keyword = vec![Token::KeyWord(KeyWordType::Wend)];
        let block = self.parse_block(&stop_keyword)?;
        self.next(); // Consume WEND

        Ok(Statement::While { left_value, cmp, right_value, block })
    }

    fn parse_for(&mut self) -> Result<Statement<'a>, ErrorHandler> {
        let variable = self.get_name()?;
        if self.next() != Some(Token::CmpOp(CmpOp::Equal)) {
            return Err(self.easy_error("Expected operator =".to_string()));
        }
        let start_idx = self.get_num()?;

        if self.next() != Some(Token::KeyWord(KeyWordType::To)) {
            let error_kw = self.dialect.get_kw_word(KeyWordType::To);
            let kw_type = self.dialect.get_kw_word(KeyWordType::For);
            return Err(self.hard_error(error_kw, kw_type));
        }
        let end_idx = self.get_num()?;

        let step = if let Some(Token::KeyWord(KeyWordType::Step)) = self.peek() {
            self.next(); // Consume STEP
            if let Some(Token::OpType(OpType::Minus)) = self.peek() {
                self.next(); // Consume -
                -self.get_num()?
            } else {
                self.get_num()?
            }
        } else {
            1
        };
        
        if let Some(Token::Newline) = self.peek() {
            self.next();
        }

        let stop_keyword = vec![Token::KeyWord(KeyWordType::Next)];
        let block = self.parse_block(&stop_keyword)?;
        self.next(); // Consume NEXT

        Ok(Statement::For { 
            increment: variable,
            start_idx, 
            end_idx, 
            block, 
            step, })
    }

    fn parse_random(&mut self) -> Result<Statement<'a>, ErrorHandler> {
        // get name of varibale
        let name = self.get_name()?;
        
        // get min and max to set a borders
        let min = self.get_num()?;
        let max = self.get_num()?;

        Ok (Statement::Random { name, min, max })
    }


    // Expr_bp used for calculating math expressions in code
    // e.g LET X = 5 + (9 * 6)^2
    pub fn expr_bp(&mut self, min_bp: u8) -> Result<Expression<'a>, ErrorHandler> {
        let mut lhs = match self.next() {
            Some(Token::Literal(Literal::Number(num))) => Expression::Atom(num),
            Some(Token::OpType(OpType::LParen)) => {
                let lhs = self.expr_bp(0)?;
                if self.next() != Some(Token::OpType(OpType::RParen)) {
                    return Err(self.easy_error("Expected matching ')'".to_string()));
                }
                lhs
            }
            Some(Token::OpType(op_type)) => {
                match op_type {
                    OpType::Plus | OpType::Minus => {
                        let prefix_bp = self.prefix_bind_operator(op_type)
                            .map_err(|err_msg| self.easy_error(err_msg.to_string()))?;
                        
                        let rhs = self.expr_bp(prefix_bp)?;
                        Expression::UnCons(op_type, Box::new(rhs))
                    }
                    _ => return Err(self.easy_error("Unexpected operator inside expression".to_string())),
                }
            }
            Some(Token::Literal(Literal::Ident(name))) => Expression::Variable(name),
            Some(Token::Literal(Literal::Text(_))) => return Err(self.easy_error("Strings are not allowed in math expressions".to_string())),
            _ => return Err(self.easy_error("Expected number, variable or prefix operator".to_string())),
        }; 

        loop {
            let op = match self.peek() {
                Some(Token::OpType(op)) => *op,
                _ => break
            };

            if let Ok(postfix_bp) = self.postfix_bind_operator(op) {
                if postfix_bp < min_bp {
                    break;
                }
                self.next();

                lhs = Expression::UnCons(op, Box::new(lhs));
                continue;
            }

            if let Ok((left_power, right_power)) = self.infix_bind_operator(op) {
                if left_power < min_bp {
                    break;
                }
                self.next();
                let rhs = self.expr_bp(right_power)?;
                lhs = Expression::BinCons(op, Box::new(lhs), Box::new(rhs));
                continue;
            }
            break;
        }

        Ok(lhs)
    }
    // check prefix operator -5
    fn prefix_bind_operator(&self, op_type: OpType) -> Result<u8, &'static str> {
        match op_type {
            OpType::Plus | OpType::Minus => Ok(5),
            _ => Err("Wrong prefix operator"),
        }
    }

    // check postfix operator 5!
    fn postfix_bind_operator(&self, op_type: OpType) -> Result<u8, &'static str> {
        match op_type {
            OpType::Factorial => Ok(8),
            _ => Err("Wrong postfix operator"),
        }
    }
    // check infix operators 
    // E.g 5 + 6 or 7 * 9 or 9 / 3, etc.
    fn infix_bind_operator(&self, op_type: OpType) -> Result<(u8, u8), &'static str> {
        match op_type {
            OpType::Plus | OpType::Minus => Some((1, 2)),
            OpType::Multiply | OpType::Divide | OpType::Mod => Some((3, 4)),
            OpType::Power => Some((7, 6)),
            _ => None,
        }
        .ok_or("Wrong infix operator")
    }
}
