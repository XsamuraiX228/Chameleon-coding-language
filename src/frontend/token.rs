pub const VALID_OPERATORS: [char; 7] = ['+', '-', '*', '%', '^', '(', ')'];
use super::vmparser::Opcode;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyWordType {
    Let,
    Print,
    Input,
    If,
    Then,
    Else,
    Goto,
    Random,
    While,
    Wend,
    For,
    To,
    Step,
    Next,
    End,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    Power,
    Factorial,
    LParen,
    RParen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Equal, // =
    DoubleEqual, // ==
    NonEqual, // !=
    LessEqual, // <=
    GreaterEqual, // >=
    Less, // <
    Greater, // >
}

impl OpType {
    pub fn to_opcode(&self) -> Opcode {
        match self {
            OpType::Plus => Opcode::Add,
            OpType::Minus => Opcode::Sub,
            OpType::Multiply => Opcode::Mul,
            OpType::Divide => Opcode::Div,
            OpType::Mod => Opcode::Mod,
            OpType::Power => Opcode::Pow,
            _ => {
                println!("Error, Math");
                unreachable!()
            },
        }
    }
}

impl CmpOp {
    pub fn to_opcode(&self) -> Opcode {
        match self {
            CmpOp::DoubleEqual => Opcode::Equal,
            CmpOp::Less => Opcode::Less,
            CmpOp::Greater => Opcode::Greater,
            CmpOp::NonEqual => Opcode::NotEqual,
            CmpOp::GreaterEqual => Opcode::GreaterEq,
            CmpOp::LessEqual => Opcode::LessEq,
            _ => {
                println!("Error, Compare");
                unreachable!()
            },
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Literal<'a> {
    Ident(&'a str), // simple string
    Text(&'a str),
    Number(i64),  // i64 number
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {   
    KeyWord(KeyWordType),
    OpType(OpType),
    CmpOp(CmpOp),
    Literal(Literal<'a>),
    Mark(&'a str), // e.g :loop 
    Semicolon,
    Newline, // \n
    EOF,
    Unexpected(char),
}

// token.rs
use std::fmt;

impl fmt::Display for OpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            OpType::Plus => "+",
            OpType::Minus => "-",
            OpType::Multiply => "*",
            OpType::Divide => "/",
            OpType::Mod => "%",
            OpType::Power => "^",
            OpType::Factorial => "!",
            OpType::LParen => "(",
            OpType::RParen => ")",
        };
        write!(f, "{}", s)
    }
}
impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            CmpOp::Equal => "=",
            CmpOp::DoubleEqual => "==",
            CmpOp::NonEqual => "!=",
            CmpOp::LessEqual => "<=",
            CmpOp::GreaterEqual => ">=",
            CmpOp::Less => "<",
            CmpOp::Greater => ">",
        };
        write!(f, "{}", s)
    }
}
impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Ident(name) => write!(f, "Ident({})", name),
            Literal::Text(text) => write!(f, "Text(\"{}\")", text),
            Literal::Number(n) => write!(f, "Number({})", n),
        }
    }
}
impl fmt::Display for KeyWordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            KeyWordType::Let => "LET",
            KeyWordType::Print => "PRINT",
            KeyWordType::Input => "INPUT",
            KeyWordType::If => "IF",
            KeyWordType::Then => "THEN",
            KeyWordType::Else => "ELSE",
            KeyWordType::Goto => "GOTO",
            KeyWordType::Random => "RANDOM",
            KeyWordType::While => "WHILE",
            KeyWordType::Wend => "WEND",
            KeyWordType::For => "FOR",
            KeyWordType::To => "TO",
            KeyWordType::Step => "STEP",
            KeyWordType::Next => "NEXT",
            KeyWordType::End => "END",
            KeyWordType::And => "AND",
            KeyWordType::Or => "OR",
            KeyWordType::Not => "NOT"
        };
        write!(f, "{}", s)
    }
}
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::KeyWord(kw) => write!(f, "KW({})", kw),
            Token::OpType(op) => write!(f, "Op({})", op),
            Token::CmpOp(cmp) => write!(f, "Cmp({})", cmp),
            Token::Literal(lit) => write!(f, "{}", lit),
            Token::Mark(name) => write!(f, "Mark(:{})", name),
            Token::Newline => write!(f, "Newline"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::EOF => write!(f, "EOF"),
            Token::Unexpected(ch) => write!(f, "Token(:{})", ch)
        }
    }
}