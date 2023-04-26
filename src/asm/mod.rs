use std::collections::{HashMap, BTreeMap};

use crate::{types::DataType, error::CompileError, parser::ast::AbstractSyntaxTree};
use crate::parser::ast::{AstNodeType, PrefixExpression, InfixExpression, FnCallExpression, Statement, Expression, Operator};
use crate::lexer::{Identifier, Literal};
use crate::parser::{DefStatement, IfStatement, LetStatement, ReturnStatement};

pub struct AsmGenerator {
    ast: AbstractSyntaxTree,
    variables: BTreeMap<String, i64>,
    functions: HashMap<String, String>,
    code: String,
}

impl AsmGenerator {
    pub fn new(ast: AbstractSyntaxTree) -> AsmGenerator {
        AsmGenerator {
            ast,
            variables: BTreeMap::new(),
            functions: HashMap::new(),
            code: String::new(),
        }
    }

    pub fn generate_asm(&mut self) -> Result<String, CompileError> {
        let mut result = String::new();

        for statement in &self.ast.statements {
            let statement_type = statement.get_type();

            if statement_type == AstNodeType::LetStatement {
                let let_statement = downcast_stmt!(LetStatement, statement);
                
                if let DataType::Int = let_statement.r#type {
                    if self.variables.contains_key(&let_statement.identifier.0) {
                        return Err(CompileError(format!("[AsmGenerator] Variable with name `{}` already exists.", &let_statement.identifier.0)))
                    } else {
                        self.variables.insert(let_statement.identifier.to_string(), self.eval_expression(&let_statement.expression).unwrap());
                    }
                } else {
                    return Err(CompileError(format!("[AsmGenerator] Data type `{:?}` is not supported for now. Use integer instead.", let_statement.r#type)))
                }
            }
        }

        for statement in &self.ast.statements {
            let statement_type = statement.get_type();

            if statement_type == AstNodeType::DefStatement {
                let mut code = String::new();
                let def_statement = downcast!(DefStatement, statement);
                let mut parameters: HashMap<String, usize> = HashMap::new();
                let mut parameter_count: usize = 0;

                if let DataType::Int = def_statement.r#type {

                } else {
                    return Err(CompileError(format!("[AsmGenerator] Data type `{:?}` is not supported for now. Use integer instead.", def_statement.r#type)))
                }

                for (identifier, datatype) in &def_statement.parameters {
                    if *datatype != DataType::Int {
                        return Err(CompileError(format!("[AsmGenerator] Data type `{:?}` is not supported for now. Use integer instead.", datatype)))
                    }

                    if parameter_count > 3 {
                        return Err(CompileError("[AsmGenerator] only 4 parameters can be passed.".into()))
                    }

                    if parameters.contains_key(&identifier.0.to_string()) {
                        return Err(CompileError(format!("[AsmGenerator] parameter with name {} already exists!", identifier.0)))
                    }

                    parameters.insert(identifier.0.to_string(), parameter_count);
                    parameter_count += 1;
                }

                for statement in &def_statement.statements {
                    self.compile_statement(&mut code, statement).unwrap();
                }
            }
        }

        println!("variables: {:#?}", self.variables);

        Ok("".into())
    }

    fn get_register(n: usize) -> &'static str {
        match n {
            0 => "rcx",
            1 => "rdx",
            2 => "r8",
            3 => "r9",
            _ => "err",
        }
    }

    fn eval_expression(&self, expression: &Box<dyn Expression>) -> Result<i64, CompileError> {
        match expression.get_type() {
            AstNodeType::Identifier => {
                let identifier = downcast!(Identifier, expression);

                if self.variables.contains_key(&identifier) {
                    return Ok(*self.variables.get(&identifier).unwrap());
                } else {
                    return Err(CompileError(format!("[AsmGenerator] Cannot evaluate expression because identifier {} it not defined.", identifier)));
                }
            }
            AstNodeType::Literal => {
                let literal = downcast!(Literal, expression);
                let literal = if let Literal::Number(number) = literal {
                    *number
                } else {
                    return Err(CompileError("[AsmGenerator] Only number literal is supported in this context.".into()))
                };

                return Ok(literal)
            }
            AstNodeType::InfixExpression => {
                let infix = downcast!(InfixExpression, expression);
                let left = self.eval_expression(&infix.left).unwrap();
                let right = self.eval_expression(&infix.right).unwrap();

                let result = match infix.operator {
                    Operator::Plus => left + right,
                    Operator::Minus => left - right,
                    Operator::Divide => left / right,
                    Operator::Multiply => left * right,
                    Operator::Modulo => left % right,
                    Operator::BitwiseAnd => left & right,
                    Operator::BitwiseXor => left ^ right,
                    Operator::BitwiseOr => left | right,
                    _ => return Err(CompileError(format!("[AsmGenerator::eval_expression] Operator {:?} is not supported.", infix.operator))),
                };

                return Ok(result)
            },
            AstNodeType::PrefixExpression => {
                let prefix = downcast!(PrefixExpression, expression);
                let right = self.eval_expression(&prefix.right).unwrap();

                let result = match prefix.operator {
                    Operator::UnaryPlus => right,
                    Operator::UnaryMinus => -right,
                    Operator::BitwiseNot => !right,
                    _ => return Err(CompileError(format!("[AsmGenerator::eval_expression] Operator {:?} is not supported.", prefix.operator))),
                };

                return Ok(result)
            },
            _ => return Err(CompileError(format!("[AsmGenerator::eval_expression] {:?} is not expected while evaluating expression.", expression.get_type())))
        }
    }

    fn compile_statement(&self, code: &mut String, statement: &Box<dyn Statement>) -> Result<(), CompileError> {
        match statement.get_type() {
            AstNodeType::IfStatement => {

            },
            AstNodeType::ReturnStatement => {
                let return_statement = downcast!(ReturnStatement, statement);
                self.compile_expression(code, &return_statement.expression, "r10").unwrap();
                code.push_str("mov rax, r10");
                code.push_str("ret");
            },
            _ => return Err(CompileError(format!("[AsmGenerator] {:?} is not expected while compiling statement.", statement.get_type()))),
        }
        Ok(())
    }

    fn compile_expression(&self, code: &mut String, expression: &Box<dyn Expression>, reg: &str) -> Result<(), CompileError> {
        match expression.get_type() {
            AstNodeType::FnCallExpression => {
                let fncall = downcast!(FnCallExpression, expression);
                let mut argument_count: usize = 0;

                for argument in &fncall.arguments {
                    if argument_count > 3 {
                        return Err(CompileError("[AsmGenerator] only 4 arguments can be passed.".into()))
                    }

                    self.compile_expression(code, argument, "r10").unwrap();
                    code.push_str(&format!("mov {}, r10\n", AsmGenerator::get_register(argument_count)));

                    argument_count += 1;
                }

                code.push_str(&format!("call {}\n", fncall.identifier.to_string()));
            },
            AstNodeType::Identifier => {
                let identifier = downcast!(Identifier, expression);

                println!("nothing to do with identifier {}", identifier);
                code.push_str("nop\n");
            },
            AstNodeType::Literal => {
                let literal = downcast!(Literal, expression);
                let literal = if let Literal::Number(number) = literal {
                    *number
                } else {
                    return Err(CompileError("[AsmGenerator] Only number literal is supported in this context.".into()))
                };

                code.push_str(&format!("mov r10, {}\n", literal));
            },
            AstNodeType::PrefixExpression => {
                let prefix = downcast!(PrefixExpression, expression);
                self.compile_expression(code, &prefix.right, "r10").unwrap();

                let result = match prefix.operator {
                    Operator::UnaryMinus => {
                        code.push_str("neg r10\n");
                    },
                    Operator::BitwiseNot => {
                        code.push_str("not r10\n");
                    },
                    _ => return Err(CompileError(format!("[AsmGenerator::compile_expression] Operator {:?} is not supported.", prefix.operator))),
                };

                return Ok(result)
            },
            AstNodeType::InfixExpression => {
                let infix = downcast!(InfixExpression, expression);
                self.compile_expression(code, &infix.left, "r10").unwrap();
                self.compile_expression(code, &infix.right, "r11").unwrap();

                match infix.operator {
                    Operator::Plus => {
                        code.push_str("add r10, r11\n".into());
                    },
                    Operator::Minus => {
                        code.push_str("sub r10, r11\n".into());
                    },
                    Operator::Divide => {
                        code.push_str("mov rax, r10\n".into());
                        code.push_str("cqo\n".into());
                        code.push_str("idiv r11\n".into());
                        code.push_str("mov r10, rax\n".into());

                    },
                    Operator::Multiply => {
                        code.push_str("mov rax, r10\n".into());
                        code.push_str("imul r11\n".into());
                        code.push_str("mov r10, rax\n".into());
                    },
                    Operator::Modulo => {
                        code.push_str("mov rax, r10\n".into());
                        code.push_str("cdq\n".into());
                        code.push_str("idiv r11\n".into());
                        code.push_str("mov eax, edx\n".into());
                        code.push_str("mov r10, rax\n".into());
                    },
                    Operator::BitwiseAnd => {
                        code.push_str("and r10, r11\n".into());
                    },
                    Operator::BitwiseXor => {
                        code.push_str("xor r10, r11\n".into());
                    },
                    Operator::BitwiseOr => {
                        code.push_str("or  r10, r11\n".into());
                    },
                    _ => return Err(CompileError(format!("[AsmGenerator::compile_expression] Operator {:?} is not supported.", infix.operator))),
                };
            },
            _ => return Err(CompileError(format!("[AsmGenerator::compile_expression] {:?} is not expected while compiling expression.", expression.get_type())))
        }

        Ok(())
    }
}