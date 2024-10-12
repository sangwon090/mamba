mod token;
pub use token::{Token, Keyword};

use crate::{error::LexerError, parser::ast::Literal};

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
            "else" => Token::Keyword(Keyword::Else),
            "def" => Token::Keyword(Keyword::Def),
            "let" => Token::Keyword(Keyword::Let),
            "int" => Token::Keyword(Keyword::Int),
            "str" => Token::Keyword(Keyword::Str),
            "void" => Token::Keyword(Keyword::Void),
            "return" => Token::Keyword(Keyword::Return),
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

    fn read_ident(&mut self) -> String {
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
        let mut indent_count: usize = 0;

        for i in 0..self.source.len() {
            self.pos = 0;
            self.line = i;

            let indent = self.read_indent();
            let indent_diff = (indent as isize) - (self.indent as isize);

            if indent_diff > 0 {
                for _ in 0..indent_diff {
                    tokens.push(Token::Indent);
                    indent_count += 1;
                }
            } else if indent_diff < 0 {
                for _ in 0..-indent_diff {
                    tokens.push(Token::Dedent);
                    indent_count -= 1;
                }
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
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => tokens.push(Token::Literal(Literal::Integer(self.read_number()))),
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