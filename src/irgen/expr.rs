use crate::parser::ast::{Expression, Operator};
use crate::lexer::Literal;
use std::collections::HashMap;

pub fn eval_constexpr(expression: &Expression, vars: &HashMap<String, i64>) -> Option<i64> {
    match expression {
        Expression::Infix(expr) => {
            let left: Option<i64> = eval_constexpr(&expr.left, &vars);
            let right: Option<i64> = eval_constexpr(&expr.right, &vars);

            if left.is_some() && right.is_some() {
                let left = left.unwrap();
                let right = right.unwrap();

                match expr.operator {
                    Operator::BitwiseAnd    => Some(left & right),
                    Operator::BitwiseOr     => Some(left | right),
                    Operator::BitwiseXor    => Some(left ^ right),
                    Operator::Plus          => Some(left + right),
                    Operator::Minus         => Some(left - right),
                    Operator::Multiply      => Some(left * right),
                    Operator::Divide        => Some(left / right),
                    Operator::Modulo        => Some(left % right),
                    Operator::Equal         => Some((left == right) as i64),
                    Operator::NotEqual      => Some((left != right) as i64),
                    Operator::Less          => Some((left < right) as i64),
                    Operator::LessEqual     => Some((left <= right) as i64),
                    Operator::Greater       => Some((left > right) as i64),
                    Operator::GreaterEqual  => Some((left >= right) as i64),
                    Operator::LeftShift     => Some(left << right),
                    Operator::RightShift    => Some(left >> right),
                    _ => None,
                }
            } else {
                None
            }
        },
        Expression::Prefix(expr) => {
            let right = eval_constexpr(&expr.right, &vars);

            match right {
                Some(right) => {
                    match expr.operator {
                        Operator::UnaryMinus => Some(-right),
                        Operator::BitwiseNot => Some(!right),
                        _ => None,
                    }
                },
                None => None,
            }
        },
        Expression::FnCall(_) => {
            None 
        }
        Expression::Literal(literal) => {
            match literal {
                Literal::Number(n) => Some(*n),
                Literal::String(_) => None,
            }
        },
        Expression::Identifier(ident) => {
            if vars.contains_key(&ident.0) {
                Some(vars[&ident.0])
            } else {
                None
            }
        },
    }
}