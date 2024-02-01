use mamba::downcast;
use mamba::irgen::expression::ExpressionGen;
use mamba::irgen::instruction::{Instruction, Operand, Value, Register};
use mamba::irgen::block::Block;
use mamba::lexer::{Lexer, Literal};
use mamba::parser::ast::{Expression, ExpressionStatement, InfixExpression, Operator};
use mamba::parser::Parser;

#[test]
fn test_instruction() {
    let operands: Vec<Operand> = vec! [
        Operand::Value(Value::i32(123)),
        Operand::Register(Register("a".into())),
    ];

    let instruction = Instruction {
        opcode: "icmp eq".into(),
        operands,
    };

    assert_eq!(instruction.to_string(), "icmp eq i32 123, %a");
}

#[test]
fn test_block() {
    let instructions = vec![Instruction {
        opcode: "icmp eq".into(),
        operands: vec! [
            Operand::Value(Value::i32(123)),
            Operand::Register(Register("a".into())),
        ],
    }, Instruction {
        opcode: "icmp ne".into(),
        operands: vec! [
            Operand::Value(Value::i32(123)),
            Operand::Register(Register("a".into())),
        ],
    }, Instruction {
        opcode: "icmp ugt".into(),
        operands: vec! [
            Operand::Value(Value::i32(123)),
            Operand::Register(Register("a".into())),
        ],
    }];

    let block = Block {
        name: "block".into(),
        instructions: Vec::new(),
    };

    println!("{}", block.to_string());
}

#[test]
fn test_expr_gen() {
    let code = "1 + 2 == 3;";
    
    let mut lexer = Lexer::new(code.into());
    let tokens = lexer.get_tokens().unwrap();
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_all();

    let expr_stmt = downcast!(ExpressionStatement, ast.statements[0]);
    println!("{:?}", expr_stmt.expression.get_type());

    let code = ExpressionGen::generate_code(&expr_stmt.expression);


    println!("{}", code);
}