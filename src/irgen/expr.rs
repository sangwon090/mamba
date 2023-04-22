use crate::parser::ast::{Expression, AstNodeType, InfixExpression, PrefixExpression, Operator};
use crate::lexer::{Literal, Identifier};
use std::collections::HashMap;

pub fn eval_constexpr(expression: &Box<dyn Expression>, vars: &HashMap<String, i64>) -> Option<i64> {
    match expression.get_type() {
        AstNodeType::InfixExpression => {
            let expression = expression.as_any().downcast_ref::<InfixExpression>().unwrap();
            let left = eval_constexpr(&expression.left, &vars);
            let right = eval_constexpr(&expression.right, &vars);

            if left.is_some() && right.is_some() {
                let left = left.unwrap();
                let right = right.unwrap();

                match expression.operator {
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
        AstNodeType::PrefixExpression => {
            let expression = expression.as_any().downcast_ref::<PrefixExpression>().unwrap();
            let right = eval_constexpr(&expression.right, &vars);

            match right {
                Some(right) => {
                    match expression.operator {
                        Operator::UnaryMinus => Some(-right),
                        Operator::BitwiseNot => Some(!right),
                        _ => None,
                    }
                },
                None => None,
            }
        },
        AstNodeType::Literal => {
            let literal = expression.as_any().downcast_ref::<Literal>().unwrap();

            match literal {
                Literal::Number(n) => Some(*n),
                Literal::String(_) => None,
            }
        },
        AstNodeType::Identifier => {
            let identifier = expression.as_any().downcast_ref::<Identifier>().unwrap();

            if vars.contains_key(&identifier.0) {
                Some(vars[&identifier.0])
            } else {
                None
            }
        }
        _ => None,
    }
}