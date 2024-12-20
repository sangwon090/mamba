
pub use def::{DefStatement, parse_def};
pub use expr_stmt::{ExpressionStatement, parse_expr_stmt};
pub use r#extern::{ExternStatement, parse_extern};
pub use r#if::{IfStatement, IfBranch, parse_if};
pub use r#let::{LetStatement, parse_let};
pub use r#return::{ReturnStatement, parse_return};

use std::fmt;

mod def;
mod expr_stmt;
mod r#extern;
mod r#if;
mod r#let;
mod r#return;

pub enum Statement {
    Expression(ExpressionStatement),
    Def(DefStatement),
    If(IfStatement),
    Let(LetStatement),
    Return(ReturnStatement),
    Extern(ExternStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Expression(stmt) => write!(f, "{}", stmt),
            Statement::Def(stmt) => write!(f, "{}", stmt),
            Statement::If(stmt) => write!(f, "{}", stmt),
            Statement::Let(stmt) => write!(f, "{}", stmt),
            Statement::Return(stmt) => write!(f, "{}", stmt),
            Statement::Extern(stmt) => write!(f, "{}", stmt),
        }
    }
}