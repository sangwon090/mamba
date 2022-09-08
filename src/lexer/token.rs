#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
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

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Let,
    Def,
    Int,
    Void,
    Return,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Number(i64),
    String(String),
}