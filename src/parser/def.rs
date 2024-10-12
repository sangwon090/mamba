use crate::parser::{Parser, Token};
use crate::parser::ast::{Statement, AstNodeType};
use crate::error::ParseError;
use crate::types::DataType;
use core::any::Any;

use super::ast::Identifier;

pub struct DefStatement {
    pub name: Identifier,
    pub params: Vec<(Identifier, DataType)>,
    pub r#type: DataType,
    pub stmts: Vec<Box<dyn Statement>>,
}

impl Statement for DefStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut params: Vec<(Identifier, DataType)> = Vec::new();
        let mut stmts: Vec<Box<dyn Statement>> = Vec::new();

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
    
                if let Token::Keyword(keyword) = token {
                    keyword.into()
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

            if let Token::Keyword(keyword) = token {
                keyword.into()
            } else {
                return Err(ParseError(format!("[DefStatement] expected keyword, found {token:?}")));
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

    fn to_string(&self) -> String {
        let mut result = format!("{{ type: fnDef, name: {}, returnType: {}, args: {:?}, stmts: {{ ", &self.name, &self.r#type.to_mnemonic(), self.params);
        for stmt in &self.stmts {
            result.push_str(&stmt.to_string())
        }

        result.push_str(" }");
        result
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::DefStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}