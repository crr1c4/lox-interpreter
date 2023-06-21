use super::value::Value;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Constant(Value),
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}