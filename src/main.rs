pub mod chunk;
pub mod vm;

use crate::chunk::{op_code::OpCode, Chunk};
use crate::vm::VirtualMachine;

fn main() {
    let mut chunk = Chunk::new();
    // ====================================================
    chunk.write(OpCode::Constant(1.2), 123);
    // chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Constant(3.8), 123);
    chunk.write(OpCode::Subtract, 123);
    chunk.write(OpCode::Return, 123);
    // chunk.write(OpCode::Return, 123);
    // chunk.write(OpCode::Return, 123);


    println!("== test chunk ==\n{:?}", &chunk);
    let mut vm = VirtualMachine::new(chunk);
    let _result = vm.interpret();
}
