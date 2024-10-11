use crate::lexer::Identifier;
use crate::parser::{Parser, PrattParser, Token, Keyword};
use crate::parser::ast::{Statement, Expression, AstNodeType};
use crate::error::ParseError;
use crate::types::DataType;
use super::pratt::Precedence;
use core::any::Any;

pub struct LetStatement {
    pub ident: Identifier,
    pub r#type: DataType,
    pub expr: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {        
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

            if let Token::Keyword(keyword) = token {
                keyword.into()
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

        let expr = PrattParser::parse_expr(parser, Precedence::Lowest).unwrap();

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

    fn to_string(&self) -> String {   
        format!("{{ type: let, name: {}, dataType: {}, expr: {} }}", &self.ident.0, &self.r#type.to_mnemonic(), self.expr.to_string())
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::LetStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}