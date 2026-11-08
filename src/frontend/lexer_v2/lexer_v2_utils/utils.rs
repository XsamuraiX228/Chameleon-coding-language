use crate::frontend::token::Literal;
use crate::frontend::token::Token;
use crate::frontend::lexer_v2::lexer_v2::LexerV2;

impl<'a> LexerV2<'a> {
    // Pick the symbol
    pub fn peek(&mut self) -> Option<(usize, char)> {
       self.chars.peek().copied()
    }

    // Consume the symbol and increment the column and line
    pub fn next(&mut self) -> Option<(usize, char)> {
        let (byte, ch) = self.chars.next()?;
        match ch {
            '\n' => {
                self.current_col = 1;
                self.current_line += 1;
            }
            '\r' => {
                if matches!(self.chars.peek(), Some((_, '\n'))) {
                    self.current_col += 1;
                } else {
                    self.current_line += 1;
                    self.current_col = 1;
                }
            }
            _ => self.current_col += 1
        }
        Some((byte, ch))
    }

    pub fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some((_, ' ' | '\t'))) {
            self.next();
        }
    }
    
    /// Consumes consecutive ASCII digits (0-9) and stops at the first
    /// non-digit character or at the end of input.
    pub fn consume_number(&mut self) {
        while matches!(self.peek(), Some((_, '0'..='9'))) {
            self.next();
        }
    }

    /// Consumes an identifier or keyword: letters and underscores.
    /// Stops at the first character that cannot be part of a word.
    pub fn consume_word(&mut self) {
        while let Some((_, c)) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.next();
            } else {
                break;
            }
        }
    }

    /// Looks ahead without consuming anything: returns true if the current
    /// character (a '.') is followed by a digit, like in "3.14".
    /// It works on a clone of the iterator, so the real position is unchanged.
    pub fn digit_after_dot(&self) -> bool {
        let mut after_dot = self.chars.clone();
        after_dot.next(); // skip the '.' in the cloned iterator
        matches!(after_dot.peek(), Some((_, '0'..='9')))
    }

    /// Decides what a word that is not a keyword means: a bool literal
    /// (from the config) or a plain identifier.
    pub fn classify_literal(&self, keyword: &'a str) -> Token<'a> {
        match self.config.bool_keywords.get(keyword) {
            Some(&b) => Token::Literal(Literal::Bool(b)),
            None => Token::Literal(Literal::Ident(keyword)),
        }
    }

    /// Consumes the next character only if it equals `expected`.
    /// Returns true if it was consumed, false if nothing changed.
    pub fn consume_char(&mut self, expected: char) -> bool {
        if matches!(self.peek(), Some((_, c)) if c == expected) {
            self.next();
            return true
        } else {
            return false;
        }
    }

    /// Classify the character if it has multipile variants
    /// Returns token if matched one of variants and default if not
    pub fn classify_char(&mut self, options: &[(char, Token<'a>)], default: Token<'a>) -> Token<'a> {
        for (c, token) in options {
            if self.consume_char(*c) {
                return token.clone();
            }
        }
        default
    }

    pub fn skip_until_newline(&mut self) {
        while let Some((_, c)) = self.peek() {
            if c == '\n' || c == '\r' {
                break;
            }
            self.next();
        }
    }
}

