pub mod parser {
    use std::collections::VecDeque;

    use crate::lexer::lexer::IntegerLiteralType;
    use crate::lexer::lexer::Token;
    use crate::lexer::lexer::Keyword;

    #[derive(Debug, PartialEq, Clone)]
    pub enum UnaryOperator {
        Plus,
        Minus,
        Dereference,
        LogicalNOT,
        AddressOf,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum BinaryOperator {
        Plus,
        Minus,
        Times,
        Divides,
        Modulo,
        LeftShift,
        RightShift,
        LogicalAND,
        LogicalOR,
        BitwiseAND,
        BitwiseOR,
        BitwiseXOR,
        Equal,
        NotEqual,
        LessThan,
        LessThanEqual,
        GreaterThan,
        GreaterThanEqual,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum AssignmentOperator {
        Equal,
        PlusEqual,
        MinusEqual,
        TimesEqual,
        DividesEqual,
        ModuloEqual,
        ANDEqual,
        OREqual,
        XOREqual,
        LShiftEqual,
        RShiftEqual,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Identifier(String);

    #[derive(Debug, PartialEq, Clone)]
    pub enum Type {
        U8,
        I8,
        U16,
        I16,
        Custom(Identifier),
        Pointer(Box<Type>),
        Array(Box<Type>, u16),
        FunctionPointer(Option<Vec<Box<Type>>>, Option<Box<Type>>),
        VariadicFunctionPointer(Vec<Box<Type>>, Option<Box<Type>>),
    }

    #[derive(Debug)]
    pub enum Expression {
        IntegerLiteral(IntegerLiteralType, i64),
        CharLiteral(char),
        StringLiteral(String),
        Identifier(Identifier),
        Group(Box<Expression>),
        Unary(UnaryOperator, Box<Expression>),
        Binary(Box<Expression>, BinaryOperator, Box<Expression>),
        FunctionCall(Identifier, Vec<Expression>),
        SizeOf(Type),
        ArrayAccess(Box<Expression>, Box<Expression>),
        MemberAccess(Box<Expression>, Identifier),
        Cast(Type, Box<Expression>),
        Assignment(Box<Expression>, AssignmentOperator, Box<Expression>),
    }

    #[derive(Debug)]
    pub struct Field(Identifier, Type);

    #[derive(Debug)]
    pub struct FunctionPrototype {
        identifier: Identifier,
        parameters: Vec<Field>,
        return_type: Option<Type>,
        is_variadic: bool,
    }

    #[derive(Debug)]
    pub enum Statement {
        ConstantDeclaration(Identifier, Type, Expression),
        VariableDeclaration(Identifier, Type, Option<Expression>),
        If,
        Switch,
        While,
        For,
        Type(Identifier, Type),
        Break,
        Continue,
        Return(Option<Expression>),
        Expression(Expression),
        FunctionDeclaration {
            identifier: Identifier,
            parameters: Vec<Field>,
            return_type: Option<Type>,
            is_variadic: bool,
        },
        EOF,
    }

    #[derive(Debug)]
    pub enum ParseError {
        UnexpectedToken(Token),
        ExpectedToken(Token),
        UnexpectedEOF,
        ExpectedEOF,
        ExpectedKeyword(Keyword),
        UnexpectedKeyword(Keyword),
        ExpectedIdentifier,
    }

    pub type ParseResult<T> = Result<T, ParseError>;

    pub fn parse(tokens: Vec<Token>) -> ParseResult<Vec<Statement>> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut deque = VecDeque::from(tokens);

        loop {
            if let None = deque.front() {
                break Ok(statements)
            }

            match parse_statement(&mut deque) {
                Ok(statement) => statements.push(statement),
                Err(error) => break Err(error),
            }
        }
    }

    fn parse_keyword(tokens: &mut VecDeque<Token>, target: Keyword) -> ParseResult<Token> {
        match tokens.pop_front() {
            Some(token) => if let Token::Keyword(keyword) = token {
                if keyword == target {
                    Ok(token)
                } else {
                    Err(ParseError::UnexpectedKeyword(keyword))
                }
            } else {
                Err(ParseError::ExpectedKeyword(target))
            },
            None => Err(ParseError::UnexpectedEOF)
        }
    }

    fn parse_identifier(tokens: &mut VecDeque<Token>) -> ParseResult<Identifier> {
        match tokens.pop_front() {
            Some(token) => if let Token::Identifier(ident) = token {
                Ok(Identifier(ident))
            } else {
                Err(ParseError::ExpectedIdentifier)
            },
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_token(tokens: &mut VecDeque<Token>, target: Token) -> ParseResult<Token> {
        match tokens.pop_front() {
            Some(token) => if target == token {
                Ok(token)
            } else {
                Err(ParseError::ExpectedToken(target))
            },
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_statement(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        match tokens.front() {
            Some(token) => match token {
                Token::Keyword(Keyword::Fn) | Token::Keyword(Keyword::Extern) => parse_function_declaration(tokens),
                Token::Keyword(Keyword::Type) => parse_type_declaration(tokens),
                Token::Keyword(Keyword::Const) => parse_const_declaration(tokens),
                Token::Keyword(Keyword::Var) => parse_var_declaration(tokens),
                Token::Keyword(Keyword::If) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::While) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::For) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::Case) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::Default) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::Switch) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::Break) => parse_break_statement(tokens),
                Token::Keyword(Keyword::Continue) => parse_continue_statement(tokens),
                Token::Keyword(Keyword::Return) => parse_return_statement(tokens),
                Token::Keyword(Keyword::Struct) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::Union) => Err(ParseError::ExpectedEOF),
                Token::Keyword(Keyword::Enum) => Err(ParseError::ExpectedEOF),
                _ => parse_expression_statement(tokens),
            },
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_function_declaration(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        // FunctionDeclaration
        // FunctionPrototype ";" |
        // FunctionPrototype "{" Statement* "}" |

        parse_function_prototype(tokens)?;

        todo!()
    }

