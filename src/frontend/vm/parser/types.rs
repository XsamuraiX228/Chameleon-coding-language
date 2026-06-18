#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Text,
    Int,
    Float,
    Bool,
}

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

