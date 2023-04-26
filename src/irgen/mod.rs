pub mod expr;

use std::collections::{HashMap};
use crate::parser::{LetStatement, DefStatement, IfStatement, ReturnStatement};
use crate::parser::ast::{AbstractSyntaxTree, AstNodeType, Expression, InfixExpression, Operator};
use crate::types::DataType;
use crate::{error::IRGenError};
use crate::lexer::{Literal, Identifier};
use crate::irgen::expr::*;
use crate::downcast;

pub struct IRGen {
    ast: AbstractSyntaxTree,
    global_var: HashMap<String, i64>,
    block_idx: usize,
}

impl IRGen {
    pub fn new(ast: AbstractSyntaxTree) -> IRGen {
        IRGen {
            ast,
            global_var: HashMap::new(),
            block_idx: 0,
        }
    }

    fn get_block_name(&mut self) -> String {
        let idx = self.block_idx;
        self.block_idx += 1;

        format!("{}", idx)
    }

    fn generate_global_var(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        self.ast.statements.retain(|statement| {
            if statement.get_type() == AstNodeType::LetStatement {
                let let_statement = downcast!(LetStatement, statement);
                
                println!("constexpr {} is {:?}!!!", let_statement.identifier.to_string(), eval_constexpr(&let_statement.expression, &self.global_var));

                match eval_constexpr(&let_statement.expression, &self.global_var) {
                    Some(n) => {
                        self.global_var.insert(let_statement.identifier.0.clone(), n);
                        true
                    },
                    None => {
                        false
                    }
                }
            } else {
                true
            }
        });

        for (ident, value) in &self.global_var {
            result += &format!("@{} = global i64 {}\n", ident, value);
        }

        Ok(result)
    }

    fn generate_function(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        Ok(result)
    }

    fn generate_functions(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        self.ast.statements.retain(|statement| {
            if statement.get_type() == AstNodeType::DefStatement {
                let def_statement = downcast!(DefStatement, statement);

                let parameters: Vec<String> = def_statement.parameters.iter().map(|(identifier, _)|
                    format!("i64 %{}", identifier.to_string())
                ).collect();

                result += &format!("define i64 @{}({}) nounwind {{\n", def_statement.name.to_string(), parameters.join(", "));
                
                for statement in &def_statement.statements {
                    match statement.get_type() {
                        AstNodeType::LetStatement => todo!("let statement in fnCall"),
                        AstNodeType::IfStatement => {
                            let statement = downcast!(IfStatement, statement);
                            let condition = &statement.condition;

                            // block for comparison
                            result += &format!("{}:\n", self.block_idx);
                            self.block_idx += 1;

                            match condition.get_type() {
                                AstNodeType::InfixExpression => {
                                    let expression = downcast!(InfixExpression, condition);

                                    println!("expression: {}", expression.to_string());
                                    println!("expr.left: {}, expr.right: {}", expression.left.to_string(), expression.right.to_string());
                                    println!("condition: {} ", condition.to_string());

                                    let cond_code = match expression.operator {
                                        Operator::Equal => "eq",
                                        Operator::NotEqual => "ne",
                                        Operator::Less => "slt",
                                        Operator::LessEqual => "sle",
                                        Operator::Greater => "sgt",
                                        Operator::GreaterEqual => "sge",
                                        _ => todo!("todo: implement non-comparison operators for if condition")
                                    };
                                    let left = match expression.left.get_type() {
                                        AstNodeType::Identifier => format!("%{}", downcast!(Identifier, expression.left).to_string()),
                                        AstNodeType::Literal => {
                                            let literal = downcast!(Literal, expression);

                                            if let Literal::Number(n) = literal {
                                                *n
                                            } else {
                                                todo!("todo: impelement non-integer for operand");
                                            }.to_string()
                                        }

                                        _ => todo!("todo: implement complex expreession for operand")
                                    };
                                    let right = match expression.right.get_type() {
                                        AstNodeType::Identifier => format!("%{}", downcast!(Identifier, expression.right).to_string()),
                                        AstNodeType::Literal => {
                                            let literal = downcast!(Literal, expression.right);

                                            if let Literal::Number(n) = literal {
                                                *n
                                            } else {
                                                todo!("todo: impelement non-integer for operand");
                                            }.to_string()
                                        }

                                        _ => todo!("todo: implement complex expreession for operand")
                                    };

                                    result += &format!("icmp {cond_code} i64 {}, {}\n", left, right);
                                },
                                AstNodeType::PrefixExpression => {
                                    todo!("todo: implement prefix expression for if condition")
                                },
                                _ => {
                                    panic!("invalid expression for if condition!")
                                }
                            }

                            println!("cond: {}", condition.to_string());
                            
                            // block for if condition
                            

                            // block for else condition
                            
                        }
                        _ => {},
                    }
                }

                result += "ret i64 0\n";
                result += &format!("}}\n");
                
                false
            } else {
                true
            }
        });

        Ok(result)
    }
    pub fn generate_ir(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        result += &self.generate_global_var().unwrap();
        result += &self.generate_functions().unwrap();

        Ok(result)
    }
}