use crate::lexer::{Token, Keyword};
use crate::parser::ast::{AbstractSyntaxTree, Statement, Expression, ExpressionStatement};
use crate::parser::pratt::PrattParser;
pub use crate::parser::{r#if::IfStatement, r#let::LetStatement, r#return::ReturnStatement, def::DefStatement};
use crate::error::ParseError;

pub mod ast;
pub mod pratt;
mod r#if;
mod r#let;
mod r#return;
mod def;

pub struct  Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            pos: 0,
        }
    }

    fn next(&self, n: usize) -> Option<Token> {
        if (self.pos + n) < self.tokens.len() {
            Some(self.tokens[self.pos + n].clone())
        } else {
            None
        }
    }

    pub fn parse_statement(&mut self) -> Result<Option<Box<dyn Statement>>, ParseError> {
        let token = &self.tokens[self.pos];

        let statement: Option<Box<dyn Statement>> = match token {
            Token::Keyword(keyword) => {
                match keyword {
                    Keyword::Def => {
                        self.pos += 1;
                        Some(Box::new(DefStatement::parse(self).unwrap()))
                    },
                    Keyword::If => {
                        self.pos += 1;
                        Some(Box::new(IfStatement::parse(self).unwrap()))
                    },
                    Keyword::Let => {
                        self.pos += 1;
                        Some(Box::new(LetStatement::parse(self).unwrap()))
                    },
                    Keyword::Return => {
                        self.pos += 1;
                        Some(Box::new(ReturnStatement::parse(self).unwrap()))
                    },
                    _ => {
                        self.pos += 1;
                        return Err(ParseError(format!("[Parser::parse_statement] unexpected keyword {:?}", keyword)));
                    },
                }
            },
            Token::EOF => {
                None
            },
            _ => Some(Box::new(ExpressionStatement::parse(self).unwrap())),
        };

        Ok(statement)
    }

    pub fn parse_all(&mut self) -> AbstractSyntaxTree {
        let mut ast = AbstractSyntaxTree::new();

        loop {
            let statement = self.parse_statement().unwrap();

            if let Some(statement) = statement {
                ast.statements.push(statement);
            } else {
                break;
            }
        }

        ast
    }
}
