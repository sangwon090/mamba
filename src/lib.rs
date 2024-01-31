use parser::ast::{AbstractSyntaxTree, Expression};
use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

pub mod irgen;
pub mod lexer;
pub mod parser;
pub mod error;
pub mod types;

use lexer::Lexer;
use parser::Parser;

#[wasm_bindgen]
pub fn lex(string: String) -> String {
    let mut lexer: Lexer = Lexer::new(string);
    let tokens = lexer.get_tokens().unwrap();

    serde_json::to_string(&tokens).unwrap()
}