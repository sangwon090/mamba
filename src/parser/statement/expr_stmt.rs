use crate::{error::ParseError, lexer::Token, parser::{pratt::{PrattParser, Precedence}, Expression, Parser}};
use std::fmt;

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expr: Expression,
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Result<ExpressionStatement, ParseError> {
    let expr = if parser.next(0).is_some() {
        PrattParser::parse_expr(parser, Precedence::Lowest, None).unwrap()
    } else {
        return Err(ParseError("[ExpressionStatement] insufficient tokens".into()));
    };

    parser.pos += 1;

    if let Some(token) = parser.next(0) {
        if Token::Semicolon == token {
            parser.pos += 1;
        }
    }

    Ok(ExpressionStatement {
        expr,
    })
}


impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}