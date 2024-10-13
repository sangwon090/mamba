use crate::lexer::Keyword;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum DataType {
    void,
    bool,
    str,

    SignedInteger(SignedInteger),
    UnsignedInteger(UnsignedInteger),
    FloatingPoint(FloatingPoint),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum SignedInteger {
    i8,
    i16,
    i32,
    i64,
    i128,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum UnsignedInteger {
    u8,
    u16,
    u32,
    u64,
    u128,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum FloatingPoint {
    f32,
    f64,
    f128,
}

impl DataType {
    pub fn to_mnemonic(&self) -> &'static str {
        match &self {
            DataType::void => "void",
            DataType::bool  => "i1",
            DataType::str => "i8*",
            DataType::SignedInteger(dtype) => match dtype {
                SignedInteger::i8 => "i8",
                SignedInteger::i16 => "i16",
                SignedInteger::i32 => "i32",
                SignedInteger::i64 => "i64",
                SignedInteger::i128 => "i128",
            },
            DataType::UnsignedInteger(dtype) => match dtype {
                UnsignedInteger::u8 => "u8",
                UnsignedInteger::u16 => "u16",
                UnsignedInteger::u32 => "u32",
                UnsignedInteger::u64 => "u64",
                UnsignedInteger::u128 => "u128",
            },
            DataType::FloatingPoint(dtype) => match dtype {
                FloatingPoint::f32 => "float",
                FloatingPoint::f64 => "double",
                FloatingPoint::f128 => "fp128",
            },
        }
    }
}