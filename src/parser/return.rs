use crate::parser::{Parser, PrattParser, Token};
use crate::parser::ast::{Statement, Expression, AstNodeType};
use crate::error::ParseError;
use super::pratt::Precedence;
use core::any::Any;

pub struct ReturnStatement {
    pub expression: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let expression = PrattParser::parse_expression(parser, Precedence::Lowest).unwrap();

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
            expression,
        })
    } 

    fn to_string(&self) -> String {
        format!("{{ type: return, expression: {} }}", self.expression.to_string())
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::ReturnStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}