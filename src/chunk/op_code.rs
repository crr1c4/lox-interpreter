use super::Value;

#[derive(Debug)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl From<&u8> for OpCode {
    fn from(value: &u8) -> Self {
        match value {
            0 => OpCode::OpConstant,
            1 => OpCode::OpReturn,
            _ => panic!("Unkonown code"),
        }
    }
}