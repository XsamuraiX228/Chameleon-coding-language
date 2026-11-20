use crate::dialect::SyntaxDict;
use crate::frontend::token::Token;
use crate::frontend::lexer_v2::lexer_v2_utils::utils::get_dialect;
use crate::diagnostic::diagnostic::{ErrorHandler, lexical_error};

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken<'a> {
    pub token: Token<'a>,
    pub current_line: usize,
    pub current_col: usize
}

impl<'a> SpannedToken<'a> {
    pub fn create_token(token: Token<'a>, current_line: usize, current_col: usize) -> SpannedToken<'a> {
        Self { token, current_line, current_col }
    }
}

pub struct LexerV2<'a> {
    pub raw_code: &'a str,
    pub config: SyntaxDict,
    pub current_line: usize,
    pub current_col: usize,
    pub chars: std::iter::Peekable<std::str::CharIndices<'a>>
}

impl<'a> LexerV2<'a>  {
    pub fn new(input: &'a str) -> Self {
        let (config, code, current_line) = get_dialect(input);
        Self {
            raw_code: code,
            config,
            current_line,
            current_col: 1,
            chars: code.char_indices().peekable()
        }
    }

    // Main function, which return a token
    pub fn next_token(&mut self) -> Result<SpannedToken<'a>, ErrorHandler> {
        // skip whitespaces, until we reach non-whitespace character
        self.skip_whitespace();

        // grab first non-whitespace character and match it to helper functions
        let Some((_, ch)) = self.peek() else {
            return Ok(SpannedToken { token: Token::EOF, current_line: self.current_line, current_col: self.current_col });
        };

        match ch {
            '\r' | '\n'                  => self.newline_token(),
            '0'..='9'                    => self.number_token(),
            '"'                          => self.string_token(),
            'a'..='z' | 'A'..='Z' | '_'  => self.keyword_token(),
            '=' | '<' | '>' | '!'        => self.comparative_tokens(),
            '+' | '-' | '*' | '^' | '%'  => self.math_tokens(),
            '/'                          => self.comment_tokens(),
            '(' | ')' | '{' | '}' | ';' | ':' | ',' => self.symbol_tokens(),
            unexpected_char => {
                return Err(lexical_error(unexpected_char.to_string(), "Unexpected char".to_string(), self.current_line, self.current_col));
            }
        }
    }

    pub fn tokenizer(&mut self) -> Result<(Vec<SpannedToken<'a>>, &SyntaxDict), ErrorHandler> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let done = token.token == Token::EOF;
            tokens.push(token);
            if done { break; }
        }
        Ok((tokens, &self.config))
    }

}

pub fn debug_tokens(tokens: &[SpannedToken]) {
    println!("\n=== DEBUG: Spanned Tokens ===\n");
    for (_, spanned) in tokens.iter().enumerate() {
        println!("Line {:3} | {:?}, Col: {:3}", spanned.current_line, spanned.token, spanned.current_col);
    }
    println!("\nTotal tokens: {}", tokens.len());
}
