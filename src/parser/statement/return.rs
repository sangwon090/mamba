use crate::parser::{Parser, PrattParser, Token};
use crate::parser::Expression;
use crate::error::ParseError;
use crate::parser::pratt::Precedence;
use std::fmt;

#[derive(Debug)]
pub struct ReturnStatement {
    pub expr: Expression,
}

pub fn parse_return(parser: &mut Parser) -> Result<ReturnStatement, ParseError> {
    let expr = PrattParser::parse_expr(parser, Precedence::Lowest, None).unwrap();

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
        expr,
    })
} 


impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ type: return, expr: {:?} }}", self.expr)
    }
}