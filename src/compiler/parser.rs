use super::{IntegerLiteralKind, Keyword, Token, TokenKind, TokenPosition};

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    UnexpectedToken,
    UnexpectedEOF,
}

impl ParseError {
    pub fn unexpected_token(parser: &Parser) -> Self {
        Self {
            kind: ParseErrorKind::UnexpectedToken,
            msg: format!(
                "Unexpected token {:?}",
                parser.input.clone().prev().unwrap_or_default()
            ),
            line: parser
                .input
                .clone()
                .prev()
                .unwrap_or_default()
                .position
                .start
                .line,
            column: parser
                .input
                .clone()
                .prev()
                .unwrap_or_default()
                .position
                .start
                .column,
        }
    }

    pub fn from_kind(parser: &Parser, kind: ParseErrorKind) -> Self {
        Self {
            kind,
            msg: format!("Parsing error: {:?}", kind),
            line: parser
                .input
                .clone()
                .prev()
                .unwrap_or_default()
                .position
                .start
                .line,
            column: parser
                .input
                .clone()
                .prev()
                .unwrap_or_default()
                .position
                .start
                .column,
        }
    }
}

#[derive(Debug)]
pub struct ConstantVariableDeclaration {
    is_var: bool,
    identifier: String,
    const_type: Type,
    value: Expression,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    identifier: String,
    parameters: Vec<Field>,
    return_type: Option<Type>,
}

#[derive(Debug)]
pub struct StructUnionDeclaration {
    is_union: bool,
    identifier: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub struct EnumDeclaration {
    identifier: String,
    variants: Vec<String>,
}

#[derive(Debug)]
pub struct TypeDeclaration {
    identifier: String,
    ty: Type,
}

#[derive(Debug)]
pub enum Statement {
    ConstantDeclaration(ConstantVariableDeclaration),
    VariableDeclaration(ConstantVariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    StructDeclaration(StructUnionDeclaration),
    UnionDeclaration(StructUnionDeclaration),
    EnumDeclaration(EnumDeclaration),
    TypeDeclaration(TypeDeclaration),
}

#[derive(Debug)]
pub enum Type {
    U8,
    I8,
    U16,
    I16,
}

#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(IntegerLiteralKind, i16),
}

#[derive(Debug)]
pub struct Field {
    identifier: String,
    field_type: Type,
}

pub type ParseResult<T> = Result<(T, TokenPosition), ParseError>;

#[derive(Clone, Debug)]
pub struct TokenInput {
    // TODO: don't keep public
    pub tokens: Vec<Token>, // TODO: don't keep public
    index: usize,
    stack: Vec<usize>,
}

impl TokenInput {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.stack.push(self.index);
    }

    pub fn pop(&mut self) {
        self.index = self
            .stack
            .pop()
            .expect("Position stack is unexpectedly empty.");
    }

    pub fn drop(&mut self) {
        self.stack
            .pop()
            .expect("Position stack is unexpectedly empty.");
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            let result = self.tokens[self.index].clone();
            self.index += 1;
            Some(result)
        }
    }

    pub fn current(self) -> Option<Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.index].clone())
        }
    }

    pub fn prev(self) -> Option<Token> {
        if self.index <= 1 {
            None
        } else if (self.index - 1) >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.index - 1].clone())
        }
    }
}

