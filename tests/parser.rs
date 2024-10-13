use mamba::lexer::{Lexer, Token};
use mamba::parser::pratt::PrattParser;
use mamba::parser::*;

fn get_tokens(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.into());
    lexer.get_tokens().unwrap()
}

fn test_prefix_expression(tokens: Vec<Token>) -> Expression {
    let mut parser = Parser::new(tokens);
    let result = PrattParser::parse_nud(&mut parser).unwrap();
    
    result
}

#[test]
fn test_prefix_expressions() {
    assert_eq!(test_prefix_expression(get_tokens("+123")).to_string(), "{ operator: UnaryPlus, right: Integer(123) }");
    assert_eq!(test_prefix_expression(get_tokens("-123")).to_string(), "{ operator: UnaryMinus, right: Integer(123) }");
    assert_eq!(test_prefix_expression(get_tokens("~123")).to_string(), "{ operator: BitwiseNot, right: Integer(123) }");
    assert_eq!(test_prefix_expression(get_tokens("+foo")).to_string(), "{ operator: UnaryPlus, right: foo }");
    assert_eq!(test_prefix_expression(get_tokens("-foo")).to_string(), "{ operator: UnaryMinus, right: foo }");
    assert_eq!(test_prefix_expression(get_tokens("~foo")).to_string(), "{ operator: BitwiseNot, right: foo }");
}

fn test_expression(tokens: Vec<Token>) -> Expression {
    let mut parser = Parser::new(tokens);
    let result = PrattParser::parse_expr(&mut parser, mamba::parser::pratt::Precedence::Lowest, None).unwrap();

    result
}

#[test]
fn test_simple_expressions() {
    assert_eq!(test_expression(get_tokens("a + b")).to_string(), "{ operator: Plus, left: a, right: b }");
    assert_eq!(test_expression(get_tokens("a - b")).to_string(), "{ operator: Minus, left: a, right: b }");
    assert_eq!(test_expression(get_tokens("a * b")).to_string(), "{ operator: Multiply, left: a, right: b }");
    assert_eq!(test_expression(get_tokens("a / b")).to_string(), "{ operator: Divide, left: a, right: b }");
    assert_eq!(test_expression(get_tokens("a % b")).to_string(), "{ operator: Modulo, left: a, right: b }");
    assert_eq!(test_expression(get_tokens("a & b")).to_string(), "{ operator: BitwiseAnd, left: a, right: b }");
    assert_eq!(test_expression(get_tokens("a ^ b")).to_string(), "{ operator: BitwiseXor, left: a, right: b }");
    assert_eq!(test_expression(get_tokens("a | b")).to_string(), "{ operator: BitwiseOr, left: a, right: b }");
}

#[test]
fn test_mixed_expressions() {
    assert_eq!(test_expression(get_tokens("-a * b")).to_string(), "{ operator: Multiply, left: { operator: UnaryMinus, right: a }, right: b }");
    assert_eq!(test_expression(get_tokens("~-a")).to_string(), "{ operator: BitwiseNot, right: { operator: UnaryMinus, right: a } }");
    assert_eq!(test_expression(get_tokens("a + b + c")).to_string(), "{ operator: Plus, left: { operator: Plus, left: a, right: b }, right: c }");
    assert_eq!(test_expression(get_tokens("a + b - c")).to_string(), "{ operator: Minus, left: { operator: Plus, left: a, right: b }, right: c }");
    assert_eq!(test_expression(get_tokens("a * b * c")).to_string(), "{ operator: Multiply, left: { operator: Multiply, left: a, right: b }, right: c }");
    assert_eq!(test_expression(get_tokens("a * b / c")).to_string(), "{ operator: Divide, left: { operator: Multiply, left: a, right: b }, right: c }");
    assert_eq!(test_expression(get_tokens("a + b * c + d / e - f")).to_string(), "{ operator: Minus, left: { operator: Plus, left: { operator: Plus, left: a, right: { operator: Multiply, left: b, right: c } }, right: { operator: Divide, left: d, right: e } }, right: f }");
    assert_eq!(test_expression(get_tokens("5 > 4 == 3 < 4")).to_string(), "{ operator: Equal, left: { operator: Greater, left: Integer(5), right: Integer(4) }, right: { operator: Less, left: Integer(3), right: Integer(4) } }");
    assert_eq!(test_expression(get_tokens("5 < 4 != 3 > 4")).to_string(), "{ operator: NotEqual, left: { operator: Less, left: Integer(5), right: Integer(4) }, right: { operator: Greater, left: Integer(3), right: Integer(4) } }");
    assert_eq!(test_expression(get_tokens("1 + (2 + 3) + 4")).to_string(), "{ operator: Plus, left: { operator: Plus, left: Integer(1), right: { operator: Plus, left: Integer(2), right: Integer(3) } }, right: Integer(4) }");
    assert_eq!(test_expression(get_tokens("(5 + 5) * 2")).to_string(), "{ operator: Multiply, left: { operator: Plus, left: Integer(5), right: Integer(5) }, right: Integer(2) }");
    assert_eq!(test_expression(get_tokens("2 / (5 + 5)")).to_string(), "{ operator: Divide, left: Integer(2), right: { operator: Plus, left: Integer(5), right: Integer(5) } }");
    assert_eq!(test_expression(get_tokens("-(5 + 5)")).to_string(), "{ operator: UnaryMinus, right: { operator: Plus, left: Integer(5), right: Integer(5) } }");
}