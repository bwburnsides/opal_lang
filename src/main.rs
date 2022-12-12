#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![warn(unused_must_use)]

mod compiler;

use compiler::*;

fn main() {
    let tokens = Token::from_kinds(
        vec![
            TokenKind::Keyword(Keyword::Var),
            TokenKind::Identifier(String::from("foo")),
            TokenKind::Colon,
            TokenKind::Keyword(Keyword::U8),
            TokenKind::Equal,
            TokenKind::IntegerLiteral(IntegerLiteralType::Decimal, 5),
            TokenKind::SemiColon,
            //
            TokenKind::Keyword(Keyword::Const),
            TokenKind::Identifier(String::from("bar")),
            TokenKind::Colon,
            TokenKind::Keyword(Keyword::U8),
            TokenKind::Equal,
            TokenKind::IntegerLiteral(IntegerLiteralType::Decimal, 5),
            TokenKind::SemiColon,
            //
            TokenKind::Keyword(Keyword::Type),
            TokenKind::Identifier(String::from("baz")),
            TokenKind::Equal,
            TokenKind::Keyword(Keyword::U8),
            TokenKind::SemiColon,
        ]
    );

    for stmt in parse(tokens).unwrap() {
        println!("{:?}", stmt);
    }
}
