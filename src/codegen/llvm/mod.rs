pub mod expr;

use std::collections::{HashMap};

use crate::parser::{DefStatement, IfStatement, LetStatement, ReturnStatement};
use crate::parser::ast::{AbstractSyntaxTree, AstNodeType, Expression, ExpressionStatement, FnCallExpression, InfixExpression, Operator, Statement};
use crate::{error::IRGenError};
use crate::lexer::{Identifier, Literal};

pub use expr::generate_expr;

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
                eprintln!("cannot generate code for `{:?}`.", literal);
            }
        } else {
            eprintln!("`{:?}` in let expression is not implemented.", stmt.expression.get_type());
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
        let (expr_code, expr_idx) = generate_expr(global_ctx, scoped_ctx, &stmt.condition).unwrap();
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

        let (expr_code, expr_idx) = generate_expr(global_ctx, scoped_ctx, &stmt.expression).unwrap();
        
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
}