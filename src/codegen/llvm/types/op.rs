use std::{collections::HashMap, sync::OnceLock};
use crate::{codegen::llvm::GlobalContext, parser::Operator, types::DataType};
use super::{SIGNED_INTEGERS, UNSIGNED_INTEGERS};

type FnUnaryOperation = dyn Fn(&mut GlobalContext, &str) -> (String, String) + Send + Sync + 'static;
type FnInfixOperation = dyn Fn(&mut GlobalContext, &str, &str) -> (String, String) + Send + Sync + 'static;

static UNARY_OPERATION: OnceLock<HashMap<(DataType, Operator), Box<FnUnaryOperation>>> = OnceLock::new();
static INFIX_OPERATION: OnceLock<HashMap<(DataType, Operator), Box<FnInfixOperation>>> = OnceLock::new();

macro_rules! add_unary_operation {
    ($hashmap:ident, $ty:expr, $op:expr, $fn:expr) => {
        $hashmap.insert(($ty, $op), Box::new($fn))
    }
}

macro_rules! add_infix_operation {
    ($hashmap:ident, $ty:expr, $op:expr, $fn:expr) => {
        $hashmap.insert(($ty, $op), Box::new($fn))
    }
}

pub fn unary_op() -> &'static HashMap<(DataType, Operator), Box<FnUnaryOperation>> {
    UNARY_OPERATION.get_or_init(|| {
        let mut op: HashMap<(DataType, Operator), Box<FnUnaryOperation>> = HashMap::new();

        for ty in &SIGNED_INTEGERS {
            add_unary_operation!(op, DataType::SignedInteger(*ty), Operator::UnaryPlus, |_, src| {
                (src.into(), String::new())
            });

            add_unary_operation!(op, DataType::SignedInteger(*ty), Operator::UnaryMinus, |ctx, src| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(),  format!("{} = sub nsw {} 0, {}", idx, ty.to_mnemonic(), src))
            });

            add_unary_operation!(op, DataType::SignedInteger(*ty), Operator::BitwiseNot, |ctx, src| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = sub nsw {} 0, {}", idx, ty.to_mnemonic(), src))
            });
        }

        for ty in &UNSIGNED_INTEGERS {            
            add_unary_operation!(op, DataType::UnsignedInteger(*ty), Operator::BitwiseNot, |ctx, src| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = xor {} {}, -1", idx, ty.to_mnemonic(), src))
            });
        }

        op
    })
}

pub fn infix_op() -> &'static HashMap<(DataType, Operator), Box<FnInfixOperation>> {
    INFIX_OPERATION.get_or_init(|| {
        let mut op: HashMap<(DataType, Operator), Box<FnInfixOperation>> = HashMap::new();


        for cmp in &[Operator::Equal, Operator::NotEqual, Operator::Less, Operator::LessEqual, Operator::Greater, Operator::GreaterEqual] {
            for ty in &SIGNED_INTEGERS {
                add_infix_operation!(op, DataType::SignedInteger(*ty), *cmp, |ctx, left, right| {
                    let idx = &format!("%{}", ctx.get_label());
                    (idx.into(), format!("{} = icmp {} {} {}, {}", idx, cmp.to_mnemonic(), ty.to_mnemonic(), left, right))
                });
            }

            for ty in &UNSIGNED_INTEGERS {
                add_infix_operation!(op, DataType::UnsignedInteger(*ty), *cmp, |ctx, left, right| {
                    let idx = &format!("%{}", ctx.get_label());
                    (idx.into(), format!("{} = icmp {} {} {}, {}", idx, cmp.to_mnemonic(), ty.to_mnemonic(), left, right))
                });
            }
        }

        // TODO: clean duplicated code in an elegant way
        for ty in &SIGNED_INTEGERS {
            add_infix_operation!(op, DataType::SignedInteger(*ty), Operator::Plus, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = add nsw {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::SignedInteger(*ty), Operator::Minus, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = sub nsw {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::SignedInteger(*ty), Operator::Multiply, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = mul nsw {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::SignedInteger(*ty), Operator::Divide, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = sdiv {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::SignedInteger(*ty), Operator::Modulo, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = srem {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::SignedInteger(*ty), Operator::LeftShift, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = shl {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::SignedInteger(*ty), Operator::RightShift, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = ashr {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });
        }

        for ty in &UNSIGNED_INTEGERS {
            add_infix_operation!(op, DataType::UnsignedInteger(*ty), Operator::Plus, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = add nsw {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::UnsignedInteger(*ty), Operator::Minus, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = sub nsw {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::UnsignedInteger(*ty), Operator::Multiply, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = mul nsw {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::UnsignedInteger(*ty), Operator::Divide, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = sdiv {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::UnsignedInteger(*ty), Operator::Modulo, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = srem {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::UnsignedInteger(*ty), Operator::LeftShift, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = shl {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });

            add_infix_operation!(op, DataType::UnsignedInteger(*ty), Operator::RightShift, |ctx, left, right| {
                let idx = &format!("%{}", ctx.get_label());
                (idx.into(), format!("{} = ashr {} {}, {}", idx, ty.to_mnemonic(), left, right))
            });
        }
        op
    })
}