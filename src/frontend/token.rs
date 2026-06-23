pub const VALID_OPERATORS: [char; 5] = ['*', '%', '^', '(', ')'];
use crate::frontend::vm::compiler::opcodes::Opcode;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuncKeyWord {
    Sin,
    Cos,
    Abs,
    Sqrt,
    Random,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuncOp {
    Arrow,
    OpenCurly,
    CloseCurly,
    Colon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyWordType {
    Let,
    Print,
    Println,
    Input,
    If,
    Then,
    Else,
    Goto,
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
    Func,
    Return,
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
    Increment,
    IncEqual,
    Decrement,
    DecEqual,
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
    Number(i64),  // i64 number
    Text(&'a str),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarType {
    Int,
    Float,
    Bool,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {   
    KeyWord(KeyWordType),
    FuncWord(FuncKeyWord),
    UserFunc(FuncOp),
    OpType(OpType),
    CmpOp(CmpOp),
    Literal(Literal<'a>),
    VarType(VarType),
    Mark(&'a str), // e.g :loop 
    Semicolon,
    Comma,
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
            OpType::Increment => "++",
            OpType::Decrement => "--",
            OpType::IncEqual => "+=",
            OpType::DecEqual => "-=",
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
            KeyWordType::Println => "PRINTLN",
            KeyWordType::Input => "INPUT",
            KeyWordType::If => "IF",
            KeyWordType::Then => "THEN",
            KeyWordType::Else => "ELSE",
            KeyWordType::Goto => "GOTO",
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
            KeyWordType::Func => "FUNC",
            KeyWordType::Return => "RETURN",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for FuncKeyWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FuncKeyWord::Sin => "SIN",
            FuncKeyWord::Cos => "COS",
            FuncKeyWord::Abs => "ABS",
            FuncKeyWord::Max => "MAX",
            FuncKeyWord::Min => "MIN",
            FuncKeyWord::Random => "RANDOM",
            FuncKeyWord::Sqrt => "SQRT",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for FuncOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FuncOp::Arrow => "ARROW",
            FuncOp::OpenCurly => "{",
            FuncOp::CloseCurly => "}",
            FuncOp::Colon => ":",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            VarType::Int => "INT",
            VarType::Float => "FLOAT",
            VarType::Bool => "BOOL",
            VarType::String => "STRING"
        };
        write!(f, "{}", s)
    }
}


impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::KeyWord(kw) => write!(f, "KW({})", kw),
            Token::FuncWord(fw) => write!(f, "FW({}", fw),
            Token::UserFunc(uw) => write!(f, "UF{}", uw),
            Token::OpType(op) => write!(f, "Op({})", op),
            Token::CmpOp(cmp) => write!(f, "Cmp({})", cmp),
            Token::Literal(lit) => write!(f, "{}", lit),
            Token::VarType(vt) => write!(f, "{}", vt),
            Token::Mark(name) => write!(f, "Mark(:{})", name),
            Token::Newline => write!(f, "Newline"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Comma => write!(f, "Comma"),
            Token::EOF => write!(f, "EOF"),
            Token::Unexpected(ch) => write!(f, "Token(:{})", ch)
        }
    }
}