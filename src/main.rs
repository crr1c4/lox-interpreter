use lox_interpreter::chunk::{op_code::OpCode, Chunk};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::OpReturn as u8, 123);
    println!("==test chunk==\n{:?}", chunk);
}
