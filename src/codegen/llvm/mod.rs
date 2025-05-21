pub mod expr;
pub mod types;

use std::borrow::Borrow;
use std::collections::HashMap;

use crate::parser::{DefStatement, Expression, ExternStatement, IfBranch, IfStatement, LetStatement, ReturnStatement, Statement, WhileStatement, AST};
use crate::lexer::Literal;
use crate::error::IRGenError;
use crate::types::{DataType, SignedInteger};
pub use expr::generate_expr;
use types::cast;

pub struct IRGen {
    ast: AST,
    context: GlobalContext,
}

#[derive(Default)]
pub struct GlobalContext {
    global_var: HashMap<String, Literal>,
    fn_decl: HashMap<String, (Vec<String>, DataType)>,
    label_idx: u64,
}

pub enum ScopedContext {
    FnDecl(HashMap<String, DataType>, DataType),
    Scope(HashMap<String, Literal>),
}

impl GlobalContext {
    pub fn get_label(&mut self) -> u64 {
        let result = self.label_idx;
        self.label_idx += 1;
        result
    }
}

impl IRGen {
    pub fn new(ast: AST) -> IRGen {
        IRGen {
            ast,
            context: GlobalContext::default(),
        }
    }

    pub fn generate_ir(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();
        let mut scoped_ctx = Vec::new();

        result += include_str!("stub.ll");
        result += &self.ast.iter()
            .map(|stmt| IRGen::generate_global_stmt(&mut self.context, &mut scoped_ctx, stmt).unwrap())
            .collect::<Vec<String>>()
            .join("");

        Ok(result)
    }

