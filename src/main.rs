#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![warn(unused_must_use)]

mod compiler;
mod tests;

use compiler::{tokenize};

fn main() {
    let tokens = match tokenize("# Gonna tokenize this whole thing!
var foo: u8: 69;  # Can do inline comments too") {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(err) => {
            println!("Tokenizing failed: {:?}", err);
            return;
        }
    };

    // let ast = match parse(tokens) {
    //     Ok(ast) => ast,
    //     Err(err) => {
    //         println!("Parsing failed: {:?}", err);
    //         return;
    //     }
    // };
}
