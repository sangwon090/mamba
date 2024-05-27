use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod irgen;
pub mod lexer;
pub mod parser;
pub mod error;
pub mod types;

use lexer::{Lexer, Token};

#[derive(Serialize, Deserialize)]
pub struct TokenOutput {
    pub tokens: Vec<Token>,
}

#[derive(Serialize, Deserialize)]
pub enum MambaOutput {
    Tokens(TokenOutput),
}

#[derive(Serialize, Deserialize)]
pub enum MambaProblem {
    Error(String),
    Warning(String),
    Info(String),
}

#[derive(Serialize, Deserialize)]
pub struct MambaResult {
    pub output: Option<MambaOutput>,
    pub problems: Option<Vec<MambaProblem>>,
}

#[wasm_bindgen]
pub fn lex(string: String) -> String {
    let mut lexer: Lexer = Lexer::new(string);

    let result = match lexer.get_tokens() {
        Ok(tokens) => {
            MambaResult {
                output: Some(MambaOutput::Tokens(TokenOutput { tokens })),
                problems: None,
            }
        },
        Err(error) => {
            MambaResult {
                output: None,
                problems: Some(vec![MambaProblem::Error(error.0)]),
            }
        }
    };

    serde_json::to_string(&result).unwrap()
}