use crate::{parser::Identifier, types::{DataType, SignedInteger, UnsignedInteger}};

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    If,
    Else,
    Let,
    Def,
    Return,
    Extern,
    DataType(DataType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    SignedInteger((i128, SignedInteger)),
    UnsignedInteger((u128, UnsignedInteger)),
    String(String), 
}