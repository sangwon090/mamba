use crate::parser::{Parser, PrattParser};
use crate::error::ParseError;
use crate::parser::pratt::Precedence;
use crate::lexer::Token;
use core::any::Any;
use std::fmt;

#[derive(Default)]
pub struct AST {
    pub stmts: Vec<Box<dyn Statement>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum AstNodeType {
    Identifier,
    Literal,
    PrefixExpression,
    InfixExpression,
    ExpressionStatement,
    FnCallExpression,
    DefStatement,
    IfStatement,
    LetStatement,
    ReturnStatement,
}

pub trait Statement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> where Self: Sized;
    fn to_string(&self) -> String;
    fn get_type(&self) -> AstNodeType;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub enum Expression {
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    FnCall(FnCallExpression),
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub operator: Operator,
    pub right: Box<Expression>,
}


#[derive(Debug)]
pub struct InfixExpression {
    pub operator: Operator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}


#[derive(Debug)]
pub struct FnCallExpression {
    pub ident: Identifier,
    pub args: Vec<Expression>,
}

pub type Identifier = String;

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Prefix(expr) => write!(f, "{}", expr),
            Self::Infix(expr) => write!(f, "{}", expr),
            Self::FnCall(expr) => write!(f, "{}", expr),
            Self::Identifier(ident) => write!(f, "{}", ident),
            Self::Literal(literal) => write!(f, "{:?}", literal),
        }
    }
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ operator: {:?}, right: {} }}", self.operator, self.right)
    }
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ operator: {:?}, left: {}, right: {} }}", self.operator, self.left, self.right)
    }
}

impl fmt::Display for FnCallExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ type: fnCall, name: {}, args: {{ {} }} }}", self.ident, self.args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    Integer(i64),
    String(String),
}

pub struct ExpressionStatement {
    expr: Expression,
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

impl Operator {
    pub fn to_mnemonic(self) -> &'static str {
        match self {
            Operator::Equal => "eq",
            Operator::NotEqual => "ne",
            Operator::Greater => "sgt",
            Operator::GreaterEqual => "sge",
            Operator::Less => "slt",
            Operator::LessEqual => "sle",
            _ => "",
        }
    }
}

impl Statement for ExpressionStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let expr = if parser.next(0).is_some() {
            PrattParser::parse_expr(parser, Precedence::Lowest).unwrap()
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
            expr,
        })
    }

    fn to_string(&self) -> String {
        self.expr.to_string()
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::ExpressionStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
} 

impl AST {
    pub fn new() -> AST {
        AST {
            stmts: Vec::new(),
        }
    }
}