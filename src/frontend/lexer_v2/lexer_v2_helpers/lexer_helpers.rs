use crate::diagnostic::diagnostic::ErrorHandler;
use crate::frontend::token::Literal;
use crate::frontend::token::{CmpOp, OpType, Token, FuncOp};
use crate::diagnostic::diagnostic::lexical_error;
use crate::frontend::lexer_v2::lexer_v2::{LexerV2, SpannedToken};

type LexerResultType<'a> = Result<SpannedToken<'a>, ErrorHandler>;

impl<'a> LexerV2<'a> {
    pub fn newline_token(&mut self) -> LexerResultType<'a> {
        // save the position of the start line and column
        let start_line = self.current_line;
        let start_col = self.current_col;
        // Pick first symbol, using next and automatically increase the column and line
        match self.peek() {
            // If the symbol is \r, we consume it and check if there is a \n after it
            Some((_, '\r')) => {
                self.next();
                if matches!(self.peek(), Some((_, '\n'))) {
                    self.next();
                }
            },
            // If the symbol is \n we simply consume it and return Newline Token
            Some((_, '\n')) => {
                self.next();
            },
            // If the symbol is a random character, throw an Error
            Some((_, ch)) => {
                return Err(lexical_error(ch.to_string(), "newline".to_string(), self.current_line));
            },
            _ => {
                return Err(lexical_error("EOF".to_string(), "newline".to_string(), self.current_line));
            }
        }
        Ok(SpannedToken::create_token(Token::Newline, start_line, start_col))
    }

    pub fn number_token(&mut self) -> LexerResultType<'a> {
        // save the position of the start line and column
        let (start_line, start_col) = (self.current_line, self.current_col);

        // get the start position of number
        let (start, _) = self.peek().unwrap();

        // Start gathering number
        self.consume_number();

        // Check if we meet a dot
        let mut is_float = false;
        if matches!(self.peek(), Some((_, '.'))) && self.digit_after_dot() {
            is_float = true;
            self.next(); // consume . in main Iterator
            self.consume_number();
        }

        // get the end posistion of a number
        let end = self.peek().map_or(self.raw_code.len(), |(i, _)| i);
        let text_number = &self.raw_code[start..end];

        // check if number has non-digit character
        if let Some((_, ch)) = self.peek()  {
            if ch.is_ascii_alphabetic() || ch == '_' {
                return Err(lexical_error(ch.to_string(), "digit (0-9)".to_string(), self.current_line));
            }
        }

        // parse the text to number and check for overflow
        let token = match is_float {
            true => {
                let number = text_number.parse::<f64>().unwrap();
                Token::Literal(Literal::Float(number))
            },
            false => {
                let number = text_number.parse::<i64>().unwrap();
                Token::Literal(Literal::Int(number))
            }
        };

        return Ok(SpannedToken::create_token(token, start_line, start_col))
    }

    pub fn string_token(&mut self) -> LexerResultType<'a> {
        // save the position of the start line and column
        let (start_line, start_col) = (self.current_line, self.current_col);
        self.next(); // consume start bracket

        // get the start position of a string
        let (start_quote, _) = self.peek().unwrap();

        loop {
            match self.peek() {
                Some((end, '"')) => {
                    self.next();
                    let string = &self.raw_code[start_quote..end];
                    return Ok(SpannedToken::create_token(Token::Literal(Literal::Text(string)), start_line, start_col));
                },
                Some((_, '\r' | '\n')) => {
                    return Err(lexical_error("newline".to_string(), "letter".to_string(), self.current_line));
                },
                _ => {
                    self.next();
                }
            }
        }
    }

    pub fn keyword_token(&mut self) -> LexerResultType<'a> {
        // save the position of the start line and column
        let (start_line, start_col) = (self.current_line, self.current_col);

        // get the start position of a keyword
        let (start, _) = self.peek().unwrap();

        // consume keyword token
        self.consume_word();

        let end = self.peek().map_or(self.raw_code.len(), |(i, _)| i);

        let keyword = &self.raw_code[start..end];

        // match the keyword to three maps of keywords for different purposes
        // if non of them matched classify the keyword as a bool token or indent
        let token = if let Some(&kw) = self.config.keywords.get(keyword) {
            Token::KeyWord(kw)
        } else if let Some(&fw) = self.config.func_keywords.get(keyword) {
            Token::FuncWord(fw)
        } else if let Some(&vt) = self.config.type_keywords.get(keyword) {
            Token::VarType(vt)
        } else {
            self.classify_literal(keyword)
        };

        return Ok(SpannedToken::create_token(token, start_line, start_col));
    }

    pub fn comparative_tokens(&mut self) -> LexerResultType<'a> {
        // save the position of the start line and column
        let (start_line, start_col) = (self.current_line, self.current_col);

        // get the first token to check what kind of comparative token it is
        let (_, first) = self.next().expect("Expected comparative token, got EOF at function comparative_tokens");

        // check if there is an = after our first token, so we change the whole token's name
        let has_eq = self.consume_char('=');

        let token = match (first, has_eq) {
            ('=', false) => Token::CmpOp(CmpOp::Equal),
            ('=', true)  => Token::CmpOp(CmpOp::DoubleEqual),
            ('<', false) => Token::CmpOp(CmpOp::Less),
            ('<', true)  => Token::CmpOp(CmpOp::LessEqual),
            ('>', false) => Token::CmpOp(CmpOp::Greater),
            ('>', true)  => Token::CmpOp(CmpOp::GreaterEqual),
            ('!', true)  => Token::CmpOp(CmpOp::NonEqual),
            ('!', false) => Token::OpType(OpType::Factorial),
            _ => unreachable!(),
        };

        Ok(SpannedToken::create_token(token, start_line, start_col))
    }

    pub fn math_tokens(&mut self) -> LexerResultType<'a> {
        // save the position of the start line and column
        let (start_line, start_col) = (self.current_line, self.current_col);

        // get the first token to check what kind of math token it is
        let (_, first) = self.next().expect("Expected math token, got EOF at function math_tokens");

        // math the first token to check if it has other tokens or it's single
        // if token may has other tokens in pair, we use "classify_char" to check what token our first token have in pair
        let token = match first {
            '*' => Token::OpType(OpType::Multiply),
            '%' => Token::OpType(OpType::Mod),
            '^' => Token::OpType(OpType::Power),
            '+' => self.classify_char(
                    &[('+', Token::OpType(OpType::Increment)), 
                               ('=', Token::OpType(OpType::IncEqual))], 
                    Token::OpType(OpType::Plus)),
            '-' => self.classify_char(
                    &[('-', Token::OpType(OpType::Decrement)),
                               ('>', Token::UserFunc(FuncOp::Arrow)),
                               ('=', Token::OpType(OpType::DecEqual))], 
                    Token::OpType(OpType::Minus)),
            _ => unreachable!(),
        };

        Ok(SpannedToken::create_token(token, start_line, start_col))
    }

    pub fn comment_tokens(&mut self) -> LexerResultType<'a> {
        // save the position of the start line and column
        let (start_line, start_col) = (self.current_line, self.current_col);

        self.next();

        if self.consume_char('/') {
            self.skip_until_newline();
            return self.newline_token();
        }

        Ok(SpannedToken::create_token(Token::OpType(OpType::Divide), start_line, start_col))
    }

    pub fn symbol_tokens(&mut self) -> LexerResultType<'a> {
        let (start_line, start_col) = (self.current_line, self.current_col);
        let (_, ch) = self.next().expect("Expected symbol, got EOF at function symbol_tokens");
    
        let token = match ch {
            '(' => Token::OpType(OpType::LParen),
            ')' => Token::OpType(OpType::RParen),
            '{' => Token::UserFunc(FuncOp::OpenCurly),
            '}' => Token::UserFunc(FuncOp::CloseCurly),
            ';' => Token::Semicolon,
            ':' => Token::UserFunc(FuncOp::Colon),
            ',' => Token::Comma,
            _ => unreachable!(),
        };
    
        Ok(SpannedToken::create_token(token, start_line, start_col))
    }

}
