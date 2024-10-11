use crate::parser::ast::{AstNodeType, Expression, FnCallExpression, InfixExpression, Operator};
use crate::error::IRGenError;
use crate::lexer::{Identifier, Literal};
use crate::codegen::llvm::*;

pub fn generate_expr(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, expr: &Box<dyn Expression>) -> Result<(String, String), IRGenError> {
    let mut result = String::new();

    let idx = match expr.get_type() {
        AstNodeType::InfixExpression => {
            let expr = expr.as_any().downcast_ref::<InfixExpression>().unwrap();
            
            let left_idx = match expr.left.get_type() {
                AstNodeType::InfixExpression | AstNodeType::PrefixExpression | AstNodeType::FnCallExpression => {
                    let (expr_code, expr_idx) = generate_expr(global_ctx, scoped_ctx, &expr.left).unwrap();
                    result += &expr_code;
                    expr_idx.to_string()
                },
                AstNodeType::Literal => {
                    let literal = expr.left.as_any().downcast_ref::<Literal>().unwrap();
                    let (literal_code, literal_idx) = IRGen::generate_literal(global_ctx, scoped_ctx, literal).unwrap();
                    result += &literal_code;
                    literal_idx.to_string()
                },
                AstNodeType::Identifier => {
                    let ident = expr.left.as_any().downcast_ref::<Identifier>().unwrap();
                    let (ident_code, ident_idx) = IRGen::generate_ident(global_ctx, scoped_ctx, &ident).unwrap();
                    result += &ident_code;
                    ident_idx
                },
                _ => global_ctx.get_label().to_string(),
            };

            let right_idx = match expr.right.get_type() {
                AstNodeType::InfixExpression | AstNodeType::PrefixExpression | AstNodeType::FnCallExpression => {
                    let (expr_code, expr_idx) = generate_expr(global_ctx, scoped_ctx, &expr.right).unwrap();
                    result += &expr_code;
                    expr_idx.to_string()
                },
                AstNodeType::Literal => {
                    let literal = expr.right.as_any().downcast_ref::<Literal>().unwrap();
                    let (literal_code, literal_idx) = IRGen::generate_literal(global_ctx, scoped_ctx, literal).unwrap();
                    result += &literal_code;
                    literal_idx.to_string()
                },
                AstNodeType::Identifier => {
                    let ident = expr.right.as_any().downcast_ref::<Identifier>().unwrap();
                    let (ident_code, ident_idx) = IRGen::generate_ident(global_ctx, scoped_ctx, &ident).unwrap();
                    result += &ident_code;
                    ident_idx
                },
                _ => global_ctx.get_label().to_string(),
            };

            let idx = global_ctx.get_label();
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

                _ => panic!("{} cannot be infix expression!", expr.to_string()),
            };

            idx.to_string()
        },
        AstNodeType::PrefixExpression => "".into(),
        AstNodeType::FnCallExpression => {
            let expr = expr.as_any().downcast_ref::<FnCallExpression>().unwrap();

            if !global_ctx.fn_decl.contains_key(&expr.identifier.to_string()) {
                panic!("Unable to find function `{}`", &expr.identifier.to_string());
            }
            
            let params = expr.arguments.iter().map(|expr| {
                let (code, idx) = generate_expr(global_ctx, scoped_ctx, &expr).unwrap();
                result += &code;
                format!("i64 %{}", idx)
            }).collect::<Vec<String>>();

            let idx = global_ctx.get_label();
            result += &format!("%{} = call i64 @{}(", idx, &expr.identifier.to_string());
            result += &params.join(", ");
            result += ")\n";

            idx.to_string()
        },
        AstNodeType::Literal => {
            let literal = expr.as_any().downcast_ref::<Literal>().unwrap();
            let (literal_code, literal_idx) = IRGen::generate_literal(global_ctx, scoped_ctx, literal).unwrap();
            result += &literal_code;
            literal_idx.to_string()
        },
        _ => {
            println!("WHAT??? {:?}\n", expr.get_type());
            "".into()
        },
    };

    Ok((result, idx))
}