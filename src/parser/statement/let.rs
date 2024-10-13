use crate::lexer::Keyword;
use crate::parser::{Expression, Identifier, Parser, PrattParser, Token};
use crate::error::ParseError;
use crate::types::DataType;
use crate::parser::pratt::Precedence;
use std::fmt;

pub struct LetStatement {
    pub ident: Identifier,
    pub r#type: DataType,
    pub expr: Expression,
}

pub fn parse_let(parser: &mut Parser) -> Result<LetStatement, ParseError> {        
    let ident = if let Some(token) = parser.next(0) {
        parser.pos += 1;

        if let Token::Identifier(ident) = token {
            ident
        } else {
            return Err(ParseError(format!("[LetStatement] expected identifier, found {token:?}")));
        }
    } else {
        return Err(ParseError("[LetStatement] insufficient tokens".into()));
    };

    if let Some(token) = parser.next(0) {
        if let Token::Colon = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[LetStatement] expected `:`, found {token:?}")));
        }
    }

    let r#type = if let Some(token) = parser.next(0) {
        parser.pos += 1;

        if let Token::Keyword(Keyword::DataType(dtype)) = token {
            dtype
        } else {
            return Err(ParseError(format!("[LetStatement] expected keyword, found {token:?}")));
        }
    } else {
        return Err(ParseError("[LetStatement] insufficient tokens".into()));
    };

    if let Some(token) = parser.next(0) {
        if let Token::Equal = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[LetStatement] expected `=`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[LetStatement] insufficient tokens".into()));
    }

    let expr = PrattParser::parse_expr(parser, Precedence::Lowest, Some(r#type)).unwrap();

    parser.pos += 1;

    if let Some(token) = parser.next(0) {
        if let Token::Semicolon = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[LetStatement] Expected `;`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[LetStatement] insufficient tokens".into()));
    }

    Ok(LetStatement {
        ident,
        r#type,
        expr,
    })

}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ type: let, name: {}, dataType: {}, expr: {} }}", &self.ident, &self.r#type.to_mnemonic(), self.expr)
    }
}