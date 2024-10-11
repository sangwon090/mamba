use crate::parser::{Parser, PrattParser};
use crate::error::ParseError;
use crate::parser::pratt::Precedence;
use crate::lexer::{Token, Identifier};
use core::any::Any;

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

pub trait Expression {
    //fn parse(parser: &mut Parser) -> Result<Self, ParseError> where Self: Sized;
    fn to_string(&self) -> String;
    fn get_type(&self) -> AstNodeType;
    fn as_any(&self) -> &dyn Any;
}

pub struct ExpressionStatement {
    expr: Box<dyn Expression>,
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

pub struct PrefixExpression {
    pub operator: Operator,
    pub right: Box<dyn Expression>
}

impl Expression for PrefixExpression {
    fn to_string(&self) -> String {
        format!("{{ operator: {:?}, right: {} }}", self.operator, self.right.to_string())
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::PrefixExpression
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InfixExpression {
    pub operator: Operator,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
    fn to_string(&self) -> String {
        format!("{{ operator: {:?}, left: {}, right: {} }}", self.operator, self.left.to_string(), self.right.to_string())
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::InfixExpression
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ExpressionStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let expr = if let Some(token) = parser.next(0) {
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

pub struct FnCallExpression {
    pub ident: Identifier,
    pub args: Vec<Box<dyn Expression>>,
}

impl Expression for FnCallExpression {
    fn to_string(&self) -> String {
        let mut result = format!("{{ type: fnCall, name: {}, args: {{ ", self.ident.0);

        for arg in &self.args {
            result.push_str(&arg.to_string());
            result.push_str(", ");
        }

        result.push_str("} }");
        result
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::FnCallExpression
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