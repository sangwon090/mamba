use crate::parser::{Expression, Parser, PrattParser, Token};
use crate::error::ParseError;

use crate::parser::pratt::Precedence;
use super::Statement;
use std::fmt;

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub blocks: Vec<Statement>,
}

pub fn parse_while(parser: &mut Parser) -> Result<WhileStatement, ParseError> {
    let condition = PrattParser::parse_expr(parser, Precedence::Lowest, None).unwrap();
    parser.pos += 1;

    if let Some(token) = parser.next(0) {
        if let Token::Colon = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[WhileStatement] expected `:`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[WhileStatement] insufficient tokens".into()));
    }

    if let Some(token) = parser.next(0) {
        if let Token::Indent = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[WhileStatement] expected indent, found {token:?}")));
        }
    } else {
        return Err(ParseError("[WhileStatement] insufficient tokens".into()));
    }

    let mut blocks: Vec<Statement> = Vec::new();

    loop {
        blocks.push(parser.parse_stmt().unwrap().unwrap());

        if let Some(token) = parser.next(0) {
            if let Token::Dedent = token {
                parser.pos += 1;
                break
            } else {
                return Err(ParseError(format!("[WhileStatement] expected dedent, found {token:?}")));
            }
        } else {
            return Err(ParseError("[WhileStatement] insufficient tokens".into()));
        }
    }


    Ok(WhileStatement {
        condition,
        blocks,
    })
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ type: while, condition: {}, blocks: {:?} }}", self.condition, self.blocks)
    }
}