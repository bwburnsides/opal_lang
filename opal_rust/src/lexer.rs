pub mod lexer {
    use core::panic;

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum Keyword {
        U8,
        I8,
        U16,
        I16,
        Fn,
        SizeOf,
        Type,
        Const,
        Var,
        If,
        Else,
        While,
        For,
        Switch,
        Case,
        Default,
        Break,
        Continue,
        Return,
        Extern,
        Struct,
        Union,
        Enum,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum IntegerLiteralType {
        Decimal,
        Hexadecimal,
        Binary,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum Token {
        Keyword(Keyword),
        Identifier(String),
        IntegerLiteral(IntegerLiteralType, u16),
        CharLiteral(Option<char>),
        StringLiteral(String),
        LeftBracket,
        RightBracket,
        LeftParenthesis,
        RightParenthesis,
        LeftBrace,
        RightBrace,
        LeftChevron,
        RightChevron,
        Colon,
        SemiColon,
        Comma,
        Plus,
        Minus,
        Bang,
        Ampersand,
        DoubleAmpersand,
        Bar,
        DoubleBar,
        Caret,
        Equal,
        DoubleEqual,
        BangEqual,
        LessThan,
        LessThanEqual,
        GreaterThan,
        GreaterThanEqual,
        PlusEqual,
        MinusEqual,
        AsteriskEqual,
        ForwardSlashEqual,
        PercentEqual,
        LeftChevronEqual,
        RightChevronEqual,
        AmpersandEqual,
        BarEqual,
        CaretEqual,
        Asterisk,
        ForwardSlash,
        Percent,
        Dot,
        EOF,
        Illegal,
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphabetic() || c.is_numeric() || c == '_'
    }

    #[derive(Debug)]
    pub struct Lexer {
        index: usize,
        current: char,
        input: Vec<char>,
    }

    impl Lexer {
        pub fn new(input: Vec<char>) -> Self {
            Self {
                index: 0,
                current: input[0],
                input: input,
            }
        }

        pub fn next_token(&mut self) -> Option<Token> {
            if self.at_end() {
                return None;
            }

            // Consume whitespace
            while self.current_char().is_whitespace() {
                self.index += 1;
            }

            // Match primitive tokens
            if let Some(token) = self.consume_primitive_token() {
                return Some(token);
            }

            // Match complex tokens
            if let Some(token) = self.consume_complex_token() {
                return Some(token);
            }

            None
        }

        fn current_char(&self) -> char {
            if self.index < self.input.len() {
                return self.input[self.index];
            }
            '\0'
        }

        fn next_char(&self) -> char {
            if (self.index + 1) < self.input.len() {
                return self.input[self.index + 1];
            }
            '\0'
        }

        fn multichar_token(&mut self, next: char, then: Token, otherwise: Token) -> Token {
            if self.next_char() == next {
                self.index += 1;
                return then;
            }
            otherwise
        }

        fn at_end(&self) -> bool {
            self.index == self.input.len()
        }

        fn consume_primitive_token(&mut self) -> Option<Token> {
            let token = match self.current_char() {
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                '(' => Token::LeftParenthesis,
                ')' => Token::RightParenthesis,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                ':' => Token::Colon,
                ';' => Token::SemiColon,
                ',' => Token::Comma,
                '.' => Token::Dot,
                '+' => self.multichar_token('=', Token::PlusEqual, Token::Plus),
                '-' => self.multichar_token('=', Token::MinusEqual, Token::Minus),
                '!' => self.multichar_token('=', Token::BangEqual, Token::Bang),
                '&' => self.multichar_token('=', Token::AmpersandEqual, Token::Ampersand),
                '|' => self.multichar_token('=', Token::BarEqual, Token::Bar),
                '^' => self.multichar_token('=', Token::CaretEqual, Token::Caret),
                '=' => self.multichar_token('=', Token::DoubleEqual, Token::Equal),
                '/' => self.multichar_token('=', Token::ForwardSlashEqual, Token::ForwardSlash),
                '%' => self.multichar_token('=', Token::PercentEqual, Token::Percent),

                '<' => {  // <= <<= << <
                    match self.next_char() {
                        '=' => Token::LessThanEqual,
                        '<' => {
                            self.index += 1;
                            if self.next_char() == '=' {
                                self.index += 1;
                                Token::LeftChevronEqual
                            } else {
                                Token::LeftChevron
                            }
                        }
                        _ => Token::LessThan
                    }
                },

                '>' => {  // >= >>= >> >
                    match self.next_char() {
                        '=' => Token::GreaterThanEqual,
                        '>' => {
                            self.index += 1;
                            if self.next_char() == '=' {
                                self.index += 1;
                                Token::RightChevronEqual
                            } else {
                                Token::RightChevron
                            }
                        }
                        _ => Token::GreaterThan
                    }
                },

                _ => Token::Illegal,
            };

            match token {
                Token::Illegal => None,
                _ => {
                    self.index += 1;
                    Some(token)
                }
            }
        }

        fn consume_complex_token(&mut self) -> Option<Token> {
            if let Some(token) = self.consume_identifier() {
                return Some(token);
            }

            if let Some(token) = self.consume_literal() {
                return Some(token);
            }

            None
        }

        fn consume_identifier(&mut self) -> Option<Token> {
            let mut chars = Vec::new();

            if self.current_char().is_alphabetic() {
                chars.push(self.current_char());
                self.index += 1;
            }

            while (chars.len() > 0) && is_identifier_char(self.current_char()) {
                chars.push(self.current_char());
                self.index += 1;
            }

            let result: String = chars.into_iter().collect();

            match result.as_str() {
                "" => None,
                "u8" => Some(Token::Keyword(Keyword::U8)),
                "i8" => Some(Token::Keyword(Keyword::I8)),
                "u16" => Some(Token::Keyword(Keyword::U16)),
                "i16" => Some(Token::Keyword(Keyword::I16)),
                "fn" => Some(Token::Keyword(Keyword::Fn)),
                "sizeof" => Some(Token::Keyword(Keyword::SizeOf)),
                "type" => Some(Token::Keyword(Keyword::Type)),
                "const" => Some(Token::Keyword(Keyword::Const)),
                "var" => Some(Token::Keyword(Keyword::Var)),
                "if" => Some(Token::Keyword(Keyword::If)),
                "else" => Some(Token::Keyword(Keyword::Else)),
                "while" => Some(Token::Keyword(Keyword::While)),
                "for" => Some(Token::Keyword(Keyword::For)),
                "case" => Some(Token::Keyword(Keyword::Case)),
                "default" => Some(Token::Keyword(Keyword::Default)),
                "switch" => Some(Token::Keyword(Keyword::Switch)),
                "break" => Some(Token::Keyword(Keyword::Break)),
                "continue" => Some(Token::Keyword(Keyword::Continue)),
                "return" => Some(Token::Keyword(Keyword::Return)),
                "extern" => Some(Token::Keyword(Keyword::Extern)),
                "struct" => Some(Token::Keyword(Keyword::Struct)),
                "union" => Some(Token::Keyword(Keyword::Union)),
                "enum" => Some(Token::Keyword(Keyword::Enum)),
                _ => Some(Token::Identifier(result))
            }
        }

        fn consume_literal(&mut self) -> Option<Token> {
            match self.current_char() {
                '"' => return self.consume_string_literal(),
                '\'' => return self.consume_char_literal(),
                _ => return self.consume_integer_literal(),
            }
        }

        fn consume_string_literal(&mut self) -> Option<Token> {
            if self.current_char() != '"' {
                return None;
            }

            let mut chars = Vec::new();

            // Advance index to first character in literal
            self.index += 1;

            loop {
                match self.current_char() {
                    '\0' => {
                        panic!("Unexpected EOF when parsing string literal.")
                    }
                    '"' => {
                        self.index += 1;
                        break;
                    }
                    char => {
                        chars.push(char);
                        self.index += 1;
                    }
                }
            }

            return Some(Token::StringLiteral(chars.into_iter().collect()));
        }

        fn consume_char_literal(&mut self) -> Option<Token> {
            if self.current_char() != '\'' {
                return None;
            }

            // Advance index to character literal
            self.index += 1;

            // Could be EOF, or an empty literal, or some other character
            match self.current_char() {
                '\0' => panic!("Unexpected EOF when parsing character literal."),
                '\'' => {
                    self.index += 1;
                    return Some(Token::CharLiteral(None))
                },
                _ => (),
            }

            let result = Some(Token::CharLiteral(Some(self.current_char())));
            self.index += 1;

            if self.current_char() != '\'' {
                panic!("Expected quotation mark when parsing character literal.");
            }
            self.index += 1;
            return result;
        }

        fn consume_integer_literal(&mut self) -> Option<Token> {
            if !(self.current_char().is_numeric()) {
                return None;
            }

            let mut chars = Vec::new();

            loop {
                if self.current_char().is_digit(16) {
                    chars.push(self.current_char());
                    self.index += 1;
                } else {
                    break;
                }
            }

            let string: String = chars.into_iter().collect();
            let result: u16 = string.parse::<u16>().unwrap();
            Some(Token::IntegerLiteral(IntegerLiteralType::Hexadecimal, result))
        }
    }

    impl Iterator for Lexer {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            self.next_token()
        }
    }
}
