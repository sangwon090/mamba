use mamba::lexer::{Lexer, Token};
use mamba::parser::Parser;
use mamba::parser::pratt::PrattParser;

fn get_tokens(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.into());
    lexer.get_tokens().unwrap()
}

fn test_prefix_expression(tokens: Vec<Token>) -> String {
    let mut parser = Parser::new(tokens);
    let result = PrattParser::parse_nud(&mut parser).unwrap();
    
    format!("{:?}", result)
}

#[test]
fn test_prefix_expressions() {
    assert_eq!(test_prefix_expression(get_tokens("+123")), "Prefix({ operator: UnaryPlus, right: Literal(Number(123)) })");
    assert_eq!(test_prefix_expression(get_tokens("-123")), "Prefix({ operator: UnaryMinus, right: Literal(Number(123)) })");
    assert_eq!(test_prefix_expression(get_tokens("~123")), "Prefix({ operator: BitwiseNot, right: Literal(Number(123)) })");
    assert_eq!(test_prefix_expression(get_tokens("+foo")), "Prefix({ operator: UnaryPlus, right: Identifier(foo) })");
    assert_eq!(test_prefix_expression(get_tokens("-foo")), "Prefix({ operator: UnaryMinus, right: Identifier(foo) })");
    assert_eq!(test_prefix_expression(get_tokens("~foo")), "Prefix({ operator: BitwiseNot, right: Identifier(foo) })");
}

fn test_expression(tokens: Vec<Token>) -> String {
    let mut parser = Parser::new(tokens);
    let result = PrattParser::parse_expression(&mut parser, mamba::parser::pratt::Precedence::Lowest).unwrap();

    format!("{:?}", result)
}

#[test]
fn test_simple_expressions() {
    assert_eq!(test_expression(get_tokens("a + b")), "Infix({ operator: Plus, left: Identifier(a), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("a - b")), "Infix({ operator: Minus, left: Identifier(a), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("a * b")), "Infix({ operator: Multiply, left: Identifier(a), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("a / b")), "Infix({ operator: Divide, left: Identifier(a), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("a % b")), "Infix({ operator: Modulo, left: Identifier(a), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("a & b")), "Infix({ operator: BitwiseAnd, left: Identifier(a), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("a ^ b")), "Infix({ operator: BitwiseXor, left: Identifier(a), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("a | b")), "Infix({ operator: BitwiseOr, left: Identifier(a), right: Identifier(b) })");
}

#[test]
fn test_mixed_expressions() {
    assert_eq!(test_expression(get_tokens("-a * b")), "Infix({ operator: Multiply, left: Prefix({ operator: UnaryMinus, right: Identifier(a) }), right: Identifier(b) })");
    assert_eq!(test_expression(get_tokens("~-a")), "Prefix({ operator: BitwiseNot, right: Prefix({ operator: UnaryMinus, right: Identifier(a) }) })");
    assert_eq!(test_expression(get_tokens("a + b + c")), "Infix({ operator: Plus, left: Infix({ operator: Plus, left: Identifier(a), right: Identifier(b) }), right: Identifier(c) })");
    assert_eq!(test_expression(get_tokens("a + b - c")), "Infix({ operator: Minus, left: Infix({ operator: Plus, left: Identifier(a), right: Identifier(b) }), right: Identifier(c) })");
    assert_eq!(test_expression(get_tokens("a * b * c")), "Infix({ operator: Multiply, left: Infix({ operator: Multiply, left: Identifier(a), right: Identifier(b) }), right: Identifier(c) })");
    assert_eq!(test_expression(get_tokens("a * b / c")), "Infix({ operator: Divide, left: Infix({ operator: Multiply, left: Identifier(a), right: Identifier(b) }), right: Identifier(c) })");
    assert_eq!(test_expression(get_tokens("a + b * c + d / e - f")), "Infix({ operator: Minus, left: Infix({ operator: Plus, left: Infix({ operator: Plus, left: Identifier(a), right: Infix({ operator: Multiply, left: Identifier(b), right: Identifier(c) }) }), right: Infix({ operator: Divide, left: Identifier(d), right: Identifier(e) }) }), right: Identifier(f) })");
    assert_eq!(test_expression(get_tokens("5 > 4 == 3 < 4")), "Infix({ operator: Equal, left: Infix({ operator: Greater, left: Literal(Number(5)), right: Literal(Number(4)) }), right: Infix({ operator: Less, left: Literal(Number(3)), right: Literal(Number(4)) }) })");
    assert_eq!(test_expression(get_tokens("5 < 4 != 3 > 4")), "Infix({ operator: NotEqual, left: Infix({ operator: Less, left: Literal(Number(5)), right: Literal(Number(4)) }), right: Infix({ operator: Greater, left: Literal(Number(3)), right: Literal(Number(4)) }) })");
    assert_eq!(test_expression(get_tokens("1 + (2 + 3) + 4")), "Infix({ operator: Plus, left: Infix({ operator: Plus, left: Literal(Number(1)), right: Infix({ operator: Plus, left: Literal(Number(2)), right: Literal(Number(3)) }) }), right: Literal(Number(4)) })");
    assert_eq!(test_expression(get_tokens("(5 + 5) * 2")), "Infix({ operator: Multiply, left: Infix({ operator: Plus, left: Literal(Number(5)), right: Literal(Number(5)) }), right: Literal(Number(2)) })");
    assert_eq!(test_expression(get_tokens("2 / (5 + 5)")), "Infix({ operator: Divide, left: Literal(Number(2)), right: Infix({ operator: Plus, left: Literal(Number(5)), right: Literal(Number(5)) }) })");
    assert_eq!(test_expression(get_tokens("-(5 + 5)")), "Prefix({ operator: UnaryMinus, right: Infix({ operator: Plus, left: Literal(Number(5)), right: Literal(Number(5)) }) })");
}