use crate::parser::{Expression, Operator};
use crate::error::IRGenError;
use crate::codegen::llvm::*;
use crate::types::DataType;

pub fn generate_expr(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, expr: &Expression) -> Result<(String, String, DataType), IRGenError> {
    let mut result = String::new();

    let (idx, dtype) = match expr {
        Expression::Prefix(_prefix) => todo!(),
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

            let idx = global_ctx.get_label();

            // TODO: type checking
            match expr.operator {
                Operator::Equal | Operator::NotEqual |
                Operator::Less | Operator::LessEqual |
                Operator::Greater | Operator::GreaterEqual => result += &format!("%{} = icmp {} i32 {}, {}\n", idx, expr.operator.to_mnemonic(), left_idx, right_idx),
                Operator::Plus => result += &format!("%{} = add nsw i32 {}, {}\n", idx, left_idx, right_idx),
                Operator::Minus => result += &format!("%{} = sub nsw i32 {}, {}\n", idx, left_idx, right_idx),
                Operator::Multiply => result += &format!("%{} = mul nsw i32 {}, {}\n", idx, left_idx, right_idx),
                Operator::Divide => result += &format!("%{} = sdiv i32 {}, {}\n", idx, left_idx, right_idx),
                Operator::Modulo => result += &format!("%{} = srem i32 {}, {}\n", idx, left_idx, right_idx),
                Operator::LeftShift => result += &format!("%{} = shl i32 {}, {}\n", idx, left_idx, right_idx),
                Operator::RightShift => result += &format!("%{} = ashr i32 {}, {}\n", idx, left_idx, right_idx),

                _ => panic!("{} cannot be infix expression!", expr),
            };

            let dtype = if left_dtype == right_dtype {
                left_dtype
            } else {
                panic!("datatype mismatch: {:?} and {:?}", left_dtype, right_dtype);
            };

            (format!("%{idx}"), dtype)
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
            if scoped_ctx.local_var.contains_key(ident) {
                let literal = &scoped_ctx.local_var[ident];
                
                let dtype = match literal {
                    Literal::SignedInteger((_, dtype)) => DataType::SignedInteger(*dtype),
                    Literal::UnsignedInteger((_, dtype)) => DataType::UnsignedInteger(*dtype),
                    Literal::String(_) => DataType::str,
                };

                (format!("%{ident}"), dtype)
            } else if global_ctx.global_var.contains_key(ident) {
                let literal = &global_ctx.global_var[ident];
                
                let dtype = match literal {
                    Literal::SignedInteger((_, dtype)) => DataType::SignedInteger(*dtype),
                    Literal::UnsignedInteger((_, dtype)) => DataType::UnsignedInteger(*dtype),
                    Literal::String(_) => DataType::str,
                };
                
                (format!("@{ident}"), dtype)
            } else {
                panic!("identifier {} not found!", ident);
            }
        },
    };

    Ok((result, idx, dtype))
}