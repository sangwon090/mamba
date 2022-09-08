use mamba::lexer::{Lexer, Token, Keyword, Literal};

const SAMPLE_CODE: &str = "
let foo = 123

def sum(a: int, b: int) -> int:
    return a + b

def main() -> void:
    print(\"hello, world!\")";


#[test]
fn test_lexer() {
    let mut lexer = Lexer::new(SAMPLE_CODE.into());
    let tokens = lexer.get_tokens().unwrap();

    assert_eq!(tokens[0], Token::Keyword(Keyword::Let));
    assert_eq!(tokens[1], Token::Identifier("foo".into()));
    assert_eq!(tokens[2], Token::Equal);
    assert_eq!(tokens[3], Token::Literal(Literal::Number(123)));
    assert_eq!(tokens[4], Token::Keyword(Keyword::Def));
    assert_eq!(tokens[5], Token::Identifier("sum".into()));
    assert_eq!(tokens[6], Token::LParen);
    assert_eq!(tokens[7], Token::Identifier("a".into()));
    assert_eq!(tokens[8], Token::Colon);
    assert_eq!(tokens[9], Token::Keyword(Keyword::Int));
    assert_eq!(tokens[10], Token::Comma);
    assert_eq!(tokens[11], Token::Identifier("b".into()));
    assert_eq!(tokens[12], Token::Colon);
    assert_eq!(tokens[13], Token::Keyword(Keyword::Int));
    assert_eq!(tokens[14], Token::RParen);
    assert_eq!(tokens[15], Token::RArrow);
    assert_eq!(tokens[16], Token::Keyword(Keyword::Int));
    assert_eq!(tokens[17], Token::Colon);
    assert_eq!(tokens[18], Token::Indent);
    assert_eq!(tokens[19], Token::Identifier("return".into()));
    assert_eq!(tokens[20], Token::Identifier("a".into()));
    assert_eq!(tokens[21], Token::Plus);
    assert_eq!(tokens[22], Token::Identifier("b".into()));
    assert_eq!(tokens[23], Token::Dedent);
    assert_eq!(tokens[24], Token::Keyword(Keyword::Def));
    assert_eq!(tokens[25], Token::Identifier("main".into()));
    assert_eq!(tokens[26], Token::LParen);
    assert_eq!(tokens[27], Token::RParen);
    assert_eq!(tokens[28], Token::RArrow);
    assert_eq!(tokens[29], Token::Keyword(Keyword::Void));
    assert_eq!(tokens[30], Token::Colon);
    assert_eq!(tokens[31], Token::Indent);
    assert_eq!(tokens[32], Token::Identifier("print".into()));
    assert_eq!(tokens[33], Token::LParen);
    assert_eq!(tokens[34], Token::Literal(Literal::String("hello, world!".into())));
    assert_eq!(tokens[35], Token::RParen);
    assert_eq!(tokens[36], Token::EOF);
}