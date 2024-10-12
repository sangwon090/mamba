use crate::parser::ast::{Expression, Operator};
use crate::error::IRGenError;
use crate::codegen::llvm::*;

pub fn generate_expr(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, expr: &Expression) -> Result<(String, String), IRGenError> {
    let mut result = String::new();

    let idx = match expr {
        Expression::Prefix(_prefix) => todo!(),
        Expression::Infix(expr) => {
            let left_idx = {
                let (code, idx) = generate_expr(global_ctx, scoped_ctx, &expr.left).unwrap();
                result += &code;
                idx.to_string()
            };
            
            let right_idx = {
                let (code, idx) = generate_expr(global_ctx, scoped_ctx, &expr.right).unwrap();
                result += &code;
                idx.to_string()
            };

            let idx = global_ctx.get_label();

            // TODO: type checking
            match expr.operator {
                Operator::Equal | Operator::NotEqual |
                Operator::Less | Operator::LessEqual |
                Operator::Greater | Operator::GreaterEqual => result += &format!("%{} = icmp {} i64 %{}, %{}\n", idx, expr.operator.to_mnemonic(), left_idx, right_idx),
                Operator::Plus => result += &format!("%{} = add nsw i64 %{}, %{}\n", idx, left_idx, right_idx),
                Operator::Minus => result += &format!("%{} = sub nsw i64 %{}, %{}\n", idx, left_idx, right_idx),
                Operator::Multiply => result += &format!("%{} = mul nsw i64 %{}, %{}\n", idx, left_idx, right_idx),
                Operator::Divide => result += &format!("%{} = sdiv i64 %{}, %{}\n", idx, left_idx, right_idx),
                Operator::Modulo => result += &format!("%{} = srem i64 %{}, %{}\n", idx, left_idx, right_idx),
                Operator::LeftShift => result += &format!("%{} = shl i64 %{}, %{}\n", idx, left_idx, right_idx),
                Operator::RightShift => result += &format!("%{} = ashr i64 %{}, %{}\n", idx, left_idx, right_idx),

                _ => panic!("{} cannot be infix expression!", expr),
            };

            idx.to_string()
        },
        Expression::FnCall(expr) => {
            if !global_ctx.fn_decl.contains_key(&expr.ident.to_string()) {
                panic!("Unable to find function `{}`", &expr.ident.to_string());
            }
            
            let params = expr.args.iter().map(|expr| {
                let (code, idx) = generate_expr(global_ctx, scoped_ctx, expr).unwrap();
                result += &code;
                format!("i64 %{}", idx)
            }).collect::<Vec<String>>();

            let idx = global_ctx.get_label();
            result += &format!("%{} = call i64 @{}(", idx, &expr.ident.to_string());
            result += &params.join(", ");
            result += ")\n";

            idx.to_string()
        },
        Expression::Literal(literal) => {
            let (literal_code, literal_idx) = IRGen::generate_literal(global_ctx, scoped_ctx, literal).unwrap();
            result += &literal_code;
            literal_idx.to_string()
        },
        Expression::Identifier(ident) => {
            let idx = if global_ctx.global_var.contains_key(ident) || scoped_ctx.local_var.contains_key(ident){
                ident.clone()
            } else {
                panic!("Unable to find identifier {}", ident);
            };

            idx.to_string()
        },
    };

    Ok((result, idx))
}