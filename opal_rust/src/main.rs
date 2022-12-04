#![allow(dead_code)]

pub mod lexer;
pub mod parser;

use lexer::lexer::Lexer;

fn main() {
    let contents: Vec<char> = std::fs::read_to_string("../simple_input.opal")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    for token in Lexer::new(contents) {
        println!("{:?}", token);
    }
}
