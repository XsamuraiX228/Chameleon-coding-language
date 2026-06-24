#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    String,
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CallType {
    Sin = 0,
    Cos = 1,
    AbsInt = 2,
    AbsFloat = 3,
    MinInt = 4,
    MinFloat = 5,
    MaxInt = 6,
    MaxFloat = 7,
    Sqrt = 8,
    RandomInt = 9,
    RandomFloat = 10,
}

#[derive(Debug)]
pub struct VarInfo<'a> {
    pub name: &'a str,
    pub data_type: DataType,
}

#[derive(PartialEq, Debug)]
pub enum Constants {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

