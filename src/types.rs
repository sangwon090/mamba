#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub enum DataType {
    void,
    bool,
    str,

    SignedInteger(SignedInteger),
    UnsignedInteger(UnsignedInteger),
    FloatingPoint(FloatingPoint),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub enum SignedInteger {
    i8,
    i16,
    i32,
    i64,
    i128,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub enum UnsignedInteger {
    u8,
    u16,
    u32,
    u64,
    u128,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub enum FloatingPoint {
    f32,
    f64,
    f128,
}