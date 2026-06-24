use std::fmt;
#[derive(Debug)]
pub enum ErrorKind {
    Lexical,
    Syntax,
    Runtime,
} 

#[derive(Debug)]
pub struct ErrorHandler {
    err_type: ErrorKind,
    error: String,
    line: usize,
}

impl ErrorHandler {
    pub fn new(err_type: ErrorKind, error: String, line: usize) -> Self {
        Self {err_type, error, line}
    }
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.err_type {
            ErrorKind::Lexical => "Lexical Error",
            ErrorKind::Syntax  => "Syntax Error",
            ErrorKind::Runtime => "Runtime Error",
        };
        write!(f, "{}: {}, line {}", prefix, self.error, self.line)
    }
}

pub fn hard_error(error_kw: String, kw_type: String, current_line: usize) -> ErrorHandler {
    let message = format!("Expected '{}' after {} block", error_kw, kw_type);
    ErrorHandler::new(ErrorKind::Syntax, message, current_line + 1)
}

pub fn easy_error(context: String, current_line: usize) -> ErrorHandler {
    ErrorHandler::new(ErrorKind::Syntax, context, current_line + 1)
}