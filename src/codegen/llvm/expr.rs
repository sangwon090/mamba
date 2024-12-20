use types::{cast, infix_op, unary_op};

use crate::parser::Expression;
use crate::error::IRGenError;
use crate::codegen::llvm::*;
use crate::types::DataType;

pub fn generate_expr(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, expr: &Expression) -> Result<(String, String, DataType), IRGenError> {
    let mut result = String::new();

    let (idx, dtype) = match expr {
        Expression::Unary(expr) => {
            let (idx, dtype) = {
                let (code, idx, dtype) = generate_expr(global_ctx, scoped_ctx, &expr.right).unwrap();
                result += &code;
                (idx.to_string(), dtype)
            };

            let (idx, code) = unary_op()[&(dtype, expr.operator)](global_ctx, &idx);
            result += &code;
            (idx, dtype)
        },
        Expression::Infix(expr) => {
            let (left_idx, left_dtype) = {
                let (code, idx, dtype) = generate_expr(global_ctx, scoped_ctx, &expr.left).unwrap();
                result += &code;
                (idx.to_string(), dtype)
            };
            
            let (right_idx, right_dtype) = {
                let (code, idx, dtype) = generate_expr(global_ctx, scoped_ctx, &expr.right).unwrap();
                result += &code;
                (idx.to_string(), dtype)
            };

            let (left_idx, right_idx, dtype) = if left_dtype < right_dtype {
                let (casted_idx, cast_code, _) = cast()[&(left_dtype, right_dtype)](global_ctx, &left_idx);
                result += &cast_code;
                (casted_idx, right_idx, right_dtype)
            } else if left_dtype > right_dtype {
                let (casted_idx, cast_code, _) = cast()[&(right_dtype, left_dtype)](global_ctx, &right_idx);
                result += &cast_code;
                (left_idx, casted_idx, left_dtype)
            } else {
                (left_idx, right_idx, left_dtype)
            };

            let (idx, code) = infix_op()[&(dtype, expr.operator)](global_ctx, &left_idx, &right_idx);
            result += &code;
            (idx, dtype)
        },
        Expression::FnCall(expr) => {
            let fn_dtype = if global_ctx.fn_decl.contains_key(&expr.ident) {
                global_ctx.fn_decl[&expr.ident].1
            } else {
                panic!("Unable to find function `{}`", &expr.ident.to_string());
            };
            
            let params = expr.args.iter().map(|expr| {
                let (code, idx, dtype) = generate_expr(global_ctx, scoped_ctx, expr).unwrap();
                result += &code;
                format!("{} {}", dtype.to_mnemonic(), idx)
            }).collect::<Vec<String>>();

            let idx = global_ctx.get_label();

            result += &format!("%{} = call {} @{}(", idx, fn_dtype.to_mnemonic(), &expr.ident.to_string());
            result += &params.join(", ");
            result += ")\n";

            let dtype = global_ctx.fn_decl[&expr.ident].1;

            (format!("%{idx}"), dtype)
        },
        Expression::Literal((literal, _)) => {
            let (literal_code, literal_idx) = IRGen::generate_literal(global_ctx, scoped_ctx, literal).unwrap();
            result += &literal_code;

            let dtype = match literal {
                Literal::SignedInteger((_, dtype)) => DataType::SignedInteger(*dtype),
                Literal::UnsignedInteger((_, dtype)) => DataType::UnsignedInteger(*dtype),
                Literal::String(_) => DataType::str,
            };

            (format!("%{literal_idx}"), dtype)
        },
        Expression::Identifier(ident) => {
            let ctx: Vec<&ScopedContext> = scoped_ctx.iter().filter(|ctx| {
                match ctx {
                    ScopedContext::FnDecl(fn_decl, _) => fn_decl.contains_key(ident),
                    ScopedContext::Scope(scope) => scope.contains_key(ident),
                }
            }).collect();

            if ctx.len() > 0 {
                match ctx.last().unwrap() {
                    ScopedContext::FnDecl(fn_decl, _) => {
                        (format!("%{ident}"), fn_decl[ident])
                    },
                    ScopedContext::Scope(scope) => {
                        match scope[ident] {
                            Literal::SignedInteger((_, dtype)) => {
                                let new_idx = global_ctx.get_label();
                                result += &format!("%{new_idx} = load {}, ptr %{}, align 4\n", dtype.to_mnemonic(), ident);
                                (format!("%{new_idx}"), DataType::SignedInteger(dtype))
                            },
                            Literal::UnsignedInteger((_, dtype)) => {
                                let new_idx = global_ctx.get_label();
                                result += &format!("%{new_idx} = load {}, ptr %{}, align 4\n", dtype.to_mnemonic(), ident);
                                (format!("%{new_idx}"), DataType::UnsignedInteger(dtype))
                            },
                            Literal::String(_) => (format!("%{ident}"), DataType::str),
                        }
                    }
                }
            } else {
                if global_ctx.global_var.contains_key(ident) {
                    let literal: Literal = global_ctx.global_var[ident].clone();
                    
                    match literal {
                        Literal::SignedInteger((_, dtype)) => {
                            let new_idx = global_ctx.get_label();
                            result += &format!("%{new_idx} = load {}, ptr @{}, align 4\n", dtype.to_mnemonic(), ident);
                            (format!("%{new_idx}"), DataType::SignedInteger(dtype))
                        },
                        Literal::UnsignedInteger((_, dtype)) => {
                            let new_idx = global_ctx.get_label();
                            result += &format!("%{new_idx} = load {}, ptr @{}, align 4\n", dtype.to_mnemonic(), ident);
                            (format!("%{new_idx}"), DataType::UnsignedInteger(dtype))
                        },
                        Literal::String(_) => (format!("@{ident}"), DataType::str),
                    }
                } else {
                    panic!("identifier {} not found!", ident);
                }
            }
        },
    };
    
    Ok((result, idx, dtype))
}