mod token;
pub use token::{Token, Keyword, Literal};

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

    fn next(&self) -> Option<char> {
        let line = &self.source[self.line];

        if (self.pos + 1) < line.len() {
            Some(line[self.pos + 1])
        } else {
            None
        }
    }

    fn next2(&self) -> Option<char> {
        let line = &self.source[self.line];

        if (self.pos + 2) < line.len() {
            Some(line[self.pos + 2])
        } else {
            None
        }
    }

    fn get_keyword(ident: &str) -> Token {
        match ident {
            "def" => Token::Keyword(Keyword::Def),
            "let" => Token::Keyword(Keyword::Let),
            "int" => Token::Keyword(Keyword::Int),
            "void" => Token::Keyword(Keyword::Void),
            _ => Token::Identifier(ident.into()),
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        for (i, line) in self.source.iter().enumerate() {
            self.pos = 0;
            self.line = i;

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
            
            let indent_diff: isize = (space / 4) as isize - self.indent as isize;
            
            if indent_diff > 0 {
                for _ in 0..indent_diff {
                    tokens.push(Token::Indent);
                }
            } else if indent_diff < 0 {
                for _ in 0..-indent_diff {
                    tokens.push(Token::Dedent);
                }
            }

            self.indent = space / 4;

            while self.pos < line.len() {
                let char = &line[self.pos];

                match char {
                    ' ' => {
                        self.pos += 1;
                        continue
                    },
                    '#' => break,
    
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        let mut number_buffer: i64 = 0;

                        while self.pos < line.len() {
                            if line[self.pos].is_ascii_digit() {
                                number_buffer *= 10;
                                number_buffer += line[self.pos].to_digit(10).unwrap() as i64;
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }

                        tokens.push(Token::Literal(Literal::Number(number_buffer)));
                    },
                    '"' => {
                        let mut string_buffer = String::new();

                        while self.pos < line.len() {
                            if let Some(next) = self.next() {
                                self.pos += 1;

                                if next != '"' {
                                    string_buffer.push(next);
                                } else {
                                    break;
                                }
                            } else {
                                eprintln!("closing quotation mark not found");
                                break;
                            }
                        }

                        tokens.push(Token::Literal(Literal::String(string_buffer)));
                    },
                    '(' => tokens.push(Token::LParen),
                    ')' => tokens.push(Token::RParen),
                    '[' => tokens.push(Token::LSqBr),
                    ']' => tokens.push(Token::RSqBr),
                    ':' => tokens.push(Token::Colon),
                    ',' => tokens.push(Token::Comma),
                    ';' => tokens.push(Token::Semicolon),
                    '+' => {
                        if let Some(next) = &self.next() {
                            if *next == '=' {
                                tokens.push(Token::PlusEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Plus);
                    },
                    '-' => {
                        if let Some(next) = &self.next() {
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
                        if let Some(next) = &self.next() {
                            if *next == '=' {
                                tokens.push(Token::StarEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Star)
                    },
                    '/' => {
                        if let Some(next) = &self.next() {
                            if *next == '=' {
                                tokens.push(Token::SlashEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Slash)
                    },
                    '|' => {
                        if let Some(next) = &self.next() {
                            if *next == '=' {
                                tokens.push(Token::VBarEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::VBar)
                    },
                    '&' => {
                        if let Some(next) = &self.next() {
                            if *next == '=' {
                                tokens.push(Token::AmpersandEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Ampersand)
                    },
                    '<' => {
                        if let Some(next) = &self.next() {
                            if *next == '<' {
                                if let Some(next2) = &self.next2() {
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
                        if let Some(next) = &self.next() {
                            if *next == '>' {
                                if let Some(next2) = &self.next2() {
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
                        if let Some(next) = &self.next() {
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
                        if let Some(next) = &self.next() {
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
                        if let Some(next) = &self.next() {
                            if *next == '=' {
                                tokens.push(Token::CircumflexEqual);
                                self.pos += 2;
                                continue;
                            }
                        }

                        tokens.push(Token::Circumflex);
                    },
                    '!' => {
                        if let Some(next) = &self.next() {
                            if *next == '=' {
                                tokens.push(Token::NotEqual);
                                self.pos += 2;
                                continue;
                            }
                        }
                    },

                    _ => {
                        let mut ident_buffer = String::new();
                        ident_buffer.push(*char);

                        loop {
                            if let Some(next) = self.next() {
                                if next.is_ascii_alphanumeric() || next == '_' {
                                    ident_buffer.push(next);
                                    self.pos += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        tokens.push(Lexer::get_keyword(&ident_buffer));
                    },
                }

                self.pos += 1;
            }
        }

        tokens
    }
}