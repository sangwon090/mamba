use std::fmt;
use crate::lexer::Keyword;
use crate::parser::{Parser, Token, Identifier, Statement};
use crate::error::ParseError;
use crate::types::DataType;

pub struct ExternStatement {
    pub name: Identifier,
    pub params: Vec<(Identifier, DataType)>,
    pub r#type: DataType,
}

pub fn parse_extern(parser: &mut Parser) -> Result<ExternStatement, ParseError> {
    let mut params: Vec<(Identifier, DataType)> = Vec::new();

    let name = if let Some(token) = parser.next(0) {
        if let Token::Identifier(ident) = token {
            parser.pos += 1;
            ident
        } else {
            return Err(ParseError(format!("[ExternStatement] expected identifier, found {token:?}")));
        }
    } else {
        return Err(ParseError("[ExternStatement] insufficient tokens".into()));
    };

    if let Some(token) = parser.next(0) {
        if let Token::LParen = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[ExternStatement] expected `(`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[ExternStatement] insufficient tokens".into()));
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
                return Err(ParseError(format!("[ExternStatement] expected identifier, found {token:?}")));
            }
        } else {
            return Err(ParseError("[ExternStatement] insufficient tokens".into()));
        };

        if let Some(token) = parser.next(0) {
            if let Token::Colon = token {
                parser.pos += 1;
            } else {
                return Err(ParseError(format!("[ExternStatement] expected `:`, found {token:?}")));
            }
        } else {
            return Err(ParseError("[ExternStatement] insufficient tokens".into()));
        }

        let r#type = if let Some(token) = parser.next(0) {
            parser.pos += 1;

            if let Token::Keyword(Keyword::DataType(dtype)) = token {
                dtype
            } else {
                return Err(ParseError(format!("[ExternStatement] expected keyword, found {token:?}")));
            }
        } else {
            return Err(ParseError("[ExternStatement] insufficient tokens".into()));
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
                    return Err(ParseError(format!("[ExternStatement] expected `,` or `)`, found {token:?}")));
                },
            }
        } else {
            return Err(ParseError("[ExternStatement] insufficient tokens".into()));
        }
    }

    if let Some(token) = parser.next(0) {
        if let Token::RArrow = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[ExternStatement] expected `->`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[ExternStatement] insufficient tokens".into()));
    }

    let r#type = if let Some(token) = parser.next(0) {
        parser.pos += 1;

        if let Token::Keyword(Keyword::DataType(dtype)) = token {
            dtype
        } else {
            return Err(ParseError(format!("[ExternStatement] expected keyword, found {token:?}")));
        }
    } else {
        return Err(ParseError("[ExternStatement] insufficient tokens".into()));
    };

    if let Some(token) = parser.next(0) {
        if let Token::Semicolon = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[ReturnStatement] Expected `;`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[ReturnStatement] insufficient tokens".into()));
    }

    Ok(ExternStatement {
        name,
        params,
        r#type,
    })
}

impl fmt::Display for ExternStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ type: extern name: {}, returnType: {}, args: {:?} }}", &self.name, &self.r#type.to_mnemonic(), self.params)
    }
}