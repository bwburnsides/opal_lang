pub mod parser {
    use crate::lexer::lexer::Token;
    use crate::lexer::lexer::Keyword;
    use crate::lexer::lexer::IntegerLiteralType;

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
        Array(Box<Type>, usize),
        FunctionPointer(Option<Vec<Type>>, Option<Box<Type>>),
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
    pub enum Statement {
        ConstantDeclaration(Identifier, Type, Expression),
        VariableDeclaration(Identifier, Type, Expression),
        If,
        Switch,
        While,
        For,
        Type(Identifier, Type),
        Break,
        Continue,
        Return(Option<Expression>),
        Expression(Expression),
        EOF,
    }

    pub struct Parser {
        tokens: Vec<Token>,
        index: usize,
    }
    impl Parser {
        pub fn new(tokens: Vec<Token>) -> Self {
            Self {
                tokens: tokens,
                index: 0,
            }
        }

        pub fn parse(mut self) -> Vec<Statement> {
            let mut statements = Vec::new();

            while !self.at_end() {
                println!("foo");
                match self.statement() {
                    None => break,
                    Some(statement) => statements.push(statement),
                }
            }

            statements
        }

        fn at_end(&self) -> bool {
            self.index == self.tokens.len()
        }

        fn statement(&mut self) -> Option<Statement> {
            match self.current_token() {
                Token::Keyword(Keyword::Const) => self.const_statement(),
                Token::Keyword(Keyword::If) => self.if_statement(),
                Token::Keyword(Keyword::Switch) => self.switch_statement(),
                Token::Keyword(Keyword::While) => self.while_statement(),
                Token::Keyword(Keyword::For) => self.for_statement(),
                Token::Keyword(Keyword::Type) => self.type_statement(),
                Token::Keyword(Keyword::Break) => self.break_statement(),
                Token::Keyword(Keyword::Continue) => self.continue_statement(),
                Token::Keyword(Keyword::Return) => self.return_statement(),
                _ => None,
            }
        }

        fn current_token(&self) -> Token {
            if self.index < self.tokens.len() {
                return self.tokens[self.index].clone();
            }
            Token::EOF
        }

        fn next_token(&self) -> Token {
            if (self.index + 1) < self.tokens.len() {
                return self.tokens[self.index + 1].clone();
            }
            Token::EOF
        }

        fn const_statement(&mut self) -> Option<Statement> {
            if !(self.current_token() == Token::Keyword(Keyword::Const)) {
                return None;
            }

            self.index += 1;

            let identifier = self.current_token();
            match identifier {
                Token::Identifier(_) => (),
                other => panic!(
                    "Expected identifier while parsing const expression, found {:?}",
                    other
                ),
            }
            self.index += 1;

            match self.current_token() {
                Token::Colon => (),
                other => panic!(
                    "Expected colon while parsing const expression, found {:?}",
                    other
                ),
            }
            self.index += 1;

            let const_type = self.current_token();
            match const_type {
                Token::
            }

            None
        }

        fn if_statement(&mut self) -> Option<Statement> {
            None
        }
        fn switch_statement(&mut self) -> Option<Statement> {
            None
        }
        fn while_statement(&mut self) -> Option<Statement> {
            None
        }
        fn for_statement(&mut self) -> Option<Statement> {
            None
        }
        fn type_statement(&mut self) -> Option<Statement> {
            None
        }
        fn break_statement(&mut self) -> Option<Statement> {
            None
        }
        fn continue_statement(&mut self) -> Option<Statement> {
            None
        }
        fn return_statement(&mut self) -> Option<Statement> {
            None
        }
    }
}