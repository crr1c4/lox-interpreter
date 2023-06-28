pub mod scanner;
pub mod token;

use scanner::Scanner;
use token::*;

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);
    let mut line: isize = -1;

    loop {
        let token = scanner.scan_token();
        
        if token.line() as isize != line {
            print!("{:4} ", token.line());
            line = token.line() as isize;
        } else {
            println!("   | ");
        }

        println!("Token {:?}", token);

        if *token.token_type() == TokenType::EOF {
            break;
        }
    }
}
