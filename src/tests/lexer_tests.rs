#[cfg(test)]
use crate::compiler::*;

#[test]
fn create_token_from_kind() {
    let token = Token::from_kind(TokenKind::Ampersand);

    assert_eq!(
        token,
        Token {
            kind: TokenKind::Ampersand,
            position: TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                end: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
            },
        }
    );
}

#[test]
fn create_tokens_from_kinds() {
    let kinds = vec![TokenKind::Minus, TokenKind::GreaterThan];
    let tokens = Token::from_kinds(&kinds);

    for (idx, token) in tokens.iter().enumerate() {
        assert_eq!(kinds[idx], tokens[idx].kind,)
    }
}

#[test]
fn next_token_when_empty_input() {
    let mut lexer = Lexer::new("");
    assert_eq!(
        lexer.next_token(),
        LexResult::Err(LexError::new(
            LexErrorKind::InputExhausted,
            TextPosition {
                absolute: 0,
                line: 0,
                column: 0,
            }
        ))
    )
}

#[test]
fn non_ascii_character() {
    let actual = Lexer::new("😅").next_token().unwrap_err();

    assert_eq!(actual.kind, LexErrorKind::UnrecognizedCharacter);

    assert_eq!(
        actual.get_position(),
        TextPosition {
            absolute: 0,
            line: 0,
            column: 0
        }
    )
}

#[test]
fn non_opal_token() {
    let expected = Lexer::new("$").next_token().unwrap_err();

    assert_eq!(expected.kind, LexErrorKind::UnrecognizedCharacter);

    assert_eq!(
        expected.get_position(),
        TextPosition {
            absolute: 0,
            line: 0,
            column: 0
        }
    )
}

#[test]
fn valid_char_literal() {
    assert_eq!(
        Lexer::new("'f'").next_token().unwrap(),
        Token::new(
            TokenKind::CharLiteral('f'),
            TokenPosition::new(
                TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                TextPosition {
                    absolute: 2,
                    line: 0,
                    column: 2,
                },
            ),
        )
    )
}

#[test]
fn empty_char_literal_error_kind() {
    assert_eq!(
        Lexer::new("''").next_token().unwrap_err().kind,
        LexErrorKind::EmptyCharLiteral
    )
}

#[test]
fn empty_char_literal_error_position() {
    assert_eq!(
        Lexer::new("''").next_token().unwrap_err().get_position(),
        TextPosition {
            absolute: 0,
            line: 0,
            column: 0,
        }
    )
}

#[test]
fn decimal_integer_literal() {
    assert_eq!(
        Lexer::new("123").next_token().unwrap(),
        Token::new(
            TokenKind::IntegerLiteral(IntegerLiteralKind::Decimal, 123),
            TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                end: TextPosition {
                    absolute: 3,
                    line: 0,
                    column: 3,
                },
            }
        )
    )
}

#[test]
fn decimal_integer_literal_with_underscore() {
    assert_eq!(
        Lexer::new("12_3").next_token().unwrap(),
        Token::new(
            TokenKind::IntegerLiteral(IntegerLiteralKind::Decimal, 123),
            TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                end: TextPosition {
                    absolute: 4,
                    line: 0,
                    column: 4,
                },
            }
        )
    )
}

#[test]
fn single_char_integer_literal() {
    assert_eq!(
        Lexer::new("1").next_token().unwrap(),
        Token::new(
            TokenKind::IntegerLiteral(IntegerLiteralKind::Decimal, 1),
            TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                end: TextPosition {
                    absolute: 1,
                    line: 0,
                    column: 1,
                },
            },
        )
    )
}

#[test]
fn keyword() {
    assert_eq!(
        Lexer::new("var").next_token(),
        LexResult::Ok(Token::keyword(
            Keyword::Var,
            TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0
                },
                end: TextPosition {
                    absolute: 3,
                    line: 0,
                    column: 3,
                },
            }
        ))
    )
}

#[test]
fn identifier() {
    assert_eq!(
        Lexer::new("foo").next_token().unwrap(),
        Token::new(
            TokenKind::Identifier(String::from("foo")),
            TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                end: TextPosition {
                    absolute: 3,
                    line: 0,
                    column: 3,
                },
            },
        )
    )
}

