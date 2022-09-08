mod token;
pub use token::{Token, Keyword, Literal};

use crate::error::LexerError;

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

    fn read_ident(ident: &str) -> Token {
        match ident {
            "def" => Token::Keyword(Keyword::Def),
            "let" => Token::Keyword(Keyword::Let),
            "int" => Token::Keyword(Keyword::Int),
            "void" => Token::Keyword(Keyword::Void),
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

    fn read_number(&mut self) -> i64 {
        let mut result: i64 = 0;

        while self.pos < self.source[self.line].len() {
            if self.source[self.line][self.pos].is_ascii_digit() {
                result *= 10;
                result += self.source[self.line][self.pos].to_digit(10).unwrap() as i64;
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

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();

        result.push(self.source[self.line][self.pos]);

        loop {
            if let Some(next) = self.next(1) {
                if next.is_ascii_alphanumeric() || next == '_' {
                    result.push(next);
                    self.pos += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        result
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        for i in 0..self.source.len() {
            self.pos = 0;
            self.line = i;

            let indent = self.read_indent();
            let indent_diff = (indent as isize) - (self.indent as isize);

            if indent_diff > 0 {
                for _ in 0..indent_diff {
                    tokens.push(Token::Indent);
                }
            } else if indent_diff < 0 {
                for _ in 0..-indent_diff {
                    tokens.push(Token::Dedent);
                }
            }

            self.indent = indent;

            while self.pos < self.source[self.line].len() {
                let current = self.source[self.line][self.pos];

                match current {
                    ' ' => (),
                    '#' => break,
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => tokens.push(Token::Literal(Literal::Number(self.read_number()))),
                    '"' => tokens.push(Token::Literal(Literal::String(self.read_string().unwrap()))),
                    '(' => tokens.push(Token::LParen),
                    ')' => tokens.push(Token::RParen),
                    '[' => tokens.push(Token::LSqBr),
                    ']' => tokens.push(Token::RSqBr),
                    ':' => tokens.push(Token::Colon),
                    ',' => tokens.push(Token::Comma),
                    ';' => tokens.push(Token::Semicolon),
                    '+' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::PlusEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Plus);
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

                        tokens.push(Token::Minus)
                    },
                    '*' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::StarEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Star)
                    },
                    '/' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::SlashEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Slash)
                    },
                    '|' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::VBarEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::VBar)
                    },
                    '&' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::AmpersandEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Ampersand)
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
                    },
                    '.' => tokens.push(Token::Dot),
                    '%' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::PercentEqual);
                                continue;
                            }
                        }

                        tokens.push(Token::Percent);
                    },
                    '{' => tokens.push(Token::LBrace),
                    '}' => tokens.push(Token::RBrace),
                    '~' => tokens.push(Token::Tilde),
                    '^' => {
                        if let Some(next) = &self.next(1) {
                            if *next == '=' {
                                tokens.push(Token::CircumflexEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Circumflex);
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

                    _ => tokens.push(Lexer::read_ident(&self.read_identifier())),
                }

                self.pos += 1;
            }
        }

        tokens.push(Token::EOF);

        Ok(tokens)
    }
}