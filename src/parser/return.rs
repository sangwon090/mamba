use crate::parser::{Parser, PrattParser, Token};
use crate::parser::ast::Expression;
use crate::error::ParseError;
use super::ast::Parsable;
use super::pratt::Precedence;
use std::fmt;

pub struct ReturnStatement {
    pub expression: Expression,
}

impl Parsable for ReturnStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let expression = PrattParser::parse_expression(parser, Precedence::Lowest).unwrap();

        parser.pos += 1;

        if let Some(token) = parser.next(0) {
            if let Token::Semicolon = token {
                parser.pos += 1;
            } else {
                return Err(ParseError(format!("[ReturnStatement] Expected `;`, found {token:?}")));
            }
        } else {
            return Err(ParseError("[ReturnStatement] insufficient tokens".into()));
        }

        Ok(ReturnStatement {
            expression,
        })
    } 
}

impl fmt::Debug for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ expression: {:?} }}", self.expression)
    }
}