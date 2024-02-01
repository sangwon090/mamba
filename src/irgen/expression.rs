use crate::{downcast, lexer::Literal, parser::ast::{AstNodeType, Expression, FnCallExpression, InfixExpression, PrefixExpression}};

pub struct ExpressionGen;

impl ExpressionGen {
    pub fn generate_code(expression: &Box<dyn Expression>) -> String {
        let mut result = String::new();

        match expression.get_type() {
            AstNodeType::InfixExpression => {
                let expr = downcast!(InfixExpression, expression);

                let left_id = if ExpressionGen::is_expression(expr.left.get_type()) {
                    result += &ExpressionGen::generate_code(&expr.left);
                };

                let right_id = if ExpressionGen::is_expression(expr.right.get_type()) {
                    result += &ExpressionGen::generate_code(&expr.right);
                };
            },
            AstNodeType::PrefixExpression => {
                let expr = downcast!(PrefixExpression, expression);
            },
            AstNodeType::FnCallExpression => {
                let expr = downcast!(FnCallExpression, expression);
            },
            AstNodeType::Literal => {
                let literal = downcast!(Literal, expression);
                println!("{:?}", literal);
            }
            _ => panic!("This code is not expected to be executed: expected expression, found {:?}", expression.get_type()),
        }

        result
    }

    fn is_expression(node_type: AstNodeType) -> bool {
        if let AstNodeType::InfixExpression | AstNodeType::PrefixExpression | AstNodeType::FnCallExpression = node_type {
            true
        } else {
            false
        }
    }

    fn is_literal(node_type: AstNodeType) -> bool {
        node_type == AstNodeType::Literal
    }
}