#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    Int,
    Str,
    Void,
}

impl DataType {
    pub fn to_mnemonic(self) -> &'static str {
        match self {
            DataType::Int => "i64",
            _ => "void",
        }
    }
}