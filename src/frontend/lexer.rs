use crate::dialect::{SyntaxDict};
use super::token::{Token, CmpOp, OpType, Literal};
use super::token::VALID_OPERATORS;

#[derive(Debug, Clone, PartialEq, Eq)]
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
            current_line: number
        }
    }

    fn next_token(&mut self) -> Option<SpannedToken<'a>> {
        let bytes = self.input.as_bytes();

        while self.pos < self.input.len() && bytes[self.pos] == b' ' {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return None;
        }

        let ch = bytes[self.pos] as char;
        let token_line = self.current_line;

        match ch {
            // ==================== Newline tokens ====================
            '\r' => {
                self.pos += 1;
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

            // ==================== Operators ====================
            '=' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::DoubleEqual)
                } else {
                    self.pos += 1;
                    Token::CmpOp(CmpOp::Equal)
                };
                Some(SpannedToken { token, line: token_line })
            }
            '!' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::NonEqual)
                } else {
                    self.pos += 1;
                    Token::OpType(OpType::Factorial)
                };
                Some(SpannedToken { token, line: token_line })
            }
            '<' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::LessEqual)
                } else {
                    self.pos += 1;
                    Token::CmpOp(CmpOp::Less)
                };
                Some(SpannedToken { token, line: token_line })
            }
            '>' => {
                let token = if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'=' {
                    self.pos += 2;
                    Token::CmpOp(CmpOp::GreaterEqual)
                } else {
                    self.pos += 1;
                    Token::CmpOp(CmpOp::Greater)
                };
                Some(SpannedToken { token, line: token_line })
            }

            op if VALID_OPERATORS.contains(&op) => {
                self.pos += 1;
                let op_type = match ch {
                    '+' => OpType::Plus,
                    '-' => OpType::Minus,
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

            // ==================== Strings ====================
            '"' => {
                self.pos += 1;
                let start = self.pos;

                while self.pos < bytes.len() && bytes[self.pos] != b'"' {
                    self.pos += 1;
                }

                let text_str = &self.input[start..self.pos];
                self.pos += 1; // пропускаем закрывающую "

                Some(SpannedToken {
                    token: Token::Literal(Literal::Text(text_str)),
                    line: token_line,
                })
            }

            // ==================== Comments ====================
            '/' => {
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'/' {
                    self.pos += 2;
                    while self.pos < bytes.len() && bytes[self.pos] != b'\n' {
                        self.pos += 1;
                    }
                    return self.next_token();
                }
                self.pos += 1;
                Some(SpannedToken {
                    token: Token::OpType(OpType::Divide),
                    line: token_line,
                })
            }

            // ==================== Marks (:mark) ====================
            ':' => {
                self.pos += 1;
                let start = self.pos;
                while self.pos < bytes.len() {
                    if let Some(current_char) = self.input[self.pos..].chars().next() {
                        if current_char.is_whitespace()
                            || current_char == '='
                            || current_char == '!'
                            || VALID_OPERATORS.contains(&current_char)
                        {
                            break;
                        }
                        self.pos += current_char.len_utf8();
                    }
                }
                Some(SpannedToken {
                    token: Token::Mark(&self.input[start..self.pos]),
                    line: token_line,
                })
            }

            // ==================== Semicolon ====================
            ';' => {
                self.pos += 1;
                Some(SpannedToken {
                    token: Token::Semicolon,
                    line: token_line,
                })
            }

            // ==================== Numbers ====================
            '0'..='9' => {
                let start = self.pos;
                while self.pos < bytes.len() && (bytes[self.pos] as char).is_ascii_digit() {
                    self.pos += 1;
                }
                let num_str = &self.input[start..self.pos];
                let number = num_str.parse::<i64>().unwrap();

                Some(SpannedToken {
                    token: Token::Literal(Literal::Number(number)),
                    line: token_line,
                })
            }

            // ==================== KeyWords and Variables ====================
            _ => {
                let start = self.pos;
                while self.pos < bytes.len() {
                    if let Some(current_char) = self.input[self.pos..].chars().next() {
                        if current_char.is_whitespace()
                            || current_char == '='
                            || current_char == '!'
                            || current_char == ';'
                            || VALID_OPERATORS.contains(&current_char)
                        {
                            break;
                        }
                        self.pos += current_char.len_utf8();
                    }
                }

                let word_str = &self.input[start..self.pos];

                let token = if let Some(kw_type) = self.config.keywords.get(word_str) {
                    Token::KeyWord(kw_type.clone())
                } else {
                    Token::Literal(Literal::Ident(word_str))
                };

                Some(SpannedToken { token, line: token_line })
            }
        }
    }
    pub fn tokenize(&mut self) -> Vec<SpannedToken<'a>> {
        let mut tokens = Vec::new();
        while let Some(spanned) = self.next_token() {
            tokens.push(spanned);
        }
        tokens.reverse();
        tokens
    }
    // This function is used for debug only
    pub fn debug_tokens(&mut self) {
        let tokens = self.tokenize();
        println!("\n=== DEBUG: Spanned Tokens ===\n");
        for (i, spanned) in tokens.iter().enumerate() {
            println!(
                "{:3} | Line {:3} | {:?}",
                i,
                spanned.line,
                spanned.token
            );
        }
        println!("\nTotal tokens: {}", tokens.len());
    }
}