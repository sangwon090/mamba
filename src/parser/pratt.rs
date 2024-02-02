use std::rc::Rc;

use crate::parser::Parser;
use crate::parser::Expression;
use crate::parser::ParseError;
use crate::lexer::{Token, Identifier};
use crate::parser::ast::InfixExpression;
use crate::parser::ast::{PrefixExpression, Operator};

use super::ast::FnCallExpression;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Precedence {
    Lowest,
    // Assignment,
    // Lambda,
    // IfElse,
    // BooleanOr,
    // BooleanAnd,
    // BooleanNot,
    EqualNotEqual,
    LessGreater,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Shift,
    PlusMinus,
    MulDivMod,
    Unary,
    // Exp,
    // Await,
    Paren,
    FnCall,
}

pub struct PrattParser;

impl PrattParser {
    fn get_operator(token: &Token, is_prefix: bool) -> Option<Operator> {
        if is_prefix {
            match token {
                Token::Plus => Some(Operator::UnaryPlus),
                Token::Minus => Some(Operator::UnaryMinus),
                Token::Tilde => Some(Operator::BitwiseNot),
                _ => None,
            }
        } else {
            match token {
                Token::Plus => Some(Operator::Plus),
                Token::Minus => Some(Operator::Minus),
                Token::Slash => Some(Operator::Divide),
                Token::Star => Some(Operator::Multiply),
                Token::EqualEqual => Some(Operator::Equal),
                Token::NotEqual => Some(Operator::NotEqual),
                Token::Less => Some(Operator::Less),
                Token::LessEqual => Some(Operator::LessEqual),
                Token::Greater => Some(Operator::Greater),
                Token::GreaterEqual => Some(Operator::GreaterEqual),
                Token::Percent => Some(Operator::Modulo),
                Token::Ampersand => Some(Operator::BitwiseAnd),
                Token::Circumflex => Some(Operator::BitwiseXor),
                Token::VBar => Some(Operator::BitwiseOr),
                Token::LParen => Some(Operator::FnCall),
                _ => None,
            }
        }
    }
    
    fn get_precedence(operator: &Operator) -> Result<Precedence, ParseError> {
        Ok(match operator {
            Operator::Plus | Operator::Minus => Precedence::PlusMinus,
            Operator::Multiply | Operator::Divide | Operator::Modulo => Precedence::MulDivMod,
            Operator::BitwiseAnd => Precedence::BitwiseAnd,
            Operator::BitwiseOr => Precedence::BitwiseOr,
            Operator::BitwiseXor => Precedence::BitwiseXor,
            Operator::LeftShift | Operator::RightShift => Precedence::Shift,
            Operator::Less | Operator::LessEqual | Operator::Greater |
            Operator::GreaterEqual => Precedence::LessGreater,
            Operator::Equal | Operator::NotEqual => Precedence::EqualNotEqual,
            Operator::FnCall => Precedence::FnCall,
            _ => return Err(ParseError(format!("[PrattParser::get_precedence] unknown operator {:?}", operator)))
        })
    }

