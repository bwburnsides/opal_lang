#![allow(dead_code)]

pub mod lexer;
pub mod parser;

use lexer::lexer::Lexer;
use lexer::lexer::Token;
use parser::parser::Parser;

fn main() {
    let contents: Vec<char> = std::fs::read_to_string("../simple_input.opal")
        .expect("Should have been able to read the file")
        .chars()
        .collect();

    let tokens: Vec<Token> = Lexer::new(contents).collect();
    let statements = Parser::new(tokens).parse();
    for statement in statements {
        println!("{:?}", statement);
    }
}
