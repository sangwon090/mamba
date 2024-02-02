use serde::{Deserialize, Serialize};
use std::fmt;

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

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Identifier(pub String);

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Literal {
    Number(i64),
    String(String),
}