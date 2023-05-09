use mamba::irgen::instruction::{Instruction, Operand, Value, Register};

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