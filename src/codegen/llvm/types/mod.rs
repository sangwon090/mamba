mod cast;

pub use cast::{Types, cast};
use crate::types::{DataType, SignedInteger, UnsignedInteger, FloatingPoint};

impl DataType {
    pub fn to_mnemonic(&self) -> &'static str {
        match self{
            DataType::void => "void",
            DataType::bool  => "i1",
            DataType::str => "i8*",
            DataType::SignedInteger(dtype) => dtype.to_mnemonic(),
            DataType::UnsignedInteger(dtype) => dtype.to_mnemonic(),
            DataType::FloatingPoint(dtype) => dtype.to_mnemonic(),
        }
    }
}

impl SignedInteger {
    pub fn to_mnemonic(&self) -> &'static str {
        match self {
            SignedInteger::i8 => "i8",
            SignedInteger::i16 => "i16",
            SignedInteger::i32 => "i32",
            SignedInteger::i64 => "i64",
            SignedInteger::i128 => "i128",
        }
    }
}

impl UnsignedInteger {
    pub fn to_mnemonic(&self) -> &'static str {
        match self {
            UnsignedInteger::u8 => "i8",
            UnsignedInteger::u16 => "i16",
            UnsignedInteger::u32 => "i32",
            UnsignedInteger::u64 => "i64",
            UnsignedInteger::u128 => "i128",
        }
    }
}

impl From<&SignedInteger> for &UnsignedInteger {
    fn from(value: &SignedInteger) -> Self {
        match value {
            SignedInteger::i8 => &UnsignedInteger::u8,
            SignedInteger::i16 => &UnsignedInteger::u16,
            SignedInteger::i32 => &UnsignedInteger::u32,
            SignedInteger::i64 => &UnsignedInteger::u64,
            SignedInteger::i128 => &UnsignedInteger::u128,
        }
    }
}

impl From<&UnsignedInteger> for &SignedInteger {
    fn from(value: &UnsignedInteger) -> Self {
        match value {
            UnsignedInteger::u8 => &SignedInteger::i8,
            UnsignedInteger::u16 => &SignedInteger::i16,
            UnsignedInteger::u32 => &SignedInteger::i32,
            UnsignedInteger::u64 => &SignedInteger::i64,
            UnsignedInteger::u128 => &SignedInteger::i128,
        }
    }
}

impl FloatingPoint {
    pub fn to_mnemonic(&self) -> &'static str {
        match self {
            FloatingPoint::f32 => "float",
            FloatingPoint::f64 => "double",
            FloatingPoint::f128 => "fp128",
        }
    }
}