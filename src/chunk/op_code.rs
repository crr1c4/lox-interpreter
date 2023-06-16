use super::value::Value;
use std::fmt::Debug;

#[repr(u8)]
pub enum OpCode {
    OpConstant(Value),
    OpReturn,
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_code = match self {
            OpCode::OpConstant(_) => "OP_CONSTANT",
            OpCode::OpReturn => "OP_RETURN",
            // _ => panic!("Unkonown operation code."),
        };

        write!(f, "{op_code}")
    }
}
