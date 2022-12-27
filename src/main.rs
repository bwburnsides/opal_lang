#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![warn(unused_must_use)]

mod compiler;
mod tests;

use compiler::{tokenize, parse};

fn main() {
    let tokens = match tokenize("var foo: u8: 69;") {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("Tokenizing failed: {:?}", err);
            return;
        }
    };

    let ast = match parse(tokens) {
        Ok(ast) => ast,
        Err(err) => {
            println!("Parsing failed: {:?}", err);
            return;
        }
    };
}
