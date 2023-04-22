use crate::parser::{Parser, PrattParser, Token, Keyword};
use crate::parser::ast::{Statement, Expression, AstNodeType};
use crate::error::ParseError;
use core::any::Any;

use super::pratt::Precedence;

pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub then: Box<dyn Statement>,
    pub r#else: Option<Box<dyn Statement>>,
}

impl Statement for IfStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let condition = PrattParser::parse_expression(parser, Precedence::Lowest).unwrap();
        parser.pos += 1;

        if let Some(token) = parser.next(0) {
            if let Token::Colon = token {
                parser.pos += 1;
            } else {
                return Err(ParseError(format!("[IfStatement] expected `:`, found {token:?}")));
            }
        } else {
            return Err(ParseError("[IfStatement] insufficient tokens".into()));
        }

        if let Some(token) = parser.next(0) {
            if let Token::Indent = token {
                parser.pos += 1;
            } else {
                return Err(ParseError(format!("[IfStatement] expected indent, found {token:?}")));
            }
        } else {
            return Err(ParseError("[IfStatement] insufficient tokens".into()));
        }

        let then = parser.parse_statement().unwrap().unwrap();

        if let Some(token) = parser.next(0) {
            if let Token::Dedent = token {
                parser.pos += 1;
            } else {
                return Err(ParseError(format!("[IfStatement] expected dedent, found {token:?}")));
            }
        } else {
            return Err(ParseError("[IfStatement] insufficient tokens".into()));
        }

        if let Some(token) = parser.next(0) {
            if let Token::Keyword(keyword) = token {
                if keyword == Keyword::Else {
                    parser.pos += 1;

                    if let Some(token) = parser.next(0) {
                        if let Token::Colon = token {
                            parser.pos += 1;
                        } else {
                            return Err(ParseError(format!("[IfStatement] expected `:`, found {token:?}")));
                        }
                    } else {
                        return Err(ParseError("[IfStatement] insufficient tokens".into()));
                    }

                    if let Some(token) = parser.next(0) {
                        if let Token::Indent = token {
                            parser.pos += 1;
                        } else {
                            return Err(ParseError(format!("[IfStatement] expected indent, found {token:?}")));
                        }
                    } else {
                        return Err(ParseError("[IfStatement] insufficient tokens".into()));
                    }
                    
            
                    let r#else = parser.parse_statement().unwrap().unwrap();
            
                    if let Some(token) = parser.next(0) {
                        if let Token::Dedent = token {
                            parser.pos += 1;
                        } else {
                            return Err(ParseError(format!("[IfStatement] expected dedent, found {token:?}")));
                        }
                    } else {
                        return Err(ParseError("[IfStatement] insufficient tokens".into()));
                    }
                    
                    Ok(IfStatement {
                        condition,
                        then,
                        r#else: Some(r#else),
                    })
                } else {
                    Ok(IfStatement {
                        condition,
                        then,
                        r#else: None,
                    })
                }
            } else {
                Ok(IfStatement {
                    condition,
                    then,
                    r#else: None,
                })
            }
        } else {
            Ok(IfStatement {
                condition,
                then,
                r#else: None,
            }) 
        }
    }

    fn to_string(&self) -> String {
        if let Some(r#else) = &self.r#else {
            format!("{{ type: if, condition: {}, then: {}, else: {} }}", self.condition.to_string(), self.then.to_string(), r#else.to_string())
        } else {
            format!("{{ type: if, condition: {}, then: {} }}", self.condition.to_string(), self.then.to_string())
        }
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::IfStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}