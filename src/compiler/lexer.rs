#[derive(Debug, Clone, PartialEq, Copy)]
pub struct TextPosition {
    pub absolute: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenPosition {
    pub start: TextPosition,
    pub end: TextPosition,
}

impl TokenPosition {
    pub fn new(start: TextPosition, end: TextPosition) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum IntegerLiteralKind {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: TokenPosition,
}

impl Token {
    pub fn new(kind: TokenKind, position: TokenPosition) -> Self {
        Self { kind, position }
    }

    pub fn from_kind(kind: TokenKind) -> Self {
        Self {
            kind,
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
    }

    pub fn from_kinds(kinds: &Vec<TokenKind>) -> Vec<Self> {
        kinds
            .iter()
            .map(|kind| Self::from_kind(kind.clone()))
            .collect()
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Illegal,
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
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier(String),
    IntegerLiteral(IntegerLiteralKind, u16),
    CharLiteral(char),
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
    Whitespace(usize),
    EOF,
    Illegal,
}

#[derive(Debug, PartialEq)]
pub enum LexErrorKind {
    EOF,
    UnrecognizedCharacter,
}

#[derive(Debug, PartialEq)]
pub struct LexError {
    pub kind: LexErrorKind,
    msg: String,
    position: TextPosition,
}

impl LexError {
    pub fn new(kind: LexErrorKind, position: TextPosition) -> Self {
        LexError {
            kind,
            msg: String::from("Not given"),
            position,
        }
    }

    pub fn with_msg(self, msg: String) -> Self {
        return Self {
            kind: self.kind,
            position: self.position,
            msg,
        };
    }
}

pub type LexResult<T> = Result<T, LexError>;

pub struct Lexer {
    input: Vec<char>,
    position: TextPosition,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: String::from(input).chars().collect(),
            position: TextPosition {
                absolute: 0,
                line: 0,
                column: 0,
            },
        }
    }

    pub fn next_token(&mut self) -> LexResult<Token> {
        if self.current().is_none() {
            return LexResult::Err(LexError::new(LexErrorKind::EOF, self.position));
        }

        match self.next_whitespace() {
            Ok(token) => return Ok(token),
            Err(error) => {
                if let LexErrorKind::EOF = error.kind {
                    return Err(error);
                }
            }
        }

        match self.next_integer_literal() {
            Ok(token) => return Ok(token),
            Err(error) => {
                if let LexErrorKind::EOF = error.kind {
                    return Err(error);
                }
            }
        }

        match self.next_char_literal() {
            Ok(token) => return Ok(token),
            Err(error) => {
                if let LexErrorKind::EOF = error.kind {
                    return Err(error);
                }
            }
        }

        match self.next_string_literal() {
            Ok(token) => return Ok(token),
            Err(error) => {
                if let LexErrorKind::EOF = error.kind {
                    return Err(error);
                }
            }
        }

        match self.next_keyword() {
            Ok(token) => return Ok(token),
            Err(error) => {
                if let LexErrorKind::EOF = error.kind {
                    return Err(error);
                }
            }
        }

        match self.next_identifier() {
            Ok(token) => return Ok(token),
            Err(error) => {
                if let LexErrorKind::EOF = error.kind {
                    return Err(error);
                }
            }
        }

        LexResult::Err(
            LexError::new(LexErrorKind::UnrecognizedCharacter, self.position).with_msg(format!(
                "Unrecognized character '{}'",
                self.current().unwrap()
            )),
        )
    }

    fn current(&self) -> Option<char> {
        match self.input.get(self.position.absolute) {
            Some(char_ref) => Some(*char_ref),
            None => None,
        }
    }

    fn advance(&mut self) {
        match self.current() {
            None => (),
            Some(char) => {
                if char == '\n' {
                    self.position.line += 1;
                    self.position.column = 0;
                }
                self.position.absolute += 1;
            }
        }
    }

    fn advance_by(&mut self, by: usize) {
        for _ in 0..by {
            self.advance();
        }
    }

    fn advance_while(&mut self, predicate: fn(char) -> bool) -> Vec<char> {
        let mut span = Vec::new();

        loop {
            match self.current() {
                None => return span,
                Some(char) => {
                    if predicate(char) {
                        span.push(char);
                        self.advance();
                    }
                }
            };
        }
    }

    fn next_whitespace(&mut self) -> LexResult<Token> {
        let start = self.position.clone();

        let consumed = self.advance_while(|c| -> bool { c.is_ascii_whitespace() });

        match consumed.len() {
            0 => LexResult::Err(LexError::new(
                LexErrorKind::UnrecognizedCharacter,
                self.position,
            )),
            line_break_count => LexResult::Ok(Token::new(
                TokenKind::Whitespace(line_break_count),
                TokenPosition::new(start, self.position),
            )),
        }
    }

    fn next_integer_literal(&mut self) -> LexResult<Token> {
        let start = self.position.clone();

        let consumed = self.advance_while(|c| -> bool { is_opal_decimal_digit(c) });
        let non_underscore: Vec<char> = consumed.into_iter().filter(|c| *c != '_').collect();

        match non_underscore.len() {
            0 => LexResult::Err(LexError::new(
                LexErrorKind::UnrecognizedCharacter,
                self.position,
            )),
            num_digits => {
                let literal: u16 = non_underscore.iter().collect::<String>().parse().unwrap();
                LexResult::Ok(Token::new(
                    TokenKind::IntegerLiteral(IntegerLiteralKind::Decimal, literal),
                    TokenPosition::new(start, self.position),
                ))
            }
        }
    }

    fn next_char_literal(&mut self) -> LexResult<Token> {
        let start = self.position.clone();

        match self.current() {
            None => LexResult::Err(LexError::new(LexErrorKind::EOF, self.position)),
            Some('\'') => {
                self.advance();

                match self.current() {
                    None => LexResult::Err(LexError::new(LexErrorKind::EOF, self.position)),
                    Some('\'') => LexResult::Err(LexError::new(
                        LexErrorKind::UnrecognizedCharacter,
                        self.position,
                    )), // this is temporary. for now not worrying about empty literals or escape sequences.
                    Some(literal) => {
                        self.advance();

                        match self.current() {
                            None => LexResult::Err(LexError::new(LexErrorKind::EOF, self.position)),
                            Some('\'') => LexResult::Ok(Token::new(
                                TokenKind::CharLiteral(literal),
                                TokenPosition::new(start, self.position),
                            )),
                            Some(_) => LexResult::Err(LexError::new(
                                LexErrorKind::UnrecognizedCharacter,
                                self.position,
                            )),
                        }
                    }
                }
            }
            Some(_) => LexResult::Err(LexError::new(
                LexErrorKind::UnrecognizedCharacter,
                self.position,
            )),
        }
    }

    fn next_string_literal(&mut self) -> LexResult<Token> {
        todo!()
    }

    fn next_keyword(&mut self) -> LexResult<Token> {
        todo!()
    }

    fn next_identifier(&mut self) -> LexResult<Token> {
        todo!()
    }
}

fn is_opal_decimal_digit(c: char) -> bool {
    c.is_digit(10) || c == '_'
}
