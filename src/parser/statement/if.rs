use crate::parser::{Expression, Keyword, Parser, PrattParser, Token};
use crate::error::ParseError;

use crate::parser::pratt::Precedence;
use super::Statement;
use std::borrow::Borrow;
use std::fmt;

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub then: Vec<Statement>,
    pub r#else: Box<IfBranch>,
    // pub r#else: Option<Box<Statement>>,
}

#[derive(Debug)]
pub enum IfBranch {
    Elif(IfStatement),
    Else(Vec<Statement>),
    None,
}

pub fn parse_if(parser: &mut Parser) -> Result<IfStatement, ParseError> {
    let condition = PrattParser::parse_expr(parser, Precedence::Lowest, None).unwrap();
    parser.pos += 1;

    if let Some(token) = parser.next(0) {
        if let Token::Colon = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[IfStatement] expected `:`, found {token:?}")));
        }
    } else {
        return Err(ParseError("[IfStatement] insufficient tokens".into()));
    }

    if let Some(token) = parser.next(0) {
        if let Token::Indent = token {
            parser.pos += 1;
        } else {
            return Err(ParseError(format!("[IfStatement] expected indent, found {token:?}")));
        }
    } else {
        return Err(ParseError("[IfStatement] insufficient tokens".into()));
    }

    let mut then: Vec<Statement> = Vec::new();

    loop {
        if let Some(token) = parser.next(0) {
            if let Token::Dedent = token {
                parser.pos += 1;
                break;
            } else if let Some(Token::Keyword(keyword)) = parser.next(0) {
                if keyword == Keyword::Elif || keyword == Keyword::Else {
                    break;
                }
            }

            let stmt = parser.parse_stmt().unwrap().unwrap();
            then.push(stmt);
        } else {
            return Err(ParseError("[IfStatement] insufficient tokens".into()));
        }
    }

    let branch = if let Some(Token::Keyword(keyword)) = parser.next(0) {
        if keyword == Keyword::Elif {
            parser.pos += 1;
            
            let elif = parse_if(parser).unwrap();
            IfBranch::Elif(elif)
        } else if keyword == Keyword::Else {
            parser.pos += 1;

            if let Some(token) = parser.next(0) {
                if let Token::Colon = token {
                    parser.pos += 1;
                } else {
                    return Err(ParseError(format!("[IfStatement] expected `:`, found {token:?}")));
                }
            } else {
                return Err(ParseError("[IfStatement] insufficient tokens".into()));
            }

            if let Some(token) = parser.next(0) {
                if let Token::Indent = token {
                    parser.pos += 1;
                } else {
                    return Err(ParseError(format!("[IfStatement] expected indent, found {token:?}")));
                }
            } else {
                return Err(ParseError("[IfStatement] insufficient tokens".into()));
            }
    
            let mut r#else : Vec<Statement> = Vec::new();

            loop {
                if let Some(token) = parser.next(0) {
                    if let Token::Dedent = token {
                        break;
                    }

                    let stmt = parser.parse_stmt().unwrap().unwrap();
                    r#else.push(stmt);
                } else {
                    return Err(ParseError("[IfStatement] insufficient tokens".into()));
                }
            }

            if let Some(token) = parser.next(0) {
                if let Token::Dedent = token {
                    parser.pos += 1;
                } else {
                    return Err(ParseError(format!("[IfStatement] expected dedent, found {token:?}")));
                }
            } else {
                return Err(ParseError("[IfStatement] insufficient tokens".into()));
            }

            IfBranch::Else(r#else)
        } else {
            IfBranch::None
        }
    } else {
        IfBranch::None
        // end of if
    };

    Ok(IfStatement {
        condition,
        then,
        r#else: Box::new(branch),
    })
}

impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.r#else.borrow() {
            IfBranch::Elif(stmt) => write!(f, "{{ type: if, condition: {}, then: {:?}, else: {} }}", self.condition, self.then, stmt),
            IfBranch::Else(stmt) => write!(f, "{{ type: if, condition: {}, then: {:?}, else: {:?} }}", self.condition, self.then, stmt),
            IfBranch::None => write!(f, "{{ type: if, condition: {}, then: {:?} }}", self.condition, self.then),
        }
    }
}