    pub fn parse_expression(parser: &mut Parser, precedence: Precedence) -> Result<Expression, ParseError> {
        let token = parser.next(0).unwrap();

        let prefix: Option<Expression> = match token.clone() {
            Token::Identifier(identifier) => Some(Expression::Identifier(identifier)),
            Token::Literal(literal) => Some(Expression::Literal(literal)),
            Token::LParen => {
                parser.pos += 1;

                let expression = PrattParser::parse_expression(parser, Precedence::Lowest);

                if let Some(token) = parser.next(1) {
                    if token == Token::RParen {
                        parser.pos += 1;
                        Some(expression.unwrap())
                    } else {
                        return Err(ParseError("[PrattParser::parse_expression] RParen not found".into()));
                    }
                } else {
                    return Err(ParseError("[PrattParser::parse_expression] insufficient tokens".into()));
                }
            }
            Token::Plus | Token::Minus | Token::Tilde => Some(PrattParser::parse_nud(parser).unwrap()),
            _ => None
        };

        let mut expression = prefix.unwrap();

        loop {
            let token = if let Some(token) = parser.next(1) {
                if let Token::EOF = token {
                    return Ok(expression);
                } else {
                    token
                }
            } else {
                return Ok(expression);
            };

            let operator = if let Some(token) = parser.next(1) {
                if let Some(operator) = PrattParser::get_operator(&token, false) {
                    operator
                } else {
                    return Ok(expression);
                }
            } else {
                return Err(ParseError("[PrattParser::parse_expression] insufficient tokens".into()));
            };

            if precedence >= PrattParser::get_precedence(&operator).unwrap() {
                return Ok(expression);
            }

            parser.pos += 1;

            match token.clone() {
                Token::Plus | Token::Minus | Token::Slash |
                Token::Star | Token::EqualEqual | Token::NotEqual |
                Token::Less | Token::LessEqual | Token::Greater | 
                Token::GreaterEqual | Token::Percent | Token::Ampersand |
                Token::Circumflex | Token::VBar | Token::LParen => expression = PrattParser::parse_led(parser, expression).unwrap(),
                Token::EOF => {
                    return Ok(expression);
                }
                _ => {
                    return Ok(expression)
                },
            }

        };
    }

    pub fn parse_nud(parser: &mut Parser) -> Result<Expression, ParseError> {
        let operator = if let Some(token) = parser.next(0) {
            PrattParser::get_operator(&token, true).unwrap()
        } else {
            return Err(ParseError("[PrattParser::parse_nud] insufficient tokens".into()));
        };

        parser.pos += 1;

        let right = PrattParser::parse_expression(parser, Precedence::Unary).unwrap();

        let prefix_expression = PrefixExpression {
            operator,
            right: Box::new(right),
        };

        Ok(Expression::Prefix(prefix_expression))
    }

    pub fn parse_led(parser: &mut Parser, left: Expression) -> Result<Expression, ParseError> {
        let operator = if let Some(token) = parser.next(0) {
            PrattParser::get_operator(&token, false).unwrap()
        } else {
            return Err(ParseError("[PrattParser::parse_led] insufficient tokens".into()));
        };

        let precedence = PrattParser::get_precedence(&operator).unwrap();
        
        if let Operator::FnCall = operator {
            let fncall_expression = PrattParser::parse_fncall(parser, left).unwrap();
            return Ok(Expression::FnCall(fncall_expression));
        }

        parser.pos += 1;


        let right = PrattParser::parse_expression(parser, precedence).unwrap();

        let infix_expression = InfixExpression {
            operator,
            left: Rc::new(left),
            right: Rc::new(right),
        };

        Ok(Expression::Infix(infix_expression))
    }

    pub fn parse_fncall(parser: &mut Parser, left: Expression) -> Result<FnCallExpression, ParseError> {
        let identifier = Identifier(format!("{:?}", left));
        let mut arguments: Vec<Expression> = Vec::new();

        parser.pos += 1;

        if let Some(token) = parser.next(0) {
            if let Token::RParen = token {
                parser.pos += 1;
                return Ok(FnCallExpression {
                    identifier,
                    arguments,
                })
            }
        } else {
            return Err(ParseError("[FnCallExpression] insufficient tokens".into()));
        }

        loop {
            let argument = PrattParser::parse_expression(parser, Precedence::Lowest).unwrap();
            arguments.push(argument);


            parser.pos += 1;

            if let Some(token) = parser.next(0) {
                match token {
                    Token::Comma => {
                        parser.pos += 1;
                        continue;
                    },
                    Token::RParen => {
                        break;
                    },
                    _ => {
                        return Err(ParseError(format!("[FnCallExpression] expected `,` or `)`, found {token:?}")));
                    },
                }
            } else {
                return Err(ParseError("[FnCallExpression] insufficient tokens".into()));
            }
        }

        Ok(FnCallExpression {
            identifier,
            arguments,
        })
    }
}