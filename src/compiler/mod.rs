pub mod lexer;
pub mod parser;

pub use lexer::*;
pub use parser::*;

#[derive(Debug)]
pub enum CompilerError {
    LexError(lexer::LexError),
    ParseError(parser::ParseError),
}

impl Into<()> for CompilerError {
    fn into(self) -> () {}
}

impl From<lexer::LexError> for CompilerError {
    fn from(error: lexer::LexError) -> Self {
        CompilerError::LexError(error)
    }
}

impl From<parser::ParseError> for CompilerError {
    fn from(error: parser::ParseError) -> Self {
        CompilerError::ParseError(error)
    }
}

pub type CompilerResult<T> = Result<T, (CompilerError, T)>;

pub fn tokenize() -> CompilerResult<Vec<Token>> {
    let mut lexer = Lexer::new("var foo: u8 = 69;");
    let mut result = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(token) => result.push(token),
            Err(error) => if let LexErrorKind::EOF = error.kind {
                return Ok(result)
            } else {
                return Err((CompilerError::from(error), result))
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> CompilerResult<Vec<Statement>> {
    let mut parser = Parser::new(tokens);
    let mut result = Vec::new();

    loop {
        match parser.parse_declaration() {
            ParseResult::Ok((statement, _)) => result.push(statement),
            ParseResult::Err(err) => match err {
                ParseError {
                    kind: ParseErrorKind::UnexpectedToken, msg: _, line: _, column: _,
                } => break CompilerResult::Err((err.into(), result)),
                ParseError {
                    kind: ParseErrorKind::UnexpectedEOF, msg: _, line: _, column: _,
                } => match parser.is_exhausted() {
                    true => break CompilerResult::Ok(result),
                    false => break CompilerResult::Err((err.into(), result)),
                },
            },
        }
    }
}
