use crate::chunk::op_code::OpCode;
use crate::chunk::value::Value;
use crate::chunk::Chunk;
use interpret_result::InterpretResult;

pub mod interpret_result;

macro_rules! binary_operation {
    ($x: expr, $y: expr, $op: tt) => {
            $x $op $y
    };
}

pub struct VirtualMachine {
    chunk: Chunk,
    stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        for code in self.chunk.get_codes() {
            match code {
                OpCode::Constant(value) => self.stack.push(*value),
                OpCode::Return => match self.stack.pop() {
                    Some(value) => println!("{}", value),
                    None => panic!("Empty stack!"),
                },
                OpCode::Negate => {
                    let value = match self.stack.pop() {
                        Some(value) => value,
                        None => panic!("Empty stack!"),
                    };
                    self.stack.push(-value);
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    let a = match self.stack.pop() {
                        Some(value) => value,
                        None => panic!("Empty stack!"),
                    };

                    let b = match self.stack.pop() {
                        Some(value) => value,
                        None => panic!("Empty stack!"),
                    };

                    let c = match code {
                        OpCode::Add => binary_operation!(a, b , +),
                        OpCode::Subtract => binary_operation!(a, b , -),
                        OpCode::Multiply => binary_operation!(a, b , *),
                        OpCode::Divide => binary_operation!(a, b , /),
                        _ => unreachable!(),
                    };

                    self.stack.push(c);
                }
            }
        }

        InterpretResult::Ok
    }
}

    // fn push(&mut self, value: Value) {
    //     self.stack.push(value);
    // }

    // fn pop(&mut self) -> Value {
    //     match self.stack.pop() {
    //         Some(value) => value,
    //         None => panic!("Empty stack!"),
    //     }
    // }

    
    // for code in codes.iter() {
    //     match code {
    //         OpCode::Constant(value) => self.push(*value),
    //         OpCode::Negate => {
    //             let value = -self.pop();
    //             self.push(value);
    //         }
    //         OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
    //            ;
    //         },
    //         OpCode::Return => {
    //             match self.stack.pop() {
    //                 Some(value) => println!("{}", value),
    //                 None => panic!("Empty stack!"),
    //             }
    //             return InterpretResult::Ok;
    //         } // _ => self.binary_operation(instruction),
    //     }

    // fn binary_operation(&mut self, op_code: &OpCode) {
    //     let a = match self.stack.pop() {
    //         Some(v) => v,
    //         None => panic!("Empty stack!"),
    //     };

    //     let b = match self.stack.pop() {
    //         Some(v) => v,
    //         None => panic!("Empty stack!"),
    //     };

    //     let c = match op_code {
    //         OpCode::Add => a + b,
    //         OpCode::Divide => a / b,
    //         OpCode::Subtract => a - b,
    //         OpCode::Multiply => a * b,
    //         _ => unreachable!(),
    //     };

    //     self.stack.push(c);
    // }