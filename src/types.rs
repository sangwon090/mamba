#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    Int,
    Str,
    Void,
}

impl DataType {
    pub fn to_llvm_type(&self) -> &str{
        match self {
            DataType::Int => "i64",
            DataType::Str => unimplemented!(),
            DataType::Void => "void",
        }
    }
}