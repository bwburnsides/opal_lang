use crate::compiler::OptionResult;

#[derive(Debug, Clone)]
pub struct TextPosition {
    pub absolute: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct TokenPosition {
    pub start: TextPosition,
    pub end: TextPosition,
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
pub enum IntegerLiteralType {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: TokenPosition,
}

impl Token {
    pub fn from_kind(kind: TokenKind) -> Self {
        Self {
            kind,
            position: TokenPosition {
                start: TextPosition { absolute: 0, line: 0, column: 0 },
                end: TextPosition { absolute: 0, line: 0, column: 0 }
            }
        }
    }

    pub fn from_kinds(kinds: Vec<TokenKind>) -> Vec<Self> {
        kinds.iter().map(
            |kind| Self::from_kind(kind.clone())
        ).collect()
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Illegal,
            position: TokenPosition {
                start: TextPosition { absolute: 0, line: 0, column: 0 },
                end: TextPosition { absolute: 0, line: 0, column: 0 }
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
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
    Whitespace(bool),
    EOF,
    Illegal,
}

#[derive(Debug)]
pub struct LexerError {
    msg: String,
    line: usize,
    column: usize,
}

pub type LexerResult<T> = OptionResult<T, LexerError>;

pub struct Lexer {
    input: String,
    position: TextPosition,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: TextPosition {
                absolute: 0,
                line: 0,
                column: 0
            },
        }
    }

    pub fn next_token(&mut self) -> LexerResult<Token> {
        if self.current().is_none() {
            return LexerResult::None;
        }

        match self.next_whitespace() {
            LexerResult::Some(token) => return LexerResult::Some(token),
            LexerResult::Err(error) => return LexerResult::Err(error),
            LexerResult::None => (),
        }

        match self.next_integer_literal() {
            LexerResult::Some(token) => return LexerResult::Some(token),
            LexerResult::Err(error) => return LexerResult::Err(error),
            LexerResult::None => (),
        }

        match self.next_char_literal() {
            LexerResult::Some(token) => return LexerResult::Some(token),
            LexerResult::Err(error) => return LexerResult::Err(error),
            LexerResult::None => (),
        }

        match self.next_string_literal() {
            LexerResult::Some(token) => return LexerResult::Some(token),
            LexerResult::Err(error) => return LexerResult::Err(error),
            LexerResult::None => (),
        }

        match self.next_keyword() {
            LexerResult::Some(token) => return LexerResult::Some(token),
            LexerResult::Err(error) => return LexerResult::Err(error),
            LexerResult::None => (),
        }

        match self.next_identifier() {
            LexerResult::Some(token) => return LexerResult::Some(token),
            LexerResult::Err(error) => return LexerResult::Err(error),
            LexerResult::None => (),
        }

        LexerResult::Err(
            LexerError {
                msg: format!(
                    "Unrecognized character '{}'",
                    self.current().unwrap(),
                ),
                line: self.position.line,
                column: self.position.column,
            }
        )
    }

    fn current(&self) -> Option<char> {
        self.input.chars().nth(self.position.absolute)
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
            },
        }
    }

    fn advance_by(&mut self, by: usize) {
        for _ in 0..by {
            self.advance();
        }
    }

    fn advance_while(&mut self, predicate: fn(char) -> bool) {
        loop {
            match self.current() {
                None => return,
                Some(char) => if predicate(char) {
                    self.advance();
                }
            };
        }
    }

    fn next_whitespace(&mut self) -> LexerResult<Token> {
        todo!()
    }

    fn next_integer_literal(&mut self) -> LexerResult<Token> {
        todo!()
    }

    fn next_char_literal(&mut self) -> LexerResult<Token> {
        todo!()
    }

    fn next_string_literal(&mut self) -> LexerResult<Token> {
        todo!()
    }

    fn next_keyword(&mut self) -> LexerResult<Token> {
        todo!()
    }

    fn next_identifier(&mut self) -> LexerResult<Token> {
        todo!()
    }
}