    fn generate_global_stmt(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, stmt: &Statement) -> Result<String, IRGenError> {
        let mut result = String::new();

        match stmt {
            Statement::Let(stmt) => result += &IRGen::generate_global_variable(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::Def(stmt) => result += &IRGen::generate_def(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::Extern(stmt) => result += &IRGen::generate_extern(global_ctx, scoped_ctx, stmt).unwrap(),
            _ => panic!("{} cannot be global", stmt),
        }

        Ok(result)
    }

    fn generate_local_stmt(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, stmt: &Statement) -> Result<String, IRGenError> {
        let mut result = String::new();

        match stmt {
            Statement::Let(stmt) => result += &IRGen::generate_local_variable(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::If(stmt) => result += &IRGen::generate_if(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::Return(stmt) => result += &IRGen::generate_ret(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::Expression(stmt) => result += &generate_expr(global_ctx, scoped_ctx, &stmt.expr).unwrap().0,
            Statement::While(stmt) => result += &IRGen::generate_while(global_ctx, scoped_ctx, stmt).unwrap(),
            _ => panic!("{} cannot be local", stmt),
        }

        Ok(result)
    }

    fn generate_global_variable(global_ctx: &mut GlobalContext, _scoped_ctx: &mut Vec<ScopedContext>, stmt: &LetStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        
        if let Expression::Literal((literal, _)) = &stmt.expr {
            match literal {
                Literal::SignedInteger((n, dtype)) => {
                    global_ctx.global_var.insert(stmt.ident.clone(), literal.clone());
                    result += &format!("@{} = global {} {}\n", stmt.ident.clone(), dtype.to_mnemonic(), n);
                },
                Literal::UnsignedInteger((n, dtype)) => {
                    global_ctx.global_var.insert(stmt.ident.clone(), literal.clone());
                    result += &format!("@{} = global {} {}\n", stmt.ident.clone(), dtype.to_mnemonic(), n);
                },
                Literal::String(s) => {
                    global_ctx.global_var.insert(stmt.ident.clone(), literal.clone());
                    result += &format!("@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", stmt.ident, s.len() + 1, s);
                },
            }
        } else {
            eprintln!("{:?} in let expression is not implemented.", stmt.expr);
        }

        Ok(result)
    }

    fn generate_local_variable(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, stmt: &LetStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        
        if let Expression::Literal((literal, _)) = &stmt.expr {
            if let ScopedContext::Scope(scope) = scoped_ctx.last_mut().unwrap() {
                scope.insert(stmt.ident.clone(), literal.clone());
            } else {
                panic!("expected scope, something else found :(");
            }

            match literal {
                Literal::SignedInteger((n, dtype)) => {
                    result += &format!("%{} = alloca {}, align 4\n", stmt.ident.clone(), dtype.to_mnemonic());
                    result += &format!("store {} {}, ptr %{}, align 4\n", dtype.to_mnemonic(), n, stmt.ident.clone());
                },
                Literal::UnsignedInteger((n, dtype)) => {
                    result += &format!("%{} = alloca {}, align 4\n", stmt.ident.clone(), dtype.to_mnemonic());
                    result += &format!("store {} {}, ptr %{}, align 4\n", dtype.to_mnemonic(), n, stmt.ident.clone());
                },
                Literal::String(s) => {
                    result += &format!("%{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", stmt.ident, s.len() + 1, s);
                },
            }
        } else {
            eprintln!("{:?} in let expression is not implemented.", stmt.expr);
        }

        Ok(result)
    }

    fn generate_def(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, stmt: &DefStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        result += &format!("define {} @{}(", stmt.r#type.to_mnemonic(), stmt.name);
        
        let mut params: HashMap<String, DataType> = HashMap::new();

        result += &stmt.params.iter()
        .map(|(ident, dtype)| {
                params.insert(ident.to_string(), *dtype);
                format!("{} %{}", dtype.to_mnemonic(), ident)
            })
            .collect::<Vec<String>>()
            .join(", ");

        result += ") {\n";

        global_ctx.fn_decl.insert(stmt.name.to_string(), (stmt.params.iter().map(|(_, dtype)| dtype.to_mnemonic().into()).collect::<Vec<String>>(), stmt.r#type));
        scoped_ctx.push(ScopedContext::FnDecl(params, stmt.r#type));

        // add statements
        scoped_ctx.push(ScopedContext::Scope(HashMap::new()));
        result += &stmt.stmts.iter()
            .map(|stmt| IRGen::generate_local_stmt(global_ctx, scoped_ctx, stmt).unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        result += "}\n";

        scoped_ctx.pop(); // pop scope
        scoped_ctx.pop(); // pop fn_decl

        Ok(result)
    }


    fn generate_if(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, stmt: &IfStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        let then_idx = global_ctx.get_label();
        let else_idx = global_ctx.get_label();

        // process condition
        let (expr_code, expr_idx, _expr_dtype) = generate_expr(global_ctx, scoped_ctx, &stmt.condition).unwrap();
        result += &expr_code;
        result += &format!("br i1 {}, label %l{}, label %l{}\n", expr_idx, then_idx, else_idx);

        if let IfBranch::None = *stmt.r#else {
            result += &format!("l{}:\n", then_idx);
            for then_stmt in &stmt.then {
                result += &IRGen::generate_local_stmt(global_ctx, scoped_ctx, then_stmt).unwrap();
            }

            result += &format!("l{}:\n", else_idx);
        } else {
            // process then
            result += &format!("l{}:\n", then_idx);
            for then_stmt in &stmt.then {
                result += &IRGen::generate_local_stmt(global_ctx, scoped_ctx, then_stmt).unwrap();
            }
            // process else
            match stmt.r#else.borrow() {
                IfBranch::Elif(stmt) => {
                    result += &format!("l{}:\n", else_idx);
                    result += &Self::generate_if(global_ctx, scoped_ctx, &stmt).unwrap();
                },
                IfBranch::Else(stmt) => {
                    result += &format!("l{}:\n", else_idx);
                    for else_stmt in stmt {
                        result += &IRGen::generate_local_stmt(global_ctx, scoped_ctx, &else_stmt).unwrap();
                    }
                },
                IfBranch::None => { },
            }
        }


        result += "\n";
        Ok(result)
    }

    fn generate_while(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, stmt: &WhileStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        

        result += "\n";
        Ok(result)
    }

    fn generate_ret(global_ctx: &mut GlobalContext, scoped_ctx: &mut Vec<ScopedContext>, stmt: &ReturnStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        let (code, idx, dtype) = generate_expr(global_ctx, scoped_ctx, &stmt.expr).unwrap();
        
        let ret_dtype: Vec<DataType> = scoped_ctx.iter().filter_map(|ctx| {
            if let ScopedContext::FnDecl(_, dtype) = ctx {
                Some(*dtype)
            } else {
                None
            }
        }).collect();

        let ret_dtype = ret_dtype.last().unwrap().clone(); // shitty code

        let (idx, cast_code) = if ret_dtype != dtype {
            let (casted_idx, cast_code, _) = cast()[&(dtype, ret_dtype)](global_ctx, &idx);
            (casted_idx, cast_code)
        } else {
            (idx, String::new())
        };

        if idx.is_empty() { // what???
            result += &code;
            result += "ret\n";
        } else {
            result += &code;
            result += &cast_code;
            result += &format!("ret {} {}\n", ret_dtype.to_mnemonic(), idx);
        }

        Ok(result)
    }

    fn generate_literal(global_ctx: &mut GlobalContext, _scoped_ctx: &mut Vec<ScopedContext>, literal: &Literal) -> Result<(String, u64), IRGenError> {
        let mut result = String::new();

        let (idx, _dtype) = match literal {
            Literal::SignedInteger((n, _dtype)) => {
                let ptr_idx = global_ctx.get_label();
                let ret_idx = global_ctx.get_label();
    
                result += &format!("%{} = alloca i32, align 4\n", ptr_idx);
                result += &format!("store i32 {}, ptr %{}, align 4\n", n, ptr_idx);
                result += &format!("%{} = load i32, ptr %{}, align 4\n", ret_idx, ptr_idx);
                
                (ret_idx, DataType::SignedInteger(SignedInteger::i32))
            },
            Literal::UnsignedInteger((n, _dtype)) => {
                let ptr_idx = global_ctx.get_label();
                let ret_idx = global_ctx.get_label();
    
                result += &format!("%{} = alloca u32, align 4\n", ptr_idx);
                result += &format!("store u32 {}, ptr %{}, align 4\n", n, ptr_idx);
                result += &format!("%{} = load u32, ptr %{}, align 4\n", ret_idx, ptr_idx);
                
                (ret_idx, DataType::SignedInteger(SignedInteger::i32))
            }
            Literal::String(s) => {
                let ptr_idx = global_ctx.get_label();
                result += &format!("%{} = alloca [{} x i8], align 4\n", ptr_idx, s.len() + 1);
                result += &format!("store [{} x i8] c\"{}\\00\", ptr %{}, align 4\n", s.len() + 1, s, ptr_idx);

                (ptr_idx, DataType::str)
            }
        };

        Ok((result, idx))
    }

    fn generate_extern(global_ctx: &mut GlobalContext, _scoped_ctx: &mut Vec<ScopedContext>, stmt: &ExternStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        global_ctx.fn_decl.insert(stmt.name.to_string(), (stmt.params.iter().map(|(_, dtype)| dtype.to_mnemonic().into()).collect::<Vec<String>>(), stmt.r#type));
        result += &format!("declare {} @{}({}) nounwind\n", stmt.r#type.to_mnemonic(), stmt.name, stmt.params.iter().map(|(_, dtype)| dtype.to_mnemonic().into()).collect::<Vec<String>>().join(", "));

        Ok(result)
    }
}