pub mod lexer;
pub mod parser;

pub use lexer::*;
pub use parser::*;

#[derive(Debug)]
pub enum CompilerError {
    LexerError(lexer::LexerError),
    ParserError(parser::ParserError),
}

impl Into<()> for CompilerError {
    fn into(self) -> () {}
}

impl From<lexer::LexerError> for CompilerError {
    fn from(error: lexer::LexerError) -> Self {
        CompilerError::LexerError(error)
    }
}

impl From<parser::ParserError> for CompilerError {
    fn from(error: parser::ParserError) -> Self {
        CompilerError::ParserError(error)
    }
}

pub type CompilerResult<T> = Result<T, CompilerError>;

pub enum OptionResult<T, E> {
    None,
    Some(T),
    Err(E),
}

impl<T, E> OptionResult<T, E> {
    pub fn or_else<F>(self, f: F) -> OptionResult<T, E>
    where
        F: FnOnce() -> OptionResult<T, E>
    {
        match self {
            OptionResult::None => f(),
            OptionResult::Err(_) => f(),
            OptionResult::Some(_) => self,
        }
    }
}

pub fn tokenize() -> CompilerResult<Vec<Token>> {
    let mut lexer = Lexer::new(String::from("var foo: u8 = 69;"));
    let mut result: Vec<Token> = Vec::new();

    loop {
        match lexer.next_token() {
            OptionResult::None => break,
            OptionResult::Some(token) => result.push(token),
            OptionResult::Err(error) => return Err(error.into()),
        }
    }

    Ok(result)
}

pub fn parse(tokens: Vec<Token>) -> CompilerResult<Vec<Statement>> {
    let mut parser = Parser::new(tokens);
    let mut result: Vec<Statement> = Vec::new();

    loop {
        match parser.parse_statement() {
            OptionResult::None => break,
            OptionResult::Some((statement, _)) => result.push(statement),
            OptionResult::Err(error) => return Err(error.into()),
        }
    }

    Ok(result)
}
