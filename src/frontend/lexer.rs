use crate::frontend::token::VALID_OPERATORS;
use crate::frontend::token::{CmpOp, OpType, Token};
use crate::dialect::SyntaxDict;
use crate::frontend::token::Literal;

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub line: usize,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    config: &'a SyntaxDict,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, config: &'a SyntaxDict, number: usize) -> Self {
        Self {
            input: input,
            pos: 0,
            config,
            current_line: number,
        }
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.input.as_bytes();
        while self.pos < bytes.len() && (bytes[self.pos] == b' ' || bytes[self.pos] == b'\t') {
            self.pos += 1;
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.pos >= self.input.len() {
            return None;
        }
        // Безопасно берем следующий символ, учитывая UTF-8
        self.input[self.pos..].chars().next()
    }

    fn next_token(&mut self) -> SpannedToken<'a> {
        self.skip_whitespace();

        if self.pos > self.input.len() {
            return SpannedToken {
                token: Token::EOF,
                line: self.current_line,
            };
        }

        let ch = match self.current_char() {
            Some(c) => c,
            None => {
                return SpannedToken {
                    token: Token::EOF,
                    line: self.current_line,
                };
            }
        };

        // check specila symbols \r , \n, ;
        if let Some(spanned_newline) = self.newline_symbols(ch) {
            return spanned_newline;
        }

        // check comparative symbols =, ==, !=, <, >, <=, >=
        if let Some(spanned_cmp) = self.comparative_symbols(ch) {
            return spanned_cmp;
        }

        // check math symbols in VALID_OPERATORS
        if let Some(spanned_math) = self.math_symbols(ch) {
            return spanned_math;
        }

        // check // symbol for comments
        if let Some(spanned_comments) = self.comments_symbol(ch) {
            return spanned_comments;
        }

        // check for numbers
        if let Some(spanned_num) = self.number(ch) {
            return spanned_num;
        }

        // check for string
        if let Some(spanned_string) = self.string(ch) {
            return spanned_string;
        }

        // check for Keywords
        if let Some(spanned_keyword) = self.keyword(ch) {
            return spanned_keyword;
        }

        let unexpected = SpannedToken {
            token: Token::Unexpected(ch),
            line: self.current_line,
        };

        // Продвигаем курсор вперед, чтобы не зациклиться на ошибке
        self.pos += ch.len_utf8();

        unexpected
    }

    fn newline_symbols(&mut self, ch: char) -> Option<SpannedToken<'a>> {
        let token_line = self.current_line;
        match ch {
            '\r' => {
                self.pos += 1;
                let bytes = self.input.as_bytes();
                if self.pos < self.input.len() && bytes[self.pos] == b'\n' {
                    self.pos += 1; // \r\n 
                }
                self.current_line += 1;
                Some(SpannedToken {
                    token: Token::Newline,
                    line: token_line,
                })
            }
            '\n' => {
                self.pos += 1;
                self.current_line += 1;
                Some(SpannedToken {
                    token: Token::Newline,
                    line: token_line,
                })
            }
            ';' => {
                self.pos += 1;
                Some(SpannedToken {
                    token: Token::Semicolon,
                    line: token_line,
                })
            }
            _ => return None,
        }
    }

    fn comparative_symbols(&mut self, ch: char) -> Option<SpannedToken<'a>> {
        let token_line = self.current_line;
        let bytes = self.input.as_bytes();
        match ch {
            '=' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::DoubleEqual)
                } else {
                    self.pos += 1;
                    Token::CmpOp(CmpOp::Equal)
                };
                Some(SpannedToken {
                    token,
                    line: token_line,
                })
            }
            '!' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::NonEqual)
                } else {
                    self.pos += 1;
                    Token::OpType(OpType::Factorial)
                };
                Some(SpannedToken {
                    token,
                    line: token_line,
                })
            }
            '<' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::LessEqual)
                } else {
                    self.pos += 1;
                    Token::CmpOp(CmpOp::Less)
                };
                Some(SpannedToken {
                    token,
                    line: token_line,
                })
            }
            '>' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::GreaterEqual)
                } else {
                    self.pos += 1;
                    Token::CmpOp(CmpOp::Greater)
                };
                Some(SpannedToken {
                    token,
                    line: token_line,
                })
            }
            _ => return None,
        }
    }

    fn math_symbols(&mut self, ch: char) -> Option<SpannedToken<'a>> {
        let token_line = self.current_line;
        let bytes = self.input.as_bytes();
        match ch {
            op if VALID_OPERATORS.contains(&op) => {
                self.pos += 1;
                let op_type = match ch {
                    '*' => OpType::Multiply,
                    '%' => OpType::Mod,
                    '^' => OpType::Power,
                    '(' => OpType::LParen,
                    ')' => OpType::RParen,
                    _ => unreachable!(),
                };
                Some(SpannedToken {
                    token: Token::OpType(op_type),
                    line: token_line,
                })
            }
            '+' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'+' {
                    self.pos += 2;
                    Token::OpType(OpType::Increment)
                } else if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::OpType(OpType::IncEqual)
                } else {
                    self.pos += 1;
                    Token::OpType(OpType::Plus)
                };
                Some(SpannedToken {
                    token,
                    line: token_line,
                })
            }
            '-' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'-' {
                    self.pos += 2;
                    Token::OpType(OpType::Decrement)
                } else if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::OpType(OpType::DecEqual)
                } else {
                    self.pos += 1;
                    Token::OpType(OpType::Minus)
                };
                Some(SpannedToken {
                    token,
                    line: token_line,
                })
            }
            _ => return None,
        }
    }

    fn comments_symbol(&mut self, ch: char) -> Option<SpannedToken<'a>> {
        let token_line = self.current_line;
        let bytes = self.input.as_bytes();

        match ch {
            '/' => {
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'/' {
                    self.pos += 2;
                    while self.pos < bytes.len() && bytes[self.pos] != b'\n' {
                        self.pos += 1;
                    }
                    return Some(self.next_token());
                }
                self.pos += 1;
                Some(SpannedToken {
                    token: Token::OpType(OpType::Divide),
                    line: token_line,
                })
            }
            _ => return None,
        }
    }

    fn number(&mut self, ch: char) -> Option<SpannedToken<'a>> {
        if !ch.is_ascii_digit() {
            return None;
        }

        let mut is_float = false;
        let token_line = self.current_line;
        let bytes = self.input.as_bytes();
        let start = self.pos;

        while self.pos < bytes.len()
            && ((bytes[self.pos] as char).is_ascii_digit() || bytes[self.pos] == b'.')
        {
            if bytes[self.pos] == b'.' {
                is_float = true;
            }
            self.pos += 1;
        }

        let num_str = &self.input[start..self.pos];
        if !is_float {
            let int_number = num_str.parse::<i64>().unwrap();
            return Some(SpannedToken {
                token: Token::Literal(Literal::Int(int_number)),
                line: token_line,
            });
        } else {
            let float_number = num_str.parse::<f64>().unwrap();
            return Some(SpannedToken {
                token: Token::Literal(Literal::Float(float_number)),
                line: token_line,
            });
        }
    }

    fn string(&mut self, ch: char) -> Option<SpannedToken<'a>> {
        if ch != '"' {
            return None;
        }

        let token_line = self.current_line;
        let bytes = self.input.as_bytes();

        self.pos += 1;
        let start = self.pos;

        while self.pos < bytes.len() && bytes[self.pos] != b'"' {
            self.pos += 1;
        }

        let text_str = &self.input[start..self.pos];
        self.pos += 1;

        Some(SpannedToken {
            token: Token::Literal(Literal::Text(text_str)),
            line: token_line,
        })
    }

    //
    fn skip_until_delimiter(&mut self) {
        let bytes = self.input.as_bytes();
        while self.pos < bytes.len() {
            if let Some(current_char) = self.input[self.pos..].chars().next() {
                if current_char.is_whitespace()
                    || current_char == '='
                    || current_char == '!'
                    || current_char == ';'
                    || current_char == ':'
                    || current_char == '+'
                    || current_char == '-'
                    || VALID_OPERATORS.contains(&current_char)
                {
                    break;
                }
                self.pos += current_char.len_utf8();
            }
        }
    }

    fn keyword(&mut self, ch: char) -> Option<SpannedToken<'a>> {
        let token_line = self.current_line;
        match ch {
            _ => {
                let start = self.pos;
                self.skip_until_delimiter();
                let word_str = &self.input[start..self.pos];
                // println!("{}", word_str);
                // println!("{:?}", self.config.keywords.get(word_str));
                let key_token = if let Some(kw_type) = self.config.keywords.get(word_str) {
                    Token::KeyWord(kw_type.clone())
                } else {
                    let type_token = match word_str {
                        "TRUE" => Token::Literal(Literal::Bool(true)),
                        "FALSE" => Token::Literal(Literal::Bool(false)),
                        _ => Token::Literal(Literal::Ident(word_str)),
                    };
                    return Some(SpannedToken {
                        token: type_token,
                        line: token_line,
                    });
                };
                Some(SpannedToken {
                    token: key_token,
                    line: token_line,
                })
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<SpannedToken<'a>> {
        let mut tokens = Vec::new();

        loop {
            let spanned = self.next_token();
            if let Token::EOF = spanned.token {
                tokens.push(spanned);
                break;
            }
            tokens.push(spanned);
        }
        tokens
    }
    // This function is used for debug only
    pub fn debug_tokens(&mut self) {
        let tokens = self.tokenize();
        println!("\n=== DEBUG: Spanned Tokens ===\n");
        for (i, spanned) in tokens.iter().enumerate() {
            println!("{:3} | Line {:3} | {:?}", i, spanned.line, spanned.token);
        }
        println!("\nTotal tokens: {}", tokens.len());
    }
}
