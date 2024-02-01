pub mod block;
pub mod expr;
pub mod expression;
pub mod function;
pub mod instruction;

use std::collections::{HashMap};
use crate::parser::{LetStatement, DefStatement, IfStatement, ReturnStatement};
use crate::parser::ast::{AbstractSyntaxTree, AstNodeType, Expression, InfixExpression, Operator, Statement};
use crate::types::DataType;
use crate::{error::IRGenError};
use crate::lexer::{Literal, Identifier};
use crate::irgen::expr::*;
use crate::downcast;

pub struct IRGen {
    ast: AbstractSyntaxTree,
    global_var: HashMap<String, i64>,
}

pub struct LabelCounter {
    count: usize,
}

impl LabelCounter {
    pub fn new() -> LabelCounter {
        LabelCounter {
            count: 0,
        }
    }
    
    fn get_label(&mut self) -> usize {
        let result = self.count;
        self.count += 1;
        
        result
    }
}

impl IRGen {
    pub fn new(ast: AbstractSyntaxTree) -> IRGen {
        IRGen {
            ast,
            global_var: HashMap::new(),
        }
    }

    fn generate_global_vars(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();
        let global_vars: Vec<LetStatement> = Vec::new();

        self.ast.statements.retain(|statement| {
            if statement.get_type() == AstNodeType::LetStatement {
                let let_statement = downcast!(LetStatement, statement);
                                
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

    fn generate_if(if_statement: &IfStatement, counter: &mut LabelCounter) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        let cond = &if_statement.condition;

        let cond_label = match cond.get_type() {
            AstNodeType::InfixExpression => {
                let expr = downcast!(InfixExpression, cond);

                let cond_code = match expr.operator {
                    Operator::Equal => "eq",
                    Operator::NotEqual => "ne",
                    Operator::Less => "slt",
                    Operator::LessEqual => "sle",
                    Operator::Greater => "sgt",
                    Operator::GreaterEqual => "sge",
                    _ => todo!("todo: implement non-comparison operators for if condition")
                };

                let left = match expr.left.get_type() {
                    AstNodeType::Identifier => {
                        let left_label_ptr = counter.get_label();
                        result += &format!("%{left_label_ptr} = alloca i64, align 8 ; copy left for if statement\n");
                        result += &format!("store i64 %0, ptr %{left_label_ptr}, align 8 ; copy left for if statement\n");
                        
                        let left_label = counter.get_label();
                        result += &format!("%{left_label} = load i64, ptr %{left_label_ptr}, align 8 ; copy left for if statement?\n");
                        format!("%{left_label}")
                    },
                    AstNodeType::Literal => {
                        let literal = downcast!(Literal, expr);

                        if let Literal::Number(n) = literal {
                            n
                        } else {
                            todo!("todo: impelement non-integer for operand");
                        }.to_string()
                    }

                    _ => todo!("todo: implement complex expreession for operand")
                };

                let right = match expr.right.get_type() {
                    AstNodeType::Identifier => {
                        let right_label_ptr = counter.get_label();
                        result += &format!("%{right_label_ptr} = alloca i64, align 8; copy right for if statement\n");
                        result += &format!("store i64 %0, ptr %{right_label_ptr}, align 8 ; copy right for if statement\n");
                        
                        let right_label = counter.get_label();
                        result += &format!("%{right_label} = load i64, ptr %{right_label_ptr}, align 8 ; copy right for if statement?\n");

                        format!("%{right_label}")
                    },
                    AstNodeType::Literal => {
                        let literal = downcast!(Literal, expr.right);

                        if let Literal::Number(n) = literal {
                            n
                        } else {
                            todo!("todo: impelement non-integer for operand");
                        }.to_string()
                    }

                    _ => todo!("todo: implement complex expreession for operand")
                };

                let cmp_result = counter.get_label();
                result += &format!("%{cmp_result} = icmp {cond_code} i64 {}, {} ; store result for if statement\n", left, right);

                cmp_result
            },
            AstNodeType::PrefixExpression => todo!("todo: implement prefix expression for if condition"),
            _ => panic!("invalid expression for if condition!"),
        };

        let branch_true = counter.get_label();
        let branch_false = counter.get_label();

        result += &format!("br i1 %{}, label %{}, label %{} ; jump to label\n", cond_label, branch_true, branch_false);
        result += &format!("{}:\n", branch_true);
        result += &IRGen::generate_statement(&if_statement.then, counter).unwrap();
        result += &format!("{}:\n", branch_false);
        if let Some(stmt) = &if_statement.r#else {
            result += &IRGen::generate_statement(&stmt, counter).unwrap();
        }

        Ok(result)
    }

    fn generate_expression(expression: &Box<dyn Expression>, counter: &mut LabelCounter) -> Result<(String, usize), IRGenError> {
        let mut result: String = String::new();

        result += "; code for expression!\n";

        Ok((result, 0))
    }

    fn generate_statement(statement: &Box<dyn Statement>, counter: &mut LabelCounter) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        match statement.get_type() {
            AstNodeType::LetStatement => todo!("let statement in fnCall"),
            AstNodeType::IfStatement => result += &IRGen::generate_if(downcast!(IfStatement, statement), counter).unwrap(),
            AstNodeType::ReturnStatement => {
                let stmt = downcast!(ReturnStatement, statement);

                if stmt.expression.get_type() == AstNodeType::Literal {
                    let literal = downcast!(Literal, stmt.expression);

                    if let Literal::Number(n) = literal {
                        result += &format!("ret i64 {}\n", n);
                    } else {
                        todo!("return value only supports number for now");
                    }
                } else {
                    let (expr_code, expr_label) = IRGen::generate_expression(&stmt.expression, counter).unwrap();
                    result += &expr_code;
                    result += &format!("ret i64 %{}\n", expr_label);
                }
            }
            _ => {},
        }
        
        Ok(result)
    }

    fn generate_function(def_statement: &DefStatement) -> Result<String, IRGenError> {
        let mut result: String = String::new();
        let mut label_idx: LabelCounter = LabelCounter::new();

        let parameters: HashMap<String, usize> = def_statement.parameters.iter().map(|(identifier, _)|
            (identifier.to_string(), label_idx.get_label())
        ).collect();

        let params_result: Vec<String> = (0..label_idx.count).map(|n| format!("i64 noundef %{n}")).collect();
        label_idx.get_label(); // <-- clang does this, but why?

        let return_label = label_idx.get_label();

        result += &format!("define i64 @{}({}) nounwind {{\n", def_statement.name.to_string(), params_result.join(", "));
        result += &format!("%{} = alloca i64, align 8 ; return value\n", return_label); // for return value

        let parameters: HashMap<String, usize> = parameters.into_iter().map(|param| {
                let new_label = label_idx.get_label();
                result += &format!("%{} = alloca i64, align 8 ; parameter {}\n", new_label, param.0);
                result += &format!("store i64 %{}, ptr %{}, align 8 ; parameter {}\n", param.1, new_label, param.0);
                
                (param.0, new_label)
        }).collect();

        for statement in &def_statement.statements {
            result += &IRGen::generate_statement(statement, &mut label_idx).unwrap();
        }

        result += "ret i64 0\n";
        result += &format!("}}\n");
        
        Ok(result)
    }

    fn generate_functions(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        self.ast.statements.retain(|statement| {
            if statement.get_type() == AstNodeType::DefStatement {
                result += &IRGen::generate_function(downcast!(DefStatement, statement)).unwrap();

                false
            } else {
                true
            }
        });

        Ok(result)
    }
    pub fn generate_ir(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();

        result += &self.generate_global_vars().unwrap();
        result += &self.generate_functions().unwrap();

        Ok(result)
    }
}