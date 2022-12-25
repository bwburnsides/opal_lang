#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![warn(unused_must_use)]

mod compiler;
mod tests;

use compiler::*;

fn main() {
    // let tokens = Token::from_kinds(
    //     &vec![
    //         TokenKind::Keyword(Keyword::Var),
    //         TokenKind::Identifier(String::from("foo")),
    //         TokenKind::Colon,
    //         TokenKind::Keyword(Keyword::U8),
    //         TokenKind::Equal,
    //         TokenKind::IntegerLiteral(IntegerLiteralType::Decimal, 5),
    //         TokenKind::SemiColon,
    //     ]
    // );

    // match Parser::new(tokens).parse_declaration() {
    //     Ok((decl, remaining)) => {
    //         println!("Success!\n {:#?}\n", decl);
    //         println!("{:#?}", remaining);
    //     },
    //     Err(err) => {
    //         println!("Failure! Got:\n{:#?}", err);
    //     },
    // }

    println!("{:#?}", Lexer::new("var foo: u8 = 5;").next_token())
}
