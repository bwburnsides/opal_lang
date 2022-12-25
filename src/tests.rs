#[cfg(test)]
mod tests {
    mod lexer_tests {
        use crate::compiler::*;

        #[test]
        fn test_from_kind() {
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
        fn test_from_kinds() {
            let kinds = vec![TokenKind::Minus, TokenKind::GreaterThan];
            let tokens = Token::from_kinds(&kinds);

            for (idx, token) in tokens.iter().enumerate() {
                assert_eq!(kinds[idx], tokens[idx].kind,)
            }
        }

        #[test]
        fn test_next_token_when_empty_input() {
            let mut lexer = Lexer::new("");
            assert_eq!(
                lexer.next_token(),
                LexResult::Err(LexError::new(
                    LexErrorKind::EOF,
                    TextPosition {
                        absolute: 0,
                        line: 0,
                        column: 0,
                    }
                ))
            )
        }

        #[test]
        fn test_lex_keyword() {
            assert_eq!(
                Lexer::new("var").next_token(),
                LexResult::Ok(Token::new(
                    TokenKind::Keyword(Keyword::Var),
                    TokenPosition {
                        start: TextPosition {
                            absolute: 0,
                            line: 0,
                            column: 0
                        },
                        end: TextPosition {
                            absolute: 2,
                            line: 0,
                            column: 2
                        }
                    }
                ))
            )
        }
    }
}
