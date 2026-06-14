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
    pub fn to_opcode_int(&self) -> Opcode {
        match self {
            OpType::Plus => Opcode::IAdd,
            OpType::Minus => Opcode::ISub,
            OpType::Multiply => Opcode::IMul,
            OpType::Divide => Opcode::IDiv,
            OpType::Mod => Opcode::IMod,
            OpType::Power => Opcode::IPow,
            _ => {
                println!("Error, Int Math");
                unreachable!()
            },
        }
    }
    pub fn to_opcode_float(&self) -> Opcode {
        match self {
            OpType::Plus => Opcode::FAdd,
            OpType::Minus => Opcode::FPow,
            OpType::Multiply => Opcode::FMul,
            OpType::Divide => Opcode::FDiv,
            OpType::Mod => Opcode::FMod,
            OpType::Power => Opcode::FPow,
            _ => {
                println!("Error, Float Math");
                unreachable!()
            },
        }
    }
}

impl CmpOp {
    pub fn to_opcode_int(&self) -> Opcode {
        match self {
            CmpOp::DoubleEqual => Opcode::IEqual,
            CmpOp::NonEqual => Opcode::INotEqual,
            CmpOp::Less => Opcode::ILess,
            CmpOp::LessEqual => Opcode::ILessEq,
            CmpOp::Greater => Opcode::IGreater,
            CmpOp::GreaterEqual => Opcode::IGreaterEq,
            _ => {
                println!("Error, Int Compare");
                unreachable!()
            },
        }
    }
    pub fn to_opcode_float(&self) -> Opcode {
        match self {
            CmpOp::DoubleEqual => Opcode::FEqual,
            CmpOp::NonEqual => Opcode::FNotEqual,
            CmpOp::Less => Opcode::FLess,
            CmpOp::LessEqual => Opcode::FLessEq,
            CmpOp::Greater => Opcode::FGreater,
            CmpOp::GreaterEqual => Opcode::FGreaterEq,
            _ => {
                println!("Error, Int Compare");
                unreachable!()
            },
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'a> {
    Ident(&'a str), // simple string
    Text(&'a str),
    Number(i64),  // i64 number
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
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
            Literal::Int(n) => write!(f, "Integer({})", n),
            Literal::Float(n) => write!(f, "Float({})", n),
            Literal::Bool(n) => write!(f, "Bool({})", n),
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
            KeyWordType::Not => "NOT",
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