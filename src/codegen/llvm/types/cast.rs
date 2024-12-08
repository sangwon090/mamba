use std::{cmp::max, collections::HashMap, sync::OnceLock};
use crate::{codegen::llvm::GlobalContext, types::{DataType, SignedInteger, UnsignedInteger}};
use super::{SIGNED_INTEGERS, UNSIGNED_INTEGERS};

type FnTypeCast = dyn Fn(&mut GlobalContext, &str) -> (String, String, DataType) + Send + Sync + 'static;

static CAST: OnceLock<HashMap<(DataType, DataType), Box<FnTypeCast>>> = OnceLock::new();

macro_rules! add_cast {
    ($hashmap:ident, $from:expr, $to:expr, $fn:expr) => {
        $hashmap.insert(($from, $to), Box::new($fn))
    }
}

pub fn cast() -> &'static HashMap<(DataType, DataType), Box<FnTypeCast>> {
    CAST.get_or_init(|| {
        let mut cast: HashMap<(DataType, DataType), Box<FnTypeCast>> = HashMap::new();

        for a in &SIGNED_INTEGERS {
            for b in &SIGNED_INTEGERS {
                if a == b {
                    continue;
                } else if a < b {
                    add_cast!(cast, DataType::SignedInteger(*a), DataType::SignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = zext {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*b)))
                    });
                } else if a > b {
                    add_cast!(cast, DataType::SignedInteger(*a), DataType::SignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = trunc {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*b)))
                    });
                }
            }

            for b in &UNSIGNED_INTEGERS {
                let c: &SignedInteger = b.into();

                if a == c {
                    continue;
                } else if a < c {
                    add_cast!(cast, DataType::SignedInteger(*a), DataType::UnsignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = zext {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*c))) });
                } else if a > c {
                    add_cast!(cast, DataType::SignedInteger(*a), DataType::UnsignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = trunc {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::SignedInteger(*a), DataType::SignedInteger(*c)))
                    });
                }
            }
        }

        for a in &UNSIGNED_INTEGERS {
            for b in &SIGNED_INTEGERS {
                let c: &UnsignedInteger = b.into();

                if a == c {
                    continue;
                } else if a < c {
                    add_cast!(cast, DataType::UnsignedInteger(*a), DataType::SignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = zext {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*c)))
                    });
                } else if a > c {
                    add_cast!(cast, DataType::UnsignedInteger(*a), DataType::SignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = trunc {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*c)))
                    });
                }
            }

            for b in &UNSIGNED_INTEGERS {
                if a == b {
                    continue;
                } else if a < b {
                    add_cast!(cast, DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = zext {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b)))
                    });
                } else if a > b {
                    add_cast!(cast, DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b), |ctx, src| {
                        let idx = &format!("%{}", ctx.get_label());
                        (idx.into(), format!("{} = trunc {} {} to {}\n", idx, a.to_mnemonic(), src, b.to_mnemonic()), max(DataType::UnsignedInteger(*a), DataType::UnsignedInteger(*b)))
                    });
                }
            }
        }

        cast
    })
}