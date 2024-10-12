pub mod expr;

use std::collections::HashMap;

use crate::parser::{DefStatement, IfStatement, LetStatement, ReturnStatement};
use crate::parser::ast::{AstNodeType, Expression, Literal, Statement, AST};
use crate::error::IRGenError;
pub use expr::generate_expr;

pub struct IRGen {
    ast: AST,
    context: GlobalContext,
}

#[derive(Default)]
pub struct GlobalContext {
    global_var: HashMap<String, i64>,
    fn_decl: HashMap<String, Vec<String>>,
    label_idx: u64,
}

#[derive(Default)]
pub struct ScopedContext {
    local_var: HashMap<String, i64>,
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
        result += &self.ast.stmts.iter()
            .map(|stmt| IRGen::generate_stmt(&mut self.context, &mut scoped_ctx, stmt).unwrap())
            .collect::<Vec<String>>()
            .join("");

        Ok(result)
    }

    fn generate_stmt(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &Box<dyn Statement>) -> Result<String, IRGenError> {
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

    fn generate_global_variable(global_ctx: &mut GlobalContext, _scoped_ctx: &mut ScopedContext, stmt: &LetStatement) -> Result<String, IRGenError> {
        let mut result = String::new();
        
        if let Expression::Literal(literal) = &stmt.expr {
            if let Literal::Integer(n) = literal {
                global_ctx.global_var.insert(stmt.ident.clone(), *n);
                result += &format!("@{} = global i64 {}\n", stmt.ident.clone(), *n);
            } else {
                eprintln!("cannot generate code for {:?}", literal);
            }
        } else {
            eprintln!("{:?} in let expression is not implemented.", stmt.expr);
        }

        Ok(result)
    }

    fn generate_def(global_ctx: &mut GlobalContext, scoped_ctx: &mut ScopedContext, stmt: &DefStatement) -> Result<String, IRGenError> {
        let mut result = String::new();

        result += &format!("define i64 @{}(", stmt.name);
        
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
            result += "ret i64 0\n";
        } else {
            result += &expr_code;
            result += &format!("ret i64 %{}\n", expr_idx);
        }

        Ok(result)
    }

    fn generate_literal(global_ctx: &mut GlobalContext, _scoped_ctx: &mut ScopedContext, literal: &Literal) -> Result<(String, u64), IRGenError> {
        let mut result = String::new();

        let idx = if let Literal::Integer(n) = literal {
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
}