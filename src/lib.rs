use wasm_bindgen::prelude::*;

pub mod irgen;
pub mod lexer;
pub mod parser;
pub mod error;
pub mod types;

use lexer::Lexer;

#[wasm_bindgen]
pub fn lex(string: String) -> String {
    let mut lexer: Lexer = Lexer::new(string);
    let tokens = lexer.get_tokens().unwrap();

    serde_json::to_string(&tokens).unwrap()
}