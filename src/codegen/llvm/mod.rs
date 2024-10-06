use std::collections::{HashMap};

use crate::parser::{DefStatement, IfStatement, LetStatement, ReturnStatement};
use crate::parser::ast::{AbstractSyntaxTree, AstNodeType, Expression, ExpressionStatement, FnCallExpression, InfixExpression, Operator, Statement};
use crate::{error::IRGenError};
use crate::lexer::{Identifier, Literal};

pub struct IRGen {
    ast: AbstractSyntaxTree,
    context: GlobalContext,
}

pub struct GlobalContext {
    global_var: HashMap<String, i64>,
    fn_decl: HashMap<String, Vec<String>>,
    label_idx: u64,
}

pub struct ScopedContext {
    local_var: HashMap<String, i64>,
}

impl GlobalContext {
    pub fn new() -> GlobalContext {
        GlobalContext {
            global_var: HashMap::new(),
            fn_decl: HashMap::new(),
            label_idx: 0,        
        }
    }

    pub fn get_label(&mut self) -> u64 {
        let result = self.label_idx;
        self.label_idx += 1;
        result
    }
}

impl ScopedContext {
    pub fn new() -> ScopedContext {
        ScopedContext {
            local_var: HashMap::new(),
        }
    }
}

impl IRGen {
    pub fn new(ast: AbstractSyntaxTree) -> IRGen {
        IRGen {
            ast,
            context: GlobalContext::new(),
        }
    }

    pub fn generate_ir(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();
        let mut scoped_ctx = ScopedContext::new();

        result += &include_str!("stub.ll");
        result += &self.ast.statements.iter()
            .map(|stmt| IRGen::generate_statement(&mut self.context, &mut scoped_ctx, &stmt).unwrap())
            .collect::<Vec<String>>()
            .join("");

        /*
        self.ast.statements.retain(|statement| {
            if statement.get_type() == AstNodeType::LetStatement {
                let let_statement = statement.as_any().downcast_ref::<LetStatement>().unwrap();
                
                if let_statement.expression.get_type() == AstNodeType::Literal {
                    let literal = let_statement.expression.as_any().downcast_ref::<Literal>().unwrap();
                    if let Literal::Number(n) = literal {
                        self.global_var.insert(let_statement.identifier.0.clone(), *n);
                        false
                    } else {
                        println!("cannot generate code for `{:?}`.", literal);
                        false
                    }
                } else {
                    println!("`{:?}` in let expression is not implemented.", let_statement.expression.get_type());
                    false
                }
            } else {
                true
            }
        });

        for (ident, value) in &self.global_var {
            println!("* {}: {}", ident, value);
            result += &format!("@{} = global i64 {}\n", ident, value);
        }

        println!("");
        
        for statement in &self.ast.statements {
            println!("- {}", statement.to_string());
        }
*/
        Ok(result)
    }

    fn generate_statement(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &Box<dyn Statement>) -> Result<String, IRGenError> {
        let mut result = String::new();

        match stmt.get_type() {
            // TODO: check if local or global
            AstNodeType::LetStatement => result += &IRGen::generate_global_variable(global_ctx, scoped_ctx, stmt.as_any().downcast_ref::<LetStatement>().unwrap()).unwrap(),
            AstNodeType::DefStatement => result += &IRGen::generate_def(global_ctx, scoped_ctx, stmt.as_any().downcast_ref::<DefStatement>().unwrap()).unwrap(),
            AstNodeType::IfStatement => result += &IRGen::generate_if(global_ctx, scoped_ctx, stmt.as_any().downcast_ref::<IfStatement>().unwrap()).unwrap(),
            AstNodeType::ReturnStatement => result += &IRGen::generate_ret(global_ctx, scoped_ctx, stmt.as_any().downcast_ref::<ReturnStatement>().unwrap()).unwrap(),
            _ => {},
        }

        Ok(result)
    }

    fn generate_global_variable(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &LetStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        
        if stmt.expression.get_type() == AstNodeType::Literal {
            let literal = stmt.expression.as_any().downcast_ref::<Literal>().unwrap();
            if let Literal::Number(n) = literal {
                global_ctx.global_var.insert(stmt.identifier.0.clone(), *n);
                result += &format!("@{} = global i64 {}\n", stmt.identifier.0.clone(), *n);
            } else {
                println!("cannot generate code for `{:?}`.", literal);
            }
        } else {
            println!("`{:?}` in let expression is not implemented.", stmt.expression.get_type());
        }

        Ok(result)
    }

