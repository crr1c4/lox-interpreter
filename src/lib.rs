pub mod chunk;
pub mod compiler;
pub mod vm;

// use crate::chunk::{op_code::OpCode, Chunk};
use crate::vm::InterpretResult;
use crate::vm::VirtualMachine;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

pub fn repl() {
    loop {
        let mut line = String::new();

        print!(" > ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.eq("\n") {
            break;
        }

        VirtualMachine::interpret(line);
    }
}

pub fn run_file(path: String) {
    println!("{path}");
    let file = fs::read_to_string(path).expect("Failed to read file");
    let result = VirtualMachine::interpret(file);

    match result {
        InterpretResult::CompileError => exit(65),
        InterpretResult::RuntimeError => exit(70),
        _ => (),
    }
}
