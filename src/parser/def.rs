use crate::parser::{Parser, Token, Keyword};
use crate::parser::ast::Statement;
use crate::error::ParseError;
use crate::types::DataType;
use crate::lexer::Identifier;
use std::fmt;

use super::ast::Parsable;

pub struct DefStatement {
    pub name: Identifier,
    pub parameters: Vec<(Identifier, DataType)>,
    pub r#type: DataType,
    pub statements: Vec<Statement>,
}

impl Parsable for DefStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut parameters: Vec<(Identifier, DataType)> = Vec::new();
        let mut statements: Vec<Statement> = Vec::new();

        let name = if let Some(token) = parser.next(0) {
            if let Token::Identifier(identifier) = token {
                parser.pos += 1;
                identifier
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
            let identifier = if let Some(token) = parser.next(0) {
                if let Token::Identifier(identifier) = token {
                    parser.pos += 1;
                    identifier
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
                    match keyword {
                        Keyword::Int => DataType::Int,
                        Keyword::Str => DataType::Str,
                        Keyword::Void => DataType::Void,
                        _ => {
                            return Err(ParseError(format!("[DefStatement] expected type, found {keyword:?}")))
                        },
                    }
                } else {
                    return Err(ParseError(format!("[DefStatement] expected keyword, found {token:?}")));
                }
            } else {
                return Err(ParseError("[DefStatement] insufficient tokens".into()));
            };

            parameters.push((identifier, r#type));

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
                match keyword {
                    Keyword::Int => DataType::Int,
                    Keyword::Str => DataType::Str,
                    _ => {
                        return Err(ParseError(format!("[DefStatement] expected type, found {keyword:?}")))
                    },
                }
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

            let statement = parser.parse_statement().unwrap();
            if let Some(statement) = statement {
                statements.push(statement);
            } else {
                break;
            }
        }

        Ok(DefStatement {
            name,
            parameters,
            r#type,
            statements,
        })
    }
}

impl fmt::Debug for DefStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r#type = match self.r#type {
            DataType::Int => "int",
            DataType::Str => "str",
            DataType::Void => "void",
        };

        let mut result = format!("{{ name: {}, returnType: {}, args: {:?}, statements: {{ ", self.name.0, r#type, self.parameters);
        for statement in &self.statements {
            result.push_str(&format!("{:?}", statement))
        }

        result.push_str(" }");

        write!(f, "{}", result)
    }
}