use mamba::lexer::{Lexer, Token, Keyword, Literal, Identifier};
use mamba::parser::Parser;
use mamba::parser::ast::Expression;
use mamba::parser::pratt::PrattParser;

fn get_tokens(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.into());
    lexer.get_tokens().unwrap()
}

fn test_prefix_expression(tokens: Vec<Token>) -> Box<dyn Expression> {
    let mut parser = Parser::new(tokens);
    let result = PrattParser::parse_nud(&mut parser).unwrap();
    
    result
}

#[test]
fn test_prefix_expressions() {
    assert_eq!(test_prefix_expression(get_tokens("+123")).to_string(), "{ operator: UnaryPlus, right: Number(123) }");
    assert_eq!(test_prefix_expression(get_tokens("-123")).to_string(), "{ operator: UnaryMinus, right: Number(123) }");
    assert_eq!(test_prefix_expression(get_tokens("~123")).to_string(), "{ operator: BitwiseNot, right: Number(123) }");
    assert_eq!(test_prefix_expression(get_tokens("+foo")).to_string(), "{ operator: UnaryPlus, right: foo }");
    assert_eq!(test_prefix_expression(get_tokens("-foo")).to_string(), "{ operator: UnaryMinus, right: foo }");
    assert_eq!(test_prefix_expression(get_tokens("~foo")).to_string(), "{ operator: BitwiseNot, right: foo }");
}

fn test_expression(tokens: Vec<Token>) -> Box<dyn Expression> {
    let mut parser = Parser::new(tokens);
    let result = PrattParser::parse_expression(&mut parser, mamba::parser::pratt::Precedence::Lowest).unwrap();

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
    assert_eq!(test_expression(get_tokens("5 > 4 == 3 < 4")).to_string(), "{ operator: Equal, left: { operator: Greater, left: Number(5), right: Number(4) }, right: { operator: Less, left: Number(3), right: Number(4) } }");
    assert_eq!(test_expression(get_tokens("5 < 4 != 3 > 4")).to_string(), "{ operator: NotEqual, left: { operator: Less, left: Number(5), right: Number(4) }, right: { operator: Greater, left: Number(3), right: Number(4) } }");
    assert_eq!(test_expression(get_tokens("1 + (2 + 3) + 4")).to_string(), "{ operator: Plus, left: { operator: Plus, left: Number(1), right: { operator: Plus, left: Number(2), right: Number(3) } }, right: Number(4) }");
    assert_eq!(test_expression(get_tokens("(5 + 5) * 2")).to_string(), "{ operator: Multiply, left: { operator: Plus, left: Number(5), right: Number(5) }, right: Number(2) }");
    assert_eq!(test_expression(get_tokens("2 / (5 + 5)")).to_string(), "{ operator: Divide, left: Number(2), right: { operator: Plus, left: Number(5), right: Number(5) } }");
    assert_eq!(test_expression(get_tokens("-(5 + 5)")).to_string(), "{ operator: UnaryMinus, right: { operator: Plus, left: Number(5), right: Number(5) } }");
}