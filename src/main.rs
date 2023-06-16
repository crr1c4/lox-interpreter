use lox_interpreter::chunk::{op_code::OpCode, Chunk};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpConstant(1.2), 123);
    chunk.write(OpCode::OpReturn, 123);
    println!("==test chunk==\n{:?}", chunk);
}
