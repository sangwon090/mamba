use std::collections::{HashMap};

use crate::parser::LetStatement;
use crate::parser::ast::{AbstractSyntaxTree, AstNodeType, Expression};
use crate::{error::IRGenError};
use crate::lexer::Literal;

pub struct IRGen {
    ast: AbstractSyntaxTree,
    global_var: HashMap<String, i64>
}

impl IRGen {
    pub fn new(ast: AbstractSyntaxTree) -> IRGen {
        IRGen {
            ast,
            global_var: HashMap::new(),
        }
    }

    pub fn generate_ir(&mut self) -> Result<String, IRGenError> {
        let mut result: String = String::new();

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

        Ok(result)
    }
}