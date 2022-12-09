#![allow(dead_code)]

pub mod lexer;
pub mod parser;

use lexer::lexer::Lexer;
use parser::parser::parse;

fn main() {
    let contents: Vec<char> = std::fs::read_to_string("../simple_input.opal")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    let tokens = Lexer::new(contents).collect();

    match parse(tokens) {
        Ok(statements) => for statement in statements {
            println!("{:?}", statement);
        },
        Err(error) => println!("Received parser error: {:?}", error),
    }
}
