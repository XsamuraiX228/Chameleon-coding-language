use crate::dialect::SyntaxDict;
use crate::frontend::token::Token;
use crate::frontend::lexer::get_dialect;


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
    pub fn next_token(&mut self) -> SpannedToken<'a> {
        // skip whitespaces, until we reach non-whitespace character
        self.skip_whitespace();

        // grab first non-whitespace character and match it to helper functions
        let Some((_, ch)) = self.peek() else {
            return SpannedToken { token: Token::EOF, current_line: self.current_line, current_col: self.current_col };
        };

        match ch {
            '\r' | '\n' => {
                // newline
                let token = self.newline_token().unwrap();
                return token;
            },
            '0'..='9' => {
                // numbers
                let token = self.number_token().unwrap();
                return token;
            },
            '"' => {
                // strings
                let token = self.string_token().unwrap();
                return token;
            }
            'a'..='z' | 'A'..='Z' => {
                // keywords, function words, indents.
                let token = self.keyword_token().unwrap();
                return token;
            }
            '=' | '<' | '>' | '!' => {
                let token = self.comparative_tokens().unwrap();
                return token;
            }
            '+' | '-' | '*' | '^' | '%' => {
                let token = self.math_tokens().unwrap();
                return token;
            }
            '/' => {
                let token = self.comment_tokens().unwrap();
                return token;
            }
            '(' | ')' | '{' | '}' | ';' | ':' | ',' => {
                let token = self.symbol_tokens().unwrap();
                return token
            }
            _ => {
                println!("")
            }
        }
    

        SpannedToken::create_token(Token::EOF, self.current_line, self.current_col)
    }


    pub fn tokenizer(&mut self) -> (Vec<SpannedToken<'a>>, &SyntaxDict) {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token.token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        let config = &self.config;
        (tokens, config)
    }

}

pub fn debug_tokens(tokens: &[SpannedToken]) {
    println!("\n=== DEBUG: Spanned Tokens ===\n");
    for (_, spanned) in tokens.iter().enumerate() {
        println!("Line {:3} | {:?}, Col: {:3}", spanned.current_line, spanned.token, spanned.current_col);
    }
    println!("\nTotal tokens: {}", tokens.len());
}