    fn parse_type_declaration(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        // TypeDeclaration:
        // "type" Identifier "=" Type ";"

        parse_keyword(tokens, Keyword::Type)?;
        let identifier = parse_identifier(tokens)?;
        parse_token(tokens, Token::Equal)?;
        let parsed_type = parse_type(tokens)?;
        parse_token(tokens, Token::SemiColon)?;

        Ok(Statement::Type(identifier, parsed_type))
    }

    fn parse_const_declaration(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        // ConstDeclaration:
        // "const" Identifier ":" Type "=" Expression ";"

        parse_keyword(tokens, Keyword::Const)?;
        let identifier = parse_identifier(tokens)?;
        parse_token(tokens, Token::Colon)?;
        let parsed_type = parse_type(tokens)?;
        parse_token(tokens, Token::Equal)?;
        let expr = parse_expression(tokens)?;
        parse_token(tokens, Token::SemiColon)?;

        Ok(Statement::ConstantDeclaration(identifier, parsed_type, expr))
    }

    fn parse_var_declaration(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        // VariableDeclaration:
        // "var" Identifier ":" Type "=" Expression ";" |
        // "var" Identifier ":" Type ";"

        parse_keyword(tokens, Keyword::Var)?;
        let identifier = parse_identifier(tokens)?;
        parse_token(tokens, Token::Colon)?;
        let parsed_type = parse_type(tokens)?;

        match tokens.pop_front() {
            Some(Token::Equal) => {
                let expr = parse_expression(tokens)?;
                parse_token(tokens, Token::SemiColon)?;
                Ok(Statement::VariableDeclaration(identifier, parsed_type, Some(expr)))
            },
            Some(Token::SemiColon) => Ok(
                Statement::VariableDeclaration(identifier, parsed_type, None)
            ),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_break_statement(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        // BreakStatement
        // "break" ";"

        parse_keyword(tokens, Keyword::Break)?;
        parse_token(tokens, Token::SemiColon)?;

        Ok(Statement::Break)
    }

    fn parse_continue_statement(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        // ContinueStatement
        // "continue" ";"

        parse_keyword(tokens, Keyword::Continue)?;
        parse_token(tokens, Token::SemiColon)?;

        Ok(Statement::Continue)
    }

    fn parse_return_statement(tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        // ReturnStatement
        // "return" ";" |
        // "return" Expression ";"

        parse_keyword(tokens, Keyword::Return)?;

        match tokens.pop_front() {
            Some(Token::SemiColon) => Ok(Statement::Return(None)),
            Some(_) => {
                let expr = parse_expression(tokens)?;
                parse_token(tokens, Token::SemiColon)?;
                Ok(Statement::Return(Some(expr)))
            },
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_expression_statement(_tokens: &mut VecDeque<Token>) -> ParseResult<Statement> {
        todo!("Implement expression statement parsing")
    }

    fn parse_function_prototype(_tokens: &mut VecDeque<Token>) -> ParseResult<FunctionPrototype> {
        // FunctionPrototype
        // "fn" Identifier "(" ( Field "," )*       ")" (":" Type)? |
        // "fn" Identifier "(" ( Field "," )* "..." ")" (":" Type)?

        // TODO: Add support for variadic function prototypes!
        todo!()
    }

    fn parse_type(tokens: &mut VecDeque<Token>) -> ParseResult<Type> {
        match tokens.pop_front() {
            Some(token) => match token {
                Token::Keyword(keyword) => {
                    match keyword {
                        // TODO: Ignore primitive pointer types and array types for now.
                        Keyword::U8 => Ok(Type::U8),
                        Keyword::I8 => Ok(Type::I8),
                        Keyword::U16 => Ok(Type::U16),
                        Keyword::I16 => Ok(Type::I16),
                        Keyword::Fn => todo!("Support function pointer parsing."),
                        _ => Err(ParseError::UnexpectedKeyword(keyword))
                    }
                }
                Token::Identifier(name) => {
                    // TODO: Ignore primitive pointer types and array types for now.
                    Ok(Type::Custom(Identifier(name)))
                }
                _ => Err(ParseError::UnexpectedToken(token)),
            },
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_expression(tokens: &mut VecDeque<Token>) -> ParseResult<Expression> {
        tokens.pop_front();
        Ok(Expression::IntegerLiteral(IntegerLiteralType::Decimal, 365))
    }
}
