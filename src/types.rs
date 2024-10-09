use crate::lexer::Keyword;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    // Boolean type
    bool,

    // Signed integer types
    i8,
    i16,
    i32,
    i64,
    i128,

    // Unsigned integer types
    u8,
    u16,
    u32,
    u64,
    u128,

    // Floating-point types
    f32,
    f64,
    f128,

    // String types
    str,

    // Void
    void,
}

impl From<Keyword> for DataType {
    fn from(value: Keyword) -> Self {
        match value {
            Keyword::Int => DataType::i64,
            Keyword::Str => DataType::str,
            Keyword::Void => DataType::void,
            _ => panic!("Cannot convert {:?} into DataType!", value),
        }
    }
}

impl DataType {
    pub fn to_mnemonic(&self) -> &'static str {
        match &self {
            DataType::bool  => "i1",

            DataType::i8 => "i8",
            DataType::i16 => "i16",
            DataType::i32 => "i32",
            DataType::i64 => "i64",
            DataType::i128 => "i128",
            DataType::u8 => "u8",
            DataType::u16 => "u16",
            DataType::u32 => "u32",
            DataType::u64 => "u64",
            DataType::u128 => "u128",

            DataType::f32 => "float",
            DataType::f64 => "double",
            DataType::f128 => "fp128",

            DataType::str => "i8*",

            DataType::void => "void",
        }
    }

    pub fn is_bool(&self) -> bool {
        if *self == DataType::bool {
            true
        } else {
            false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            DataType::bool | 
            DataType::i8 | DataType::i16 | DataType::i32 | DataType::i64 |
            DataType::u8 | DataType::u16 | DataType::u32 | DataType::u64 => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        if *self == DataType::void {
            true
        } else {
            false
        }
    }


}