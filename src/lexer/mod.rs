mod token;
pub use token::{Token, Keyword, Literal};

use std::cmp::Ordering;
use crate::{error::LexerError, types::{DataType, FloatingPoint, SignedInteger, UnsignedInteger}};

pub struct Lexer {
    source: Vec<Vec<char>>,
    line: usize,
    pos: usize,
    indent: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.split('\n').map(|s| s.chars().collect()).collect(),
            line: 0,
            pos: 0,
            indent: 0,
        }
    }

    fn next(&self, n: usize) -> Option<char> {
        let line = &self.source[self.line];

        if (self.pos + n) < line.len() {
            Some(line[self.pos + n])
        } else {
            None
        }
    }

    fn read_keyword(ident: &str) -> Token {
        match ident {
            "if" => Token::Keyword(Keyword::If),
            "elif" => Token::Keyword(Keyword::Elif),
            "else" => Token::Keyword(Keyword::Else),
            "extern" => Token::Keyword(Keyword::Extern),
            "def" => Token::Keyword(Keyword::Def),
            "let" => Token::Keyword(Keyword::Let),
            "return" => Token::Keyword(Keyword::Return),

            "void" => Token::Keyword(Keyword::DataType(DataType::void)),
            "bool" => Token::Keyword(Keyword::DataType(DataType::bool)),
            "str" => Token::Keyword(Keyword::DataType(DataType::str)),
            "i8" => Token::Keyword(Keyword::DataType(DataType::SignedInteger(SignedInteger::i8))),
            "i16" => Token::Keyword(Keyword::DataType(DataType::SignedInteger(SignedInteger::i16))),
            "i32" => Token::Keyword(Keyword::DataType(DataType::SignedInteger(SignedInteger::i32))),
            "i64" => Token::Keyword(Keyword::DataType(DataType::SignedInteger(SignedInteger::i64))),
            "i128" => Token::Keyword(Keyword::DataType(DataType::SignedInteger(SignedInteger::i128))),
            "u8" => Token::Keyword(Keyword::DataType(DataType::UnsignedInteger(UnsignedInteger::u8))),
            "u16" => Token::Keyword(Keyword::DataType(DataType::UnsignedInteger(UnsignedInteger::u16))),
            "u32" => Token::Keyword(Keyword::DataType(DataType::UnsignedInteger(UnsignedInteger::u32))),
            "u64" => Token::Keyword(Keyword::DataType(DataType::UnsignedInteger(UnsignedInteger::u64))),
            "u128" => Token::Keyword(Keyword::DataType(DataType::UnsignedInteger(UnsignedInteger::u128))),
            "f32" => Token::Keyword(Keyword::DataType(DataType::FloatingPoint(FloatingPoint::f32))),
            "f64" => Token::Keyword(Keyword::DataType(DataType::FloatingPoint(FloatingPoint::f64))),
            "f128" => Token::Keyword(Keyword::DataType(DataType::FloatingPoint(FloatingPoint::f128))),

            _ => Token::Identifier(ident.into()),
        }
    }

    fn read_indent(&mut self) -> usize {
        let mut space: usize = 0;

        loop {
            if self.pos < self.source[self.line].len() {
                if self.source[self.line][self.pos] == ' ' {
                    space += 1;
                    self.pos += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        space / 4
    }

    fn read_number(&mut self) -> i128 {
        let mut result: i128 = 0;

        while self.pos < self.source[self.line].len() {
            if self.source[self.line][self.pos].is_ascii_digit() {
                result *= 10;
                result += self.source[self.line][self.pos].to_digit(10).unwrap() as i128;
                self.pos += 1;
            } else {
                break;
            }
        }

        result
    }

    fn read_string(&mut self) -> Result<String, LexerError> {
        let mut result = String::new();

        while self.pos < self.source[self.line].len() {
            if let Some(next) = self.next(1) {
                self.pos += 1;

                if next != '"' {
                    result.push(next);
                } else {
                    break;
                }
            } else {
                return Err(LexerError("closing quotation mark expected".into()));
            }
        }

        Ok(result)
    }

    fn read_ident(&mut self) -> String {
        let mut result = String::new();

        result.push(self.source[self.line][self.pos]);

        while let Some(next) = self.next(1) {
            if next.is_ascii_alphanumeric() || next == '_' {
                result.push(next);
                self.pos += 1;
            } else {
                break;
            }
        }
        
        result
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut indent_count: usize = 0;

        for i in 0..self.source.len() {
            self.pos = 0;
            self.line = i;

            let indent = self.read_indent();
            let indent_diff = (indent as isize) - (self.indent as isize);

            match indent_diff.cmp(&0) {
                Ordering::Greater => {
                    for _ in 0..indent_diff {
                        tokens.push(Token::Indent);
                        indent_count += 1;
                    }
                },
                Ordering::Less => {
                    for _ in 0..-indent_diff {
                        tokens.push(Token::Dedent);
                        indent_count -= 1;
                    }
                },
                Ordering::Equal => { },
            }

            self.indent = indent;

            while self.pos < self.source[self.line].len() {
                match self.source[self.line][self.pos] {
                    ' ' => {
                        self.pos += 1;
                    },
                    '#' => {
                        self.pos += 1;
                        break;
                    },
                    // TODO: parse l, ll, lll, u, ul, ull, ulll suffix
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => tokens.push(Token::Literal(Literal::SignedInteger((self.read_number(), SignedInteger::i32)))),
                    '"' => {
                        tokens.push(Token::Literal(Literal::String(self.read_string().unwrap())));
                        self.pos += 1;
                    },
                    '(' => {
                        tokens.push(Token::LParen);
                        self.pos += 1;
                    },
                    ')' => {
                        tokens.push(Token::RParen);
                        self.pos += 1;
                    },
                    '[' => {
                        tokens.push(Token::LSqBr);
                        self.pos += 1;
                    },
                    ']' => {
                        tokens.push(Token::RSqBr);
                        self.pos += 1;
                    },
                    ':' => {
                        tokens.push(Token::Colon);
                        self.pos += 1;
                    },
                    ',' => {
                        tokens.push(Token::Comma);
                        self.pos += 1;
                    },
                    ';' => {
                        tokens.push(Token::Semicolon);
                        self.pos += 1;
                    },
                    '+' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::PlusEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Plus);
                        self.pos += 1;
                    },
                    '-' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::MinusEqual);
                                self.pos += 2;
                                continue;
                            } else if *next == '>' {
                                tokens.push(Token::RArrow);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Minus);
                        self.pos += 1;
                    },
                    '*' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::StarEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Star);
                        self.pos += 1;
                    },
                    '/' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::SlashEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Slash);
                        self.pos += 1;
                    },
                    '|' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::VBarEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::VBar);
                        self.pos += 1;
                    },
                    '&' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::AmpersandEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Ampersand);
                        self.pos += 1;
                    },
                    '<' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '<' {
                                if let Some(next2) = &self.next(2) {
                                    if *next2 == '=' {
                                        tokens.push(Token::LeftShiftEqual);
                                        self.pos += 3;
                                        continue;
                                    } else {
                                        tokens.push(Token::LeftShift);
                                        self.pos += 2;
                                        continue;
                                    }
                                } else {
                                    tokens.push(Token::LeftShift);
                                    self.pos += 2;
                                    continue;
                                }
                            } else if *next == '=' {
                                tokens.push(Token::LessEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Less);
                        self.pos += 1;
                    },
                    '>' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '>' {
                                if let Some(next2) = &self.next(2) {
                                    if *next2 == '=' {
                                        tokens.push(Token::RightShiftEqual);
                                        self.pos += 3;
                                        continue;
                                    } else {
                                        tokens.push(Token::RightShift);
                                        self.pos += 2;
                                        continue;
                                    }
                                } else {
                                    tokens.push(Token::RightShift);
                                    self.pos += 2;
                                    continue;
                                }
                            } else if *next == '=' {
                                tokens.push(Token::GreaterEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Greater);
                        self.pos += 1;
                    },
                    '=' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::EqualEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Equal);
                        self.pos += 1;
                    },
                    '.' => {
                        tokens.push(Token::Dot);
                        self.pos += 1;
                    },
                    '%' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::PercentEqual);
                                continue;
                            }
                        }

                        tokens.push(Token::Percent);
                        self.pos += 1;
                    },
                    '{' => {
                        tokens.push(Token::LBrace);
                        self.pos += 1;
                    },
                    '}' => {
                        tokens.push(Token::RBrace);
                        self.pos += 1;
                    },
                    '~' => {
                        tokens.push(Token::Tilde);
                        self.pos += 1;
                    }
                    '^' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::CircumflexEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Circumflex);
                        self.pos += 1;
                    },
                    '!' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::NotEqual);
                                self.pos += 2;
                                continue;
                            }
                        }
                    },
                    _ => {
                        tokens.push(Lexer::read_keyword(&self.read_ident()));
                        self.pos += 1;
                    },
                }
            }
        }

        for _ in 0..indent_count {
            tokens.push(Token::Dedent);
        }
        
        tokens.push(Token::EOF);

        Ok(tokens)
    }
}