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
        write!(f, "[{}] on line {}: {}", prefix, self.line, self.error)
    }
}