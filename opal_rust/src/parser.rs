pub mod parser {
    #[derive(Debug)]
    enum UnaryOperator {
        Plus,
        Minus,
        Dereference,
        LogicalNOT,
        AddressOf,
    }

    #[derive(Debug)]
    enum BinaryOperator {
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

    #[derive(Debug)]
    enum AssignmentOperator {
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

    #[derive(Debug)]
    struct Identifier(String);

    #[derive(Debug)]
    enum Type {
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
    enum IntegerLiteralBase {
        Decimal,
        Hexadecimal,
        Binary,
    }

    #[derive(Debug)]
    enum Expression {
        IntegerLiteral(IntegerLiteralBase, i64),
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
    enum Statement {
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
    }
}