    fn generate_def(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &DefStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        result += &format!("define i64 @{}(", stmt.name.to_string());
        
        result += &stmt.parameters.iter()
        .map(|(ident, dtype)| {
                scoped_ctx.local_var.insert(ident.to_string(), 0);
                format!("{} %{}", dtype.clone().to_mnemonic(), ident.to_string())
            })
            .collect::<Vec<String>>()
            .join(", ");

        result += ") {\n";

        global_ctx.fn_decl.insert(stmt.name.to_string(), stmt.parameters.iter().map(|(_, dtype)| dtype.clone().to_mnemonic().into()).collect::<Vec<String>>());

        // add statements
        result += &stmt.statements.iter()
            .map(|stmt| IRGen::generate_statement(global_ctx, scoped_ctx, stmt).unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        result += "}\n";

        Ok(result)
    }


    fn generate_if(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &IfStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        let then_idx = global_ctx.get_label();
        let else_idx = if let Some(_) = &stmt.r#else { global_ctx.get_label() } else { 0 };
        //let cont_idx = global_ctx.get_label();

        // process condition
        let (expr_code, expr_idx) = IRGen::generate_expr(global_ctx, scoped_ctx, &stmt.condition).unwrap();
        result += &expr_code;
        result += &format!("br i1 %{}, label %l{}, label %l{}\n", expr_idx, then_idx, else_idx);
        
        // process then
        result += &format!("l{}:\n", then_idx);
        result += &IRGen::generate_statement(global_ctx, scoped_ctx, &stmt.then).unwrap();

        // process else
        if let Some(stmt) = &stmt.r#else {
            result += &format!("l{}:\n", else_idx);
            result += &IRGen::generate_statement(global_ctx, scoped_ctx, &stmt).unwrap();
    
        }
        
        // continue
        //result += &format!("{}:\n", cont_idx);

        result += "\n";
        Ok(result)
    }

    fn generate_ret(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &ReturnStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        let (expr_code, expr_idx) = IRGen::generate_expr(global_ctx, scoped_ctx, &stmt.expression).unwrap();
        
        if expr_idx == "" {
            result += &expr_code;
            result += "ret i64 0\n";
        } else {
            result += &expr_code;
            result += &format!("ret i64 %{}\n", expr_idx);
        }

        Ok(result)
    }

    fn generate_literal(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, literal: &Literal) -> Result<(String, u64), IRGenError> {
        let mut result = String::new();

        let idx = if let Literal::Number(n) = literal {
            let ptr_idx = global_ctx.get_label();
            let ret_idx = global_ctx.get_label();

            result += &format!("%{} = alloca i64, align 4\n", ptr_idx);
            result += &format!("store i64 {}, ptr %{}, align 4\n", n, ptr_idx);
            result += &format!("%{} = load i64, ptr %{}, align 4\n", ret_idx, ptr_idx);
            
            ret_idx
        } else {
            panic!("`{:?}` not supported for literal", literal);
        };

        Ok((result, idx))
    }

    fn generate_ident(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, ident: &Identifier) -> Result<(String, String), IRGenError> {
        let mut result = String::new();

        let idx = if scoped_ctx.local_var.contains_key(&ident.0) {
            ident.0.clone()
        } else if global_ctx.global_var.contains_key(&ident.0) {
            ident.0.clone()
        } else {
            panic!("Unable to find identifier {}", ident.0);
        };

        Ok((result, idx))
    }

    fn generate_expr(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, expr: &Box<dyn Expression>) -> Result<(String, String), IRGenError> {
        let mut result = String::new();

        let idx = match expr.get_type() {
            AstNodeType::InfixExpression => {
                let expr = expr.as_any().downcast_ref::<InfixExpression>().unwrap();
                
                let left_idx = match expr.left.get_type() {
                    AstNodeType::InfixExpression | AstNodeType::PrefixExpression | AstNodeType::FnCallExpression => {
                        let (expr_code, expr_idx) = IRGen::generate_expr(global_ctx, scoped_ctx, &expr.left).unwrap();
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
                        let (expr_code, expr_idx) = IRGen::generate_expr(global_ctx, scoped_ctx, &expr.right).unwrap();
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
                    let (code, idx) = IRGen::generate_expr(global_ctx, scoped_ctx, &expr).unwrap();
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
}