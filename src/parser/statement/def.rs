use std::fmt;
use crate::lexer::Keyword;
use crate::parser::{Parser, Token, Identifier, Statement};
use crate::error::ParseError;
use crate::types::DataType;

#[derive(Debug)]
pub struct DefStatement {
    pub name: Identifier,
    pub params: Vec<(Identifier, DataType)>,
    pub r#type: DataType,
    pub stmts: Vec<Statement>,
}

pub fn parse_def(parser: &mut Parser) -> Result<DefStatement, ParseError> {
    let mut params: Vec<(Identifier, DataType)> = Vec::new();
    let mut stmts: Vec<Statement> = Vec::new();

    let name = if let Some(token) = parser.next(0) {
        if let Token::Identifier(ident) = token {
            parser.pos += 1;
            ident
        } else {
            return Err(ParseError(format!("[DefStatement] expected identifier, found {token:?}")));
        }
    } else {
        return Err(ParseError("[DefStatement] insufficient tokens".into()));
    };

    if let Some(token) = parser.next(0) {
        if let Token::LParen = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[DefStatement] expected `(`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[DefStatement] insufficient tokens".into()));
    }

    loop {
        let ident = if let Some(token) = parser.next(0) {
            if let Token::Identifier(ident) = token {
                parser.pos += 1;
                ident
            } else if let Token::RParen = token {
                parser.pos += 1;
                break;
            } else {
                return Err(ParseError(format!("[DefStatement] expected identifier, found {token:?}")));
            }
        } else {
            return Err(ParseError("[DefStatement] insufficient tokens".into()));
        };

        if let Some(token) = parser.next(0) {
            if let Token::Colon = token {
                parser.pos += 1;
            } else {
                return Err(ParseError(format!("[DefStatement] expected `:`, found {token:?}")));
            }
        } else {
            return Err(ParseError("[DefStatement] insufficient tokens".into()));
        }

        let r#type = if let Some(token) = parser.next(0) {
            parser.pos += 1;

            if let Token::Keyword(Keyword::DataType(dtype)) = token {
                dtype
            } else {
                return Err(ParseError(format!("[DefStatement] expected keyword, found {token:?}")));
            }
        } else {
            return Err(ParseError("[DefStatement] insufficient tokens".into()));
        };

        params.push((ident, r#type));

        if let Some(token) = parser.next(0) {
            match token {
                Token::RParen => {
                    parser.pos += 1;
                    break;
                },
                Token::Comma => {
                    parser.pos += 1;
                    continue;
                },
                _ => {
                    return Err(ParseError(format!("[DefStatement] expected `,` or `)`, found {token:?}")));
                },
            }
        } else {
            return Err(ParseError("[DefStatement] insufficient tokens".into()));
        }
    }

    if let Some(token) = parser.next(0) {
        if let Token::RArrow = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[DefStatement] expected `->`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[DefStatement] insufficient tokens".into()));
    }

    let r#type = if let Some(token) = parser.next(0) {
        parser.pos += 1;

        if let Token::Keyword(Keyword::DataType(dtype)) = token {
            dtype
        } else {
            return Err(ParseError(format!("[DefStatement] expected keyword(data type), found {token:?}")));
        }
    } else {
        return Err(ParseError("[DefStatement] insufficient tokens".into()));
    };

    if let Some(token) = parser.next(0) {
        if let Token::Colon = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[DefStatement] expected `:`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[DefStatement] insufficient tokens".into()));
    }

    if let Some(token) = parser.next(0) {
        if let Token::Indent = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[DefStatement] expected indent, found {token:?}")));
        }
    } else {
        return Err(ParseError("[DefStatement] insufficient tokens".into()));
    }

    loop {
        if let Some(token) = parser.next(0) {
            if let Token::Dedent = token {
                parser.pos += 1;
                break;
            }
        } else {
            return Err(ParseError("[DefStatement] insufficient tokens".into()));
        }

        let stmt = parser.parse_stmt().unwrap();
        if let Some(stmt) = stmt {
            stmts.push(stmt);
        } else {
            break;
        }
    }

    Ok(DefStatement {
        name,
        params,
        r#type,
        stmts,
    })
}

impl fmt::Display for DefStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ type: fnDef, name: {}, returnType: {}, args: {:?}, stmts: {{ {} }}", &self.name, &self.r#type.to_mnemonic(), self.params, self.stmts.iter().map(|stmt| stmt.to_string()).collect::<Vec<String>>().join(", "))
    }
}