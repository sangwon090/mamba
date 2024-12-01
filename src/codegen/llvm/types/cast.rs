use std::{cell::OnceCell, cmp::max, collections::HashMap, hash::Hash, iter::zip, sync::OnceLock};

use crate::{parser::Operator, types::{DataType, SignedInteger, UnsignedInteger}};

type FnTypeCast = dyn Fn(&str, &str) -> (String, DataType) + Send + Sync + 'static;
type FnUnaryOperation = dyn Fn() -> String + Send + 'static;
type FnInfixOperation = dyn Fn() -> String + Send + 'static;

static CASTERS: OnceLock<HashMap<(DataType, DataType), Box<FnTypeCast>>> = OnceLock::new();
const SIGNED_INTEGERS: [SignedInteger; 5] = [SignedInteger::i8, SignedInteger::i16, SignedInteger::i32, SignedInteger::i64, SignedInteger::i128];
const UNSIGNED_INTEGERS: [UnsignedInteger; 5] = [UnsignedInteger::u8, UnsignedInteger::u16, UnsignedInteger::u32, UnsignedInteger::u64, UnsignedInteger::u128];

pub struct Types {
    unary_operation: HashMap<(DataType, Operator), Box<FnUnaryOperation>>,
    infix_operation: HashMap<(DataType, DataType, Operator), Box<FnInfixOperation>>,
}

macro_rules! add_type_cast {
    ($hashmap:ident, $from:expr, $to:expr, $fn:expr) => {
        $hashmap.insert(($from, $to), Box::new($fn))
    }
}

pub fn cast() -> &'static HashMap<(DataType, DataType), Box<FnTypeCast>> {
    CASTERS.get_or_init(|| {
        let mut casters: HashMap<(DataType, DataType), Box<FnTypeCast>> = HashMap::new();

        for a in &SIGNED_INTEGERS {
            for b in &SIGNED_INTEGERS {
                if a == b {
                    continue;
                } else if a < b {
                    add_type_cast!(casters, DataType::SignedInteger(*a), DataType::SignedInteger(*b), |src, dest| { (format!("{} = zext {} {} to {}\n", dest, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*b))) });
                } else if a > b {
                    add_type_cast!(casters, DataType::SignedInteger(*a), DataType::SignedInteger(*b), |src, dest| { (format!("{} = trunc {} {} to {}\n", dest, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*b))) });
                }
            }

            for b in &UNSIGNED_INTEGERS {
                let c: &SignedInteger = b.into();

                if a == c {
                    continue;
                } else if a < c {
                    add_type_cast!(casters, DataType::SignedInteger(*a), DataType::UnsignedInteger(*b), |src, dest| { (format!("{} = zext {} {} to {}\n", dest, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*c))) });
                } else if a > c {
                    add_type_cast!(casters, DataType::SignedInteger(*a), DataType::UnsignedInteger(*b), |src, dest| { (format!("{} = trunc {} {} to {}\n", dest, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*c))) });
                }
            }
        }

        for a in &UNSIGNED_INTEGERS {
            for b in &SIGNED_INTEGERS {
                // TODO
            }

            for b in &UNSIGNED_INTEGERS {
                if a == b {
                    continue;
                } else if a < b {
                    add_type_cast!(casters, DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b), |src, dest| {
                        (format!("{} = zext {} {} to {}\n", dest, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b)))
                    });
                } else if a > b {
                    add_type_cast!(casters, DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b), |src, dest| {
                        (format!("{} = trunc {} {} to {}\n", dest, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b)))
                    });
                }
            }
        }

        casters
    })
}

impl Types {
    pub fn new() -> Types {
        let mut type_cast: HashMap<(DataType, DataType), Box<FnTypeCast>> = HashMap::new();
        




        let unary_operation: HashMap<(DataType, Operator), Box<FnUnaryOperation>> = HashMap::new();


        let infix_operation: HashMap<(DataType, DataType, Operator), Box<FnInfixOperation>> = HashMap::new();


        Types {
            unary_operation,
            infix_operation,
        }
    }
}