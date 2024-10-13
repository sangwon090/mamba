pub mod expr;

use std::collections::HashMap;

use crate::parser::{DefStatement, Expression, ExternStatement, IfStatement, LetStatement, ReturnStatement, Statement, AST};
use crate::lexer::Literal;
use crate::error::IRGenError;
pub use expr::generate_expr;

pub struct IRGen {
    ast: AST,
    context: GlobalContext,
}

#[derive(Default)]
pub struct GlobalContext {
    global_var: HashMap<String, Literal>,
    fn_decl: HashMap<String, Vec<String>>,
    label_idx: u64,
}

#[derive(Default)]
pub struct ScopedContext {
    local_var: HashMap<String, i32>,
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
        let mut scoped_ctx = ScopedContext::default();

        result += include_str!("stub.ll");
        result += &self.ast.iter()
            .map(|stmt| IRGen::generate_stmt(&mut self.context, &mut scoped_ctx, stmt).unwrap())
            .collect::<Vec<String>>()
            .join("");

        Ok(result)
    }

    fn generate_stmt(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &Statement) -> Result<String, IRGenError> {
        let mut result = String::new();

        match stmt {
            Statement::Let(stmt) => result += &IRGen::generate_global_variable(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::Def(stmt) => result += &IRGen::generate_def(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::If(stmt) => result += &IRGen::generate_if(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::Return(stmt) => result += &IRGen::generate_ret(global_ctx, scoped_ctx, stmt).unwrap(),
            Statement::Expression(_stmt) => todo!(),
            Statement::Extern(stmt) => result += &IRGen::generate_extern(global_ctx, scoped_ctx, stmt).unwrap(),
        }

        Ok(result)
    }

    fn generate_global_variable(global_ctx: &mut GlobalContext, _scoped_ctx: &mut ScopedContext, stmt: &LetStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        
        if let Expression::Literal(literal) = &stmt.expr {
            match literal {
                Literal::Integer(n) => {
                    global_ctx.global_var.insert(stmt.ident.clone(), literal.clone());
                    result += &format!("@{} = global i32 {}\n", stmt.ident.clone(), *n);
                },
                Literal::String(s) => {
                    global_ctx.global_var.insert(stmt.ident.clone(), literal.clone());
                    result += &format!("@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"\n", stmt.ident, s.len() + 1, s);
                },
                _ => todo!(),
            }
        } else {
            eprintln!("{:?} in let expression is not implemented.", stmt.expr);
        }

        Ok(result)
    }

    fn generate_def(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &DefStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        result += &format!("define i32 @{}(", stmt.name);
        
        result += &stmt.params.iter()
        .map(|(ident, dtype)| {
                scoped_ctx.local_var.insert(ident.to_string(), 0);
                format!("{} %{}", dtype.to_mnemonic(), ident)
            })
            .collect::<Vec<String>>()
            .join(", ");

        result += ") {\n";

        global_ctx.fn_decl.insert(stmt.name.to_string(), stmt.params.iter().map(|(_, dtype)| dtype.to_mnemonic().into()).collect::<Vec<String>>());

        // add statements
        result += &stmt.stmts.iter()
            .map(|stmt| IRGen::generate_stmt(global_ctx, scoped_ctx, stmt).unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        result += "}\n";

        Ok(result)
    }


    fn generate_if(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &IfStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        let then_idx = global_ctx.get_label();
        let else_idx = if stmt.r#else.is_some() { global_ctx.get_label() } else { 0 };
        //let cont_idx = global_ctx.get_label();

        // process condition
        let (expr_code, expr_idx) = generate_expr(global_ctx, scoped_ctx, &stmt.condition).unwrap();
        result += &expr_code;
        result += &format!("br i1 %{}, label %l{}, label %l{}\n", expr_idx, then_idx, else_idx);
        
        // process then
        result += &format!("l{}:\n", then_idx);
        result += &IRGen::generate_stmt(global_ctx, scoped_ctx, &stmt.then).unwrap();

        // process else
        if let Some(stmt) = &stmt.r#else {
            result += &format!("l{}:\n", else_idx);
            result += &IRGen::generate_stmt(global_ctx, scoped_ctx, stmt).unwrap();
    
        }
        
        // continue
        //result += &format!("{}:\n", cont_idx);

        result += "\n";
        Ok(result)
    }

    fn generate_ret(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &ReturnStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        let (expr_code, expr_idx) = generate_expr(global_ctx, scoped_ctx, &stmt.expr).unwrap();
        
        if expr_idx.is_empty() {
            result += &expr_code;
            result += "ret i32 0\n";
        } else {
            result += &expr_code;
            result += &format!("ret i32 %{}\n", expr_idx);
        }

        Ok(result)
    }

    fn generate_literal(global_ctx: &mut GlobalContext, _scoped_ctx: &mut ScopedContext, literal: &Literal) -> Result<(String, u64), IRGenError> {
        let mut result = String::new();

        let idx = if let Literal::Integer(n) = literal {
            let ptr_idx = global_ctx.get_label();
            let ret_idx = global_ctx.get_label();

            result += &format!("%{} = alloca i32, align 4\n", ptr_idx);
            result += &format!("store i32 {}, ptr %{}, align 4\n", n, ptr_idx);
            result += &format!("%{} = load i32, ptr %{}, align 4\n", ret_idx, ptr_idx);
            
            ret_idx
        } else {
            panic!("`{:?}` not supported for literal", literal);
        };

        Ok((result, idx))
    }

    fn generate_extern(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &ExternStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        global_ctx.fn_decl.insert(stmt.name.to_string(), stmt.params.iter().map(|(_, dtype)| dtype.to_mnemonic().into()).collect::<Vec<String>>());
        result += &format!("declare {} @{}({}) nounwind\n", stmt.r#type.to_mnemonic(), stmt.name, stmt.params.iter().map(|(_, dtype)| dtype.to_mnemonic().into()).collect::<Vec<String>>().join(", "));

        Ok(result)
    }
}