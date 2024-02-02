use crate::lexer::Identifier;
use crate::parser::{Parser, PrattParser, Token, Keyword};
use crate::parser::ast::Expression;
use crate::error::ParseError;
use crate::types::DataType;
use super::ast::Parsable;
use super::pratt::Precedence;
use std::fmt;

pub struct LetStatement {
    pub identifier: Identifier,
    pub r#type: DataType,
    pub expression: Expression,
}

impl Parsable for LetStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {        
        let identifier = if let Some(token) = parser.next(0) {
            parser.pos += 1;

            if let Token::Identifier(identifier) = token {
                identifier
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

            if let Token::Keyword(keyword) = token {
                match keyword {
                    Keyword::Int => DataType::Int,
                    Keyword::Str => DataType::Str,
                    Keyword::Void => DataType::Void,
                    _ => {
                        return Err(ParseError(format!("[LetStatement] expected type, found {keyword:?}")))
                    },
                }
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

        let expression = PrattParser::parse_expression(parser, Precedence::Lowest).unwrap();

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
            identifier,
            r#type,
            expression,
        })

    }
}

impl fmt::Debug for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r#type = match self.r#type {
            DataType::Int => "int",
            DataType::Str => "str",
            DataType::Void => "void",
        };
        
        write!(f, "{{ name: {}, dataType: {}, expression: {:?} }}", self.identifier.0, r#type, self.expression)
    }
}