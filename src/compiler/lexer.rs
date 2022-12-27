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

    pub fn keyword(key: Keyword, position: TokenPosition) -> Self {
        Self {
            kind: TokenKind::Keyword(key),
            position,
        }
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
    EOF, //
    Illegal,
}

#[derive(Debug, PartialEq)]
pub enum LexErrorKind {
    UnexpectedEOF,
    InputExhausted,
    UnrecognizedCharacter,
    MultipleCharLiteral,
    EmptyCharLiteral,
    EmptyStringLiteral,
    NonconformingLexeme,
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

    pub fn get_position(&self) -> TextPosition {
        return self.position;
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
        use LexErrorKind::*;

        if self.current().is_none() {
            return Err(LexError::new(InputExhausted, self.position));
        }

        match self.next_whitespace() {
            Ok(token) => return Ok(token),
            Err(err) => {
                match err.kind {
                    UnexpectedEOF => return Err(err),
                    UnrecognizedCharacter => return Err(err),
                    _ => (),
                }
            }
        }

        match self.next_integer_literal() {
            Ok(token) => return Ok(token),
            Err(err) => {
                match err.kind {
                    UnexpectedEOF => return Err(err),
                    _ => (),
                }
            }
        }

        match self.next_char_literal() {
            Ok(token) => return Ok(token),
            Err(err) => {
                match err.kind {
                    UnexpectedEOF => return Err(err),
                    UnrecognizedCharacter => return Err(err),
                    MultipleCharLiteral => return Err(err),
                    EmptyCharLiteral => return Err(err),
                    _ => (),
                }
            }
        }

        match self.next_string_literal() {
            Ok(token) => return Ok(token),
            Err(err) => {
                match err.kind {
                    UnexpectedEOF => return Err(err),
                    UnrecognizedCharacter => return Err(err),
                    EmptyStringLiteral => return Err(err),
                    _ => (),
                }
            }
        }

        match self.next_identifier_or_keyword() {
            Ok(token) => return Ok(token),
            Err(err) => {
                match err.kind {
                    UnexpectedEOF => return Err(err),
                    UnrecognizedCharacter => return Err(err),
                    _ => (),
                }
            }
        }

        match self.next_primitive_token() {
            Ok(token) => return Ok(token),
            Err(err) => {
                match err.kind {
                    UnexpectedEOF => return Err(err),
                    UnrecognizedCharacter => return Err(err),
                    _ => (),
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
                self.position.column += 1;
            }
        }
    }

    fn advance_by(&mut self, by: usize) -> String {
        let mut result = String::new();
        let mut count = by;

        while let Some(c) = self.current() {
            if count > 0 {
                self.advance();
                result.push(c);
            } else {
                break;
            }
            count -= 1;
        }

        result
    }

    fn advance_while(&mut self, predicate: fn(char) -> bool) -> Vec<char> {
        let mut result = Vec::new();

        while let Some(c) = self.current() {
            if predicate(c) {
                self.advance();
                result.push(c);
            } else {
                break;
            }
        }

        result
    }

    fn next_whitespace(&mut self) -> LexResult<Token> {
        let start = self.position.clone();

        let consumed = self.advance_while(|c| c.is_ascii_whitespace());

        match consumed.len() {
            0 => LexResult::Err(LexError::new(
                LexErrorKind::NonconformingLexeme,
                self.position,
            )),
            _ => LexResult::Ok(Token::new(
                TokenKind::Whitespace(
                    consumed
                        .into_iter()
                        .filter(|&c| c == '\n')
                        .collect::<String>()
                        .len(),
                ),
                TokenPosition::new(start, self.position),
            )),
        }
    }

    fn next_integer_literal(&mut self) -> LexResult<Token> {
        let start = self.position.clone();

        let consumed = self.advance_while(|c| -> bool { is_opal_decimal_digit(c) });
        let non_underscore: Vec<char> = consumed.into_iter().filter(|&c| c != '_').collect();

        match non_underscore.len() {
            0 => Err(LexError::new(
                LexErrorKind::UnrecognizedCharacter,
                self.position,
            )),
            _ => {
                let literal: u16 = non_underscore.iter().collect::<String>().parse().unwrap();
                Ok(Token::new(
                    TokenKind::IntegerLiteral(IntegerLiteralKind::Decimal, literal),
                    TokenPosition::new(start, self.position),
                ))
            }
        }
    }

    fn next_char_literal(&mut self) -> LexResult<Token> {
        use LexErrorKind::{EmptyCharLiteral, MultipleCharLiteral, UnexpectedEOF, NonconformingLexeme};
        use TokenKind::CharLiteral;

        let start = self.position.clone();
        let mut literal = Vec::new();

        match self.current() {
            None => return Err(LexError::new(UnexpectedEOF, self.position)),
            Some('\'') => loop {
                self.advance();

                match self.current() {
                    None => {
                        return Err(LexError::new(UnexpectedEOF, self.position)
                            .with_msg(String::from("Unexpected EOF when searching for `'`")))
                    }
                    Some('\'') => break,
                    Some(c) => literal.push(c),
                }
            },
            Some(c) => {
                return Err(LexError::new(NonconformingLexeme, self.position)
                    .with_msg(format!("Expected `'`, found {}", c)))
            }
        }

        match literal.len() {
            1 => Ok(Token::new(
                CharLiteral(*literal.get(0).unwrap()),
                TokenPosition::new(start, self.position),
            )),
            0 => {
                self.position = start;
                Err(LexError::new(EmptyCharLiteral, start)
                    .with_msg(format!("Character literal may not be empty.")))
            }
            n => {
                self.position = start;
                Err(LexError::new(MultipleCharLiteral, start).with_msg(format!(
                    "Character literal may only contain 1 character. Found {}",
                    n
                )))
            }
        }
    }

    fn next_string_literal(&mut self) -> LexResult<Token> {
        use LexErrorKind::UnexpectedEOF;

        let start = self.position.clone();
        let mut literal: Vec<char> = Vec::new();

        match self.current() {
            None => Err(LexError::new(UnexpectedEOF, self.position)),
            Some('"') => {
                self.advance();
                loop {
                    match self.current() {
                        None => {
                            return Err(LexError::new(
                                LexErrorKind::UnexpectedEOF,
                                self.position,
                            ))
                        }
                        Some('"') => {
                            let rv = Ok(Token::new(
                                TokenKind::StringLiteral(literal.into_iter().collect::<String>()),
                                TokenPosition::new(start, self.position),
                            ));
                            self.advance();
                            return rv;
                        }
                        Some(c) => {
                            self.advance();
                            literal.push(c);
                        }
                    }
                }
            }
            Some(_) => Err(LexError::new(
                LexErrorKind::NonconformingLexeme,
                self.position,
            )),
        }
    }

    fn next_identifier_or_keyword(&mut self) -> LexResult<Token> {
        use LexErrorKind::UnexpectedEOF;

        let start = self.position.clone();
        let mut token = String::new();

        match self.current() {
            None => return Err(LexError::new(UnexpectedEOF, self.position)),
            Some(c) => {
                if !is_ident_head(c) {
                    self.position = start;
                    return Err(LexError::new(
                        LexErrorKind::NonconformingLexeme,
                        self.position,
                    ));
                } else {
                    token.push(c);
                    self.advance();
                }
            }
        }

        loop {
            match self.current() {
                None => return self.to_identifier_or_keyword(start, token.as_str()),
                Some(c) => {
                    if !is_ident_tail(c) {
                        return self.to_identifier_or_keyword(start, token.as_str());
                    }

                    token.push(c);
                    self.advance();
                }
            }
        }
    }

    fn to_identifier_or_keyword(&self, start: TextPosition, input: &str) -> LexResult<Token> {
        let position = TokenPosition {
            start,
            end: self.position,
        };

        match input {
            "u8" => Ok(Token::keyword(Keyword::U8, position)),
            "i8" => Ok(Token::keyword(Keyword::I8, position)),
            "u16" => Ok(Token::keyword(Keyword::U16, position)),
            "i16" => Ok(Token::keyword(Keyword::I16, position)),
            "fn" => Ok(Token::keyword(Keyword::Fn, position)),
            "sizeof" => Ok(Token::keyword(Keyword::SizeOf, position)),
            "type" => Ok(Token::keyword(Keyword::Type, position)),
            "const" => Ok(Token::keyword(Keyword::Const, position)),
            "var" => Ok(Token::keyword(Keyword::Var, position)),
            "if" => Ok(Token::keyword(Keyword::If, position)),
            "else" => Ok(Token::keyword(Keyword::Else, position)),
            "while" => Ok(Token::keyword(Keyword::While, position)),
            "for" => Ok(Token::keyword(Keyword::For, position)),
            "switch" => Ok(Token::keyword(Keyword::Switch, position)),
            "case" => Ok(Token::keyword(Keyword::Case, position)),
            "default" => Ok(Token::keyword(Keyword::Default, position)),
            "break" => Ok(Token::keyword(Keyword::Break, position)),
            "continue" => Ok(Token::keyword(Keyword::Continue, position)),
            "return" => Ok(Token::keyword(Keyword::Return, position)),
            "extern" => Ok(Token::keyword(Keyword::Extern, position)),
            "struct" => Ok(Token::keyword(Keyword::Struct, position)),
            "union" => Ok(Token::keyword(Keyword::Union, position)),
            "enum" => Ok(Token::keyword(Keyword::Enum, position)),
            ident => Ok(Token::new(
                TokenKind::Identifier(String::from(ident)),
                position,
            )),
        }
    }

    fn next_primitive_token(&mut self) -> LexResult<Token> {
        use LexErrorKind::NonconformingLexeme;

        const PATTERNS: [(&str, TokenKind); 40] = [
            ("<<=", TokenKind::LeftChevronEqual),
            (">>=", TokenKind::RightChevronEqual),
            ("<<", TokenKind::LeftChevron),
            (">>", TokenKind::RightChevron),
            ("+=", TokenKind::PlusEqual),
            ("-=", TokenKind::MinusEqual),
            ("*=", TokenKind::AsteriskEqual),
            ("/=", TokenKind::ForwardSlashEqual),
            ("%=", TokenKind::PercentEqual),
            ("&=", TokenKind::AmpersandEqual),
            ("|=", TokenKind::BarEqual),
            ("^=", TokenKind::CaretEqual),
            ("==", TokenKind::DoubleEqual),
            ("!=", TokenKind::BangEqual),
            ("<=", TokenKind::LessThanEqual),
            (">=", TokenKind::GreaterThanEqual),
            ("&&", TokenKind::DoubleAmpersand),
            ("||", TokenKind::DoubleBar),
            ("+", TokenKind::Plus),
            ("-", TokenKind::Minus),
            ("!", TokenKind::Bang),
            ("&", TokenKind::Ampersand),
            ("|", TokenKind::Bar),
            ("^", TokenKind::Caret),
            ("=", TokenKind::Equal),
            ("<", TokenKind::LessThan),
            (">", TokenKind::GreaterThan),
            ("*", TokenKind::Asterisk),
            ("/", TokenKind::ForwardSlash),
            ("%", TokenKind::Percent),
            (".", TokenKind::Dot),
            ("[", TokenKind::LeftBracket),
            ("]", TokenKind::RightBracket),
            ("(", TokenKind::LeftParenthesis),
            (")", TokenKind::RightParenthesis),
            ("{", TokenKind::LeftBrace),
            ("}", TokenKind::RightBrace),
            (":", TokenKind::Colon),
            (";", TokenKind::SemiColon),
            (",", TokenKind::Comma),
        ];

        let start = self.position.clone();

        for (pattern, token) in PATTERNS.iter() {
            if let Ok(_) = self.next_match(pattern) {
                return Ok(Token::new(
                    token.clone(),
                    TokenPosition::new(start, self.position),
                ));
            }
        }

        Err(LexError::new(NonconformingLexeme, self.position))
    }

    fn next_match(&mut self, pattern: &str) -> LexResult<()> {
        use LexErrorKind::NonconformingLexeme;

        let start = self.position.clone();
        let s = self.advance_by(pattern.chars().count());

        if s.eq(pattern) {
            Ok(())
        } else {
            self.position = start;
            Err(LexError::new(NonconformingLexeme, self.position))
        }
    }
}

fn is_ident_head(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_tail(c: char) -> bool {
    is_ident_head(c) || c.is_ascii_digit()
}

fn is_opal_decimal_digit(c: char) -> bool {
    c.is_digit(10) || c == '_'
}
