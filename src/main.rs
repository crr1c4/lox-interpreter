use lox_interpreter::{repl, run_file};
use std::env::args;
use std::process::exit;

fn main() {
    let mut args = args();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(args.nth(2).unwrap())
    } else {
        eprintln!("Usage, rlox [path]");
        exit(64);
    }
}

// let mut chunk = Chunk::new();
// // ====================================================
// chunk.write(OpCode::Constant(1.2), 123);
// // chunk.write(OpCode::Negate, 123);
// chunk.write(OpCode::Constant(3.8), 123);
// chunk.write(OpCode::Subtract, 123);
// chunk.write(OpCode::Return, 123);
// // chunk.write(OpCode::Return, 123);
// // chunk.write(OpCode::Return, 123);

// println!("== test chunk ==\n{:?}", &chunk);
// let mut vm = VirtualMachine::new(chunk);
// let _result = vm.interpret();
