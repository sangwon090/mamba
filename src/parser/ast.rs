use crate::parser::{Parser, PrattParser};
use crate::error::ParseError;
use crate::parser::pratt::Precedence;
use crate::lexer::{Identifier, Literal, Token};
use std::fmt;
use std::rc::Rc;

use super::{DefStatement, IfStatement, LetStatement, ReturnStatement};

pub struct AbstractSyntaxTree {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum AstNode {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    Def(DefStatement),
    If(IfStatement),
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug)]
pub enum Expression {
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    FnCall(FnCallExpression),
    Identifier(Identifier),
    Literal(Literal),
}

pub trait Parsable {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> where Self: Sized;
}

pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    UnaryPlus,
    UnaryMinus,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftShift,
    RightShift,
    FnCall,
}

pub struct PrefixExpression {
    pub operator: Operator,
    pub right: Box<Expression>,
}


impl fmt::Debug for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ operator: {:?}, right: {:?} }}", self.operator, *self.right)
    }
}

pub struct InfixExpression {
    pub operator: Operator,
    pub left: Rc<Expression>,
    pub right: Rc<Expression>,
}

impl fmt::Debug for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ operator: {:?}, left: {:?}, right: {:?} }}", self.operator, *self.left, *self.right)
    }
}

impl Parsable for ExpressionStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let expression = if let Some(token) = parser.next(0) {
            PrattParser::parse_expression(parser, Precedence::Lowest).unwrap()
        } else {
            return Err(ParseError("[ExpressionStatement] insufficient tokens".into()));
        };

        parser.pos += 1;

        if let Some(token) = parser.next(0) {
            if Token::Semicolon == token {
                parser.pos += 1;
            }
        }

        Ok(ExpressionStatement {
            expression,
        })
    }
} 

impl fmt::Debug for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{:?}", self.expression)
    }
}

pub struct FnCallExpression {
    pub identifier: Identifier,
    pub arguments: Vec<Expression>,
}

impl fmt::Debug for FnCallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = format!("{{ name: {}, args: {{ ", self.identifier.0);

        for argument in &self.arguments {
            result.push_str(&format!("{:?}", argument));
            result.push_str(", ");
        }

        result.push_str("} }");

        write!(f, "{}", result)
    }
}

impl AbstractSyntaxTree {
    pub fn new() -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            statements: Vec::new(),
        }
    }
}