#[test]
fn single_char_whitespace() {
    assert_eq!(
        Lexer::new(" ").next_token(),
        LexResult::Ok(Token::new(
            TokenKind::Whitespace(0),
            TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                end: TextPosition {
                    absolute: 1,
                    line: 0,
                    column: 1,
                },
            }
        ))
    )
}

#[test]
fn string_literal() {
    assert_eq!(
        Lexer::new("\"foo\"").next_token(),
        LexResult::Ok(Token::new(
            TokenKind::StringLiteral(String::from("foo")),
            TokenPosition::new(
                TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                TextPosition {
                    absolute: 4,
                    line: 0,
                    column: 4,
                },
            )
        ))
    )
}

// TODO: Should an empty string literal be valid syntax?
#[test]
fn empty_string_literal() {
    assert_eq!(
        Lexer::new("\"\"").next_token().unwrap(),
        Token::new(
            TokenKind::StringLiteral(String::from("")),
            TokenPosition {
                start: TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0
                },
                end: TextPosition {
                    absolute: 1,
                    line: 0,
                    column: 1
                },
            },
        )
    )
}

#[test]
fn keyword_then_identifier() {
    let mut lexer = Lexer::new("var foo");
    let mut tokens = Vec::new();

    loop {
        match lexer.next_token() {
            Err(_) => break,
            Ok(token) => tokens.push(token),
        }
    }

    assert_eq!(tokens.len(), 3);

    assert_eq!(
        tokens[0],
        Token::new(
            TokenKind::Keyword(Keyword::Var),
            TokenPosition::new(
                TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0
                },
                TextPosition {
                    absolute: 3,
                    line: 0,
                    column: 3,
                },
            )
        )
    );

    assert_eq!(
        tokens[1],
        Token::new(
            TokenKind::Whitespace(0),
            TokenPosition::new(
                TextPosition {
                    absolute: 3,
                    line: 0,
                    column: 3
                },
                TextPosition {
                    absolute: 4,
                    line: 0,
                    column: 4,
                },
            )
        )
    );

    assert_eq!(
        tokens[2],
        Token::new(
            TokenKind::Identifier(String::from("foo")),
            TokenPosition::new(
                TextPosition {
                    absolute: 4,
                    line: 0,
                    column: 4,
                },
                TextPosition {
                    absolute: 7,
                    line: 0,
                    column: 7,
                }
            )
        )
    )
}

#[test]
fn single_character_primitive_token() {
    assert_eq!(
        Lexer::new("[").next_token().unwrap(),
        Token::new(
            TokenKind::LeftBracket,
            TokenPosition::new(
                TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                TextPosition {
                    absolute: 1,
                    line: 0,
                    column: 1,
                },
            ),
        )
    )
}

#[test]
fn double_character_primitive_token() {
    assert_eq!(
        Lexer::new("+=").next_token().unwrap(),
        Token::new(
            TokenKind::PlusEqual,
            TokenPosition::new(
                TextPosition {
                    absolute: 0,
                    line: 0,
                    column: 0,
                },
                TextPosition {
                    absolute: 2,
                    line: 0,
                    column: 2,
                },
            )
        )
    )
}

#[test]
fn test_variable_declaration() {
    let mut lexer = Lexer::new("var foo: u8 = 5;");
    let mut tokens = Vec::new();

    let expected_kinds = vec![
        TokenKind::Keyword(Keyword::Var),
        TokenKind::Whitespace(0),
        TokenKind::Identifier(String::from("foo")),
        TokenKind::Colon,
        TokenKind::Whitespace(0),
        TokenKind::Keyword(Keyword::U8),
        TokenKind::Whitespace(0),
        TokenKind::Equal,
        TokenKind::Whitespace(0),
        TokenKind::IntegerLiteral(IntegerLiteralKind::Decimal, 5),
        TokenKind::SemiColon,
    ];

    loop {
        match lexer.next_token() {
            Ok(token) => tokens.push(token),
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    let actual_kinds: Vec<TokenKind> = tokens.iter().map(|item| item.clone().kind).collect();
    assert_eq!(actual_kinds, expected_kinds);
}

#[test]
fn unterminated_string_literal() {
    assert_eq!(
        Lexer::new("\"foo").next_token().unwrap_err().kind,
        LexErrorKind::UnexpectedEOF
    );
}
