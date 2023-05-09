pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Operand>,
}

pub enum Operand {
    Condition(Condition),
    Value(Value),
    Register(Register),
}

pub enum Condition {
    eq,
    ne,
    ugt,
    uge,
    ult,
    ule,
    sgt,
    sge,
    slt,
    sle
}

pub enum Value {
    i8(i8),
    i16(i16),
    i32(i32),
    i64(i64),
}

pub struct Register(pub String);

impl Condition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Condition::eq => "eq",
            Condition::ne => "ne",
            Condition::ugt => "ugt",
            Condition::uge => "uge",
            Condition::ult => "ult",
            Condition::ule => "ule",
            Condition::sgt => "sgt",
            Condition::sge => "sge",
            Condition::slt => "slt",
            Condition::sle => "sle",
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::i8(val) => format!("i8 {val}"),
            Value::i16(val) => format!("i16 {val}"),
            Value::i32(val) => format!("i32 {val}"),
            Value::i64(val) => format!("i64 {val}"),
        }
    }
}

impl Operand {
    pub fn to_string(&self) -> String {
        match self {
            Operand::Condition(cond) => format!("{}", cond.as_str()),
            Operand::Value(val) => format!("{}", val.to_string()),
            Operand::Register(reg) => format!("%{}", reg.0),
        }
    }
}

impl Instruction {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let operands: Vec<String> = self.operands.iter().map(|operand| operand.to_string()).collect();

        result += &self.opcode;
        result += " ";
        result += &operands.join(", ");

        result
    }
}