use crate::parser::{Parser, Expression, ParseError, Operator};
use crate::lexer::{Literal, Token};
use crate::types::{DataType, SignedInteger, UnsignedInteger};

use super::expression::*;

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

    // TODO: Support various types
    pub fn parse_expr(parser: &mut Parser, precedence: Precedence, expected_dtype: Option<DataType>) -> Result<Expression, ParseError> {
        let token = parser.next(0).unwrap();

        // TODO: refactor
        // TODO: support type casting
        let prefix: Option<Expression> = match token.clone() {
            Token::Identifier(ident) => Some(Expression::Identifier(ident)),
            Token::Literal(literal) => {
                match literal {
                    Literal::SignedInteger((n, _)) => {
                        let dtype = if let DataType::SignedInteger(signed) = expected_dtype.unwrap_or(DataType::SignedInteger(SignedInteger::i32)) {
                            signed
                        } else {
                            panic!();
                        };

                        let literal = Literal::SignedInteger((n, dtype));
                        Some(Expression::Literal((literal, DataType::SignedInteger(dtype))))
                    },
                    Literal::UnsignedInteger((n, _)) => {
                        let dtype = if let DataType::UnsignedInteger(unsigned) = expected_dtype.unwrap_or(DataType::UnsignedInteger(UnsignedInteger::u32)) {
                            unsigned
                        } else {
                            panic!();
                        };

                        let literal = Literal::UnsignedInteger((n, dtype));
                        Some(Expression::Literal((literal, DataType::UnsignedInteger(dtype))))
                    },
                    Literal::String(s) => {
                        Some(Expression::Literal((Literal::String(s), DataType::str)))
                    },
                    // TODO: Unsigned Integer with 'u' suffix
                }
            },
            Token::LParen => {
                parser.pos += 1;

                let expr = PrattParser::parse_expr(parser, Precedence::Lowest, None);

                if let Some(token) = parser.next(1) {
                    if token == Token::RParen {
                        parser.pos += 1;
                        Some(expr.unwrap())
                    } else {
                        return Err(ParseError("[PrattParser::parse_expr] RParen not found".into()));
                    }
                } else {
                    return Err(ParseError("[PrattParser::parse_expr] insufficient tokens".into()));
                }
            }
            Token::Plus | Token::Minus | Token::Tilde => Some(PrattParser::parse_nud(parser).unwrap()),
            _ => { println!("unexpected token {token:?} found"); None }
        };

        let mut expr = prefix.unwrap();

        loop {
            let token = if let Some(token) = parser.next(1) {
                if let Token::EOF = token {
                    return Ok(expr);
                } else {
                    token
                }
            } else {
                return Ok(expr);
            };

            let operator = if let Some(token) = parser.next(1) {
                if let Some(operator) = PrattParser::get_operator(&token, false) {
                    operator
                } else {
                    return Ok(expr);
                }
            } else {
                return Err(ParseError("[PrattParser::parse_expr] insufficient tokens".into()));
            };

            if precedence >= PrattParser::get_precedence(&operator).unwrap() {
                return Ok(expr);
            }

            parser.pos += 1;

            match token.clone() {
                Token::Plus | Token::Minus | Token::Slash |
                Token::Star | Token::EqualEqual | Token::NotEqual |
                Token::Less | Token::LessEqual | Token::Greater | 
                Token::GreaterEqual | Token::Percent | Token::Ampersand |
                Token::Circumflex | Token::VBar | Token::LParen => expr = PrattParser::parse_led(parser, expr).unwrap(),
                Token::EOF => {
                    return Ok(expr);
                }
                _ => {
                    return Ok(expr);
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

        let right = PrattParser::parse_expr(parser, Precedence::Unary, None).unwrap();

        let unary_expr = UnaryExpression {
            operator,
            right: Box::new(right),
        };
        
        Ok(Expression::Unary(unary_expr))
    }

    pub fn parse_led(parser: &mut Parser, left: Expression) -> Result<Expression, ParseError> {
        let operator = if let Some(token) = parser.next(0) {
            PrattParser::get_operator(&token, false).unwrap()
        } else {
            return Err(ParseError("[PrattParser::parse_led] insufficient tokens".into()));
        };

        let precedence = PrattParser::get_precedence(&operator).unwrap();
        
        if let Operator::FnCall = operator {
            let fncall_expr = PrattParser::parse_fncall(parser, left).unwrap();
            return Ok(Expression::FnCall(fncall_expr));
        }

        parser.pos += 1;


        let right = PrattParser::parse_expr(parser, precedence, None).unwrap();

        let infix_expr = InfixExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        };

        Ok(Expression::Infix(infix_expr))
    }

    pub fn parse_fncall(parser: &mut Parser, left: Expression) -> Result<FnCallExpression, ParseError> {
        let ident = if let Expression::Identifier(ident) = left {
            ident
        } else {
            return Err(ParseError("[FnCallExpression] expected identifier, found ?".into()));
        };
        let mut args: Vec<Expression> = Vec::new();

        parser.pos += 1;

        if let Some(token) = parser.next(0) {
            if let Token::RParen = token {
                return Ok(FnCallExpression {
                    ident,
                    args,
                })
            }
        } else {
            return Err(ParseError("[FnCallExpression] insufficient tokens".into()));
        }

        loop {
            let arg = PrattParser::parse_expr(parser, Precedence::Lowest, None).unwrap();
            args.push(arg);

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
            ident,
            args,
        })
    }
}