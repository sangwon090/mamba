use crate::parser::ast::{Expression, AstNodeType};
use serde::{Deserialize, Serialize};
use core::any::Any;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(Identifier),
    Keyword(Keyword),
    Literal(Literal),
    
    NewLine,
    Indent,
    Dedent,

    LParen,             // (
    RParen,             // )
    LSqBr,              // [
    RSqBr,              // ]
    Colon,              // :
    Comma,              // ,
    Semicolon,          // ;
    Plus,               // +
    Minus,              // -
    Star,               // *
    Slash,              // /
    VBar,               // |
    Ampersand,          // &
    Less,               // <
    Greater,            // >
    Equal,              // =
    Dot,                // .
    Percent,            // %
    LBrace,             // {
    RBrace,             // }
    EqualEqual,         // ==
    NotEqual,           // !=
    LessEqual,          // <=
    GreaterEqual,       // >=
    Tilde,              // ~
    Circumflex,         // ^
    LeftShift,          // <<
    RightShift,         // >>
//  DoubleStar,         // **
    PlusEqual,          // +=
    MinusEqual,         // -=
    StarEqual,          // *=
    SlashEqual,         // /=
    PercentEqual,       // %=
    AmpersandEqual,     // &=
    VBarEqual,          // |=
    CircumflexEqual,    // ^=
    LeftShiftEqual,     // <<=
    RightShiftEqual,    // >>=
//  DoubleStarEqual,    // **=
//  DoubleSlash,        // //
//  DoubleSlashEqual,   // //=
//  At,                 // @
//  AtEqual,            // @=
    RArrow,             // ->
//  Ellipsis,           // ...
//  ColonEqual,         // :=

    Unknown(String),
    EOF,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Identifier(pub String);

impl Expression for Identifier {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::Identifier
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    If,
    Else,
    Let,
    Def,
    Int,
    Str,
    Void,
    Return,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    Number(i64),
    String(String),
}

impl Expression for Literal {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn get_type(&self) -> AstNodeType {
        AstNodeType::Literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}