pub struct Parser {
    pub input: TokenInput, // TODO: don't keep public
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            input: TokenInput::new(tokens),
        }
    }

    pub fn is_exhausted(&mut self) -> bool {
        self.input.push();
        let next = self.input.next();
        self.input.pop();

        match next {
            Option::None => false,
            Option::Some(_) => true,
        }
    }

    pub fn parse_declaration(&mut self) -> ParseResult<Statement> {
        match self.parse_const_or_var_decl(false) {
            ParseResult::Err(err) => match err {
                ParseError {
                    kind: ParseErrorKind::UnexpectedEOF,
                    msg: _,
                    line: _,
                    column: _,
                } => return ParseResult::Err(err),
                _ => (),
            },
            ParseResult::Ok((decl, pos)) => {
                return ParseResult::Ok((Statement::ConstantDeclaration(decl), pos))
            }
        }

        match self.parse_const_or_var_decl(true) {
            ParseResult::Err(err) => match err {
                ParseError {
                    kind: ParseErrorKind::UnexpectedEOF,
                    msg: _,
                    line: _,
                    column: _,
                } => return ParseResult::Err(err),
                _ => (),
            },
            ParseResult::Ok((decl, pos)) => {
                return ParseResult::Ok((Statement::VariableDeclaration(decl), pos))
            }
        }

        match self.parse_fn_decl() {
            ParseResult::Err(err) => match err {
                ParseError {
                    kind: ParseErrorKind::UnexpectedEOF,
                    msg: _,
                    line: _,
                    column: _,
                } => return ParseResult::Err(err),
                _ => (),
            },
            ParseResult::Ok((decl, pos)) => {
                return ParseResult::Ok((Statement::FunctionDeclaration(decl), pos))
            }
        }

        match self.parse_struct_or_union_decl(false) {
            ParseResult::Err(err) => match err {
                ParseError {
                    kind: ParseErrorKind::UnexpectedEOF,
                    msg: _,
                    line: _,
                    column: _,
                } => return ParseResult::Err(err),
                _ => (),
            },
            ParseResult::Ok((decl, pos)) => {
                return ParseResult::Ok((Statement::StructDeclaration(decl), pos))
            }
        }

        match self.parse_struct_or_union_decl(true) {
            ParseResult::Err(err) => match err {
                ParseError {
                    kind: ParseErrorKind::UnexpectedEOF,
                    msg: _,
                    line: _,
                    column: _,
                } => return ParseResult::Err(err),
                _ => (),
            },
            ParseResult::Ok((decl, pos)) => {
                return ParseResult::Ok((Statement::UnionDeclaration(decl), pos))
            }
        }

        match self.parse_type_decl() {
            ParseResult::Err(err) => match err {
                ParseError {
                    kind: ParseErrorKind::UnexpectedEOF,
                    msg: _,
                    line: _,
                    column: _,
                } => return ParseResult::Err(err),
                _ => (),
            },
            ParseResult::Ok((decl, pos)) => {
                return ParseResult::Ok((Statement::TypeDeclaration(decl), pos))
            }
        }

        ParseResult::Err(ParseError::from_kind(self, ParseErrorKind::UnexpectedToken))
    }

    fn parse_const_or_var_decl(
        &mut self,
        is_var: bool,
    ) -> ParseResult<ConstantVariableDeclaration> {
        // "const" / "var" Identifier ":" Type "=" Expression ";"
        // keyword Field "=" Expression ";"

        let keyword = if is_var { Keyword::Var } else { Keyword::Const };

        // "const" / "var"
        match self.parse_keyword(keyword) {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok(_) => (),
        }

        // Identifier ":" Type
        let field = match self.parse_field() {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok((field, _)) => field,
        };

        // "="
        match self.parse_token(TokenKind::Equal) {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok(_) => (),
        }

        // Expression
        let expression = match self.parse_expression() {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok((expression, _)) => expression,
        };

        // ";"
        let position = match self.parse_token(TokenKind::SemiColon) {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok((_, remaining)) => remaining,
        };

        ParseResult::Ok((
            ConstantVariableDeclaration {
                is_var,
                identifier: field.identifier,
                const_type: field.field_type,
                value: expression,
            },
            position,
        ))
    }

    fn parse_fn_decl(&mut self) -> ParseResult<FunctionDeclaration> {
        ParseResult::Err(ParseError::from_kind(self, ParseErrorKind::UnexpectedToken))
    }

    fn parse_struct_or_union_decl(
        &mut self,
        is_union: bool,
    ) -> ParseResult<StructUnionDeclaration> {
        // "struct" / "union" Identifier "{" ( Field ";" )* "}"
        // keyword Identifir "{" ( Field ";" )* "}"

        let keyword = if is_union {
            Keyword::Union
        } else {
            Keyword::Struct
        };

        let mut fields = Vec::new();

        // keyword Identifier
        let identifier = match self.parse_keyword_with_identifier(keyword) {
            ParseResult::Err(error) => return ParseResult::Err(error),
            ParseResult::Ok((identifier, _)) => identifier,
        };

        // "{"
        match self.parse_token(TokenKind::LeftBrace) {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok(_) => (),
        }

        // ( Field "," )*
        loop {
            // Parse fields until there are no more to parse.
            // parse_field will return ParseResult::Err when it is unable to parse another field:
            //
            match self.parse_field() {
                ParseResult::Err(err) => match err {
                    ParseError {
                        kind: ParseErrorKind::UnexpectedEOF,
                        msg: _,
                        line: _,
                        column: _,
                    } => {
                        return ParseResult::Err(ParseError::from_kind(
                            self,
                            ParseErrorKind::UnexpectedEOF,
                        ))
                    }
                    _ => break,
                },
                ParseResult::Ok((field, _)) => fields.push(field),
            }

            match self.parse_token(TokenKind::Comma) {
                ParseResult::Err(err) => {
                    return ParseResult::Err(ParseError::unexpected_token(self))
                }
                ParseResult::Ok(_) => (),
            }
        }

        // "}"
        match self.parse_token(TokenKind::RightBrace) {
            ParseResult::Err(err) => ParseResult::Err(err),
            ParseResult::Ok((_, remaining)) => ParseResult::Ok((
                StructUnionDeclaration {
                    is_union,
                    identifier,
                    fields,
                },
                remaining,
            )),
        }
    }

    fn parse_type_decl(&mut self) -> ParseResult<TypeDeclaration> {
        // "type" Identifier "=" Type ";"
        // keyword Identifir "=" Type ";"

        // keyword Identifier
        let identifier = match self.parse_keyword_with_identifier(Keyword::Type) {
            ParseResult::Err(error) => return ParseResult::Err(error),
            ParseResult::Ok((identifier, _)) => identifier,
        };

        // "="
        match self.parse_token(TokenKind::Equal) {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok(_) => (),
        }

        // Type
        let ty = match self.parse_type() {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok((ty, _)) => ty,
        };

        // ";"
        let position = match self.parse_token(TokenKind::SemiColon) {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok((_, remaining)) => remaining,
        };

        ParseResult::Ok((TypeDeclaration { identifier, ty }, position))
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        match self.parse_token(TokenKind::IntegerLiteral(IntegerLiteralKind::Decimal, 5)) {
            ParseResult::Err(err) => ParseResult::Err(err),
            ParseResult::Ok((expr, position)) => ParseResult::Ok((
                Expression::IntegerLiteral(IntegerLiteralKind::Decimal, 5),
                position,
            )),
        }
    }

    fn parse_type(&mut self) -> ParseResult<Type> {
        let primitives = vec![
            (Keyword::U8, Type::U8),
            (Keyword::I8, Type::I8),
            (Keyword::U16, Type::U16),
            (Keyword::I16, Type::I16),
        ];

        for (primitive, kind) in primitives {
            match self.parse_keyword(primitive) {
                ParseResult::Err(_) => continue,
                ParseResult::Ok((_, position)) => return ParseResult::Ok((kind, position)),
            }
        }

        ParseResult::Err(ParseError::unexpected_token(self))
    }

    fn parse_keyword_with_identifier(&mut self, target: Keyword) -> ParseResult<String> {
        match self.parse_keyword(target) {
            ParseResult::Err(err) => ParseResult::Err(err),
            ParseResult::Ok(_) => match self.parse_identifier() {
                ParseResult::Err(err) => ParseResult::Err(err),
                ParseResult::Ok(val) => ParseResult::Ok(val),
            },
        }
    }

    fn parse_field(&mut self) -> ParseResult<Field> {
        // Field
        // Identifier ":" Type

        // Identifier
        let identifier = match self.parse_identifier() {
            ParseResult::Err(err) => return ParseResult::Err(err),
            ParseResult::Ok((identifier, _)) => identifier,
        };

        // ":"
        match self.parse_token(TokenKind::Colon) {
            ParseResult::Err(err) => ParseResult::Err(err),
            // Type
            ParseResult::Ok(_) => match self.parse_type() {
                ParseResult::Err(err) => ParseResult::Err(err),
                ParseResult::Ok((field_type, remaining)) => {
                    return ParseResult::Ok((
                        Field {
                            identifier,
                            field_type,
                        },
                        remaining,
                    ))
                }
            },
        }
    }

    fn parse_keyword(&mut self, target: Keyword) -> ParseResult<()> {
        self.input.push();

        if let Some(token) = self.input.next() {
            if let TokenKind::Keyword(keyword) = token.kind {
                if keyword == target {
                    self.input.drop();
                    ParseResult::Ok(((), token.position))
                } else {
                    self.input.pop();
                    ParseResult::Err(ParseError::unexpected_token(self))
                }
            } else {
                self.input.pop();
                ParseResult::Err(ParseError::unexpected_token(self))
            }
        } else {
            self.input.drop();
            ParseResult::Err(ParseError::from_kind(self, ParseErrorKind::UnexpectedEOF))
        }
    }

    fn parse_token(&mut self, target: TokenKind) -> ParseResult<()> {
        self.input.push();

        // Fetch the next token. If it exists:
        if let Some(token) = self.input.next() {
            // If this token matches our target, then return that success.
            if target == token.kind {
                self.input.drop();
                ParseResult::Ok(((), token.position))
            // Otherwise, the fetched token didn't match the target and parsing fails.
            } else {
                self.input.pop();
                ParseResult::Err(ParseError::unexpected_token(self))
            }
        // Otherwise, if there wasn't another token, then we've unexpectedly reached the end of our input.
        } else {
            self.input.drop();
            ParseResult::Err(ParseError::from_kind(self, ParseErrorKind::UnexpectedEOF))
        }
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        self.input.push();

        if let Some(token) = self.input.next() {
            if let TokenKind::Identifier(identifier) = token.kind {
                self.input.drop();
                ParseResult::Ok((identifier, token.position))
            } else {
                self.input.pop();
                ParseResult::Err(ParseError::unexpected_token(self))
            }
        } else {
            self.input.drop();
            ParseResult::Err(ParseError::from_kind(self, ParseErrorKind::UnexpectedEOF))
        }
    }

    fn parse_whitespace(&mut self) -> ParseResult<usize> {
        self.input.push();

        if let Some(token) = self.input.next() {
            if let TokenKind::Whitespace(line_break_count) = token.kind {
                self.input.drop();
                ParseResult::Ok((line_break_count, token.position))
            } else {
                self.input.pop();
                ParseResult::Err(ParseError::unexpected_token(self))
            }
        } else {
            self.input.drop();
            ParseResult::Err(ParseError::from_kind(self, ParseErrorKind::UnexpectedEOF))
        }
    }
}
