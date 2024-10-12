use crate::lexer::{Token, Keyword};
use crate::parser::pratt::PrattParser;
use crate::error::ParseError;

pub mod pratt;
mod expression;
mod statement;

pub use expression::*;
pub use statement::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

pub type AST = Vec<Statement>;

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

    pub fn parse_stmt(&mut self) -> Result<Option<Statement>, ParseError> {
        let token = &self.tokens[self.pos];

        let stmt: Option<Statement> = match token {
            Token::Keyword(keyword) => {
                match keyword {
                    Keyword::Def => {
                        self.pos += 1;
                        Some(Statement::Def(parse_def(self).unwrap()))
                    },
                    Keyword::If => {
                        self.pos += 1;
                        Some(Statement::If(parse_if(self).unwrap()))
                    },
                    Keyword::Let => {
                        self.pos += 1;
                        Some(Statement::Let(parse_let(self).unwrap()))
                    },
                    Keyword::Return => {
                        self.pos += 1;
                        Some(Statement::Return(parse_return(self).unwrap()))
                    },
                    _ => {
                        self.pos += 1;
                        return Err(ParseError(format!("[Parser::parse_stmt] unexpected keyword {:?}", keyword)));
                    },
                }
            },
            Token::EOF => {
                None
            },
            _ => Some(Statement::Expression(parse_expr_stmt(self).unwrap())),
        };

        Ok(stmt)
    }

    pub fn parse_all(&mut self) -> AST {
        let mut ast = AST::new();

        loop {
            let stmt = self.parse_stmt ().unwrap();

            if let Some(stmt) = stmt {
                ast.push(stmt);
            } else {
                break;
            }
        }

        ast
    }
}
