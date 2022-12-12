use crate::compiler::OptionResult;

use super::{Token, Keyword, TokenKind, TokenPosition, IntegerLiteralType};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
}

#[derive(Debug)]
pub struct ConstantVariableDeclaration {
    is_var: bool,
    identifier: String,
    const_type: Type,
    value: Expression,
}

#[derive(Debug)]
pub struct FunctionDeclaration;

#[derive(Debug)]
pub struct StructUnionDeclaration {
    is_union: bool,
    identifier: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
pub enum Statement {
    ConstantDeclaration(ConstantVariableDeclaration),
    VariableDeclaration(ConstantVariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    StructDeclaration(StructUnionDeclaration),
    UnionDeclaration(StructUnionDeclaration),
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
    IntegerLiteral(IntegerLiteralType, i16),
}

#[derive(Debug)]
pub struct Field {
    identifier: String,
    field_type: Type,
}

pub type ParserResult<T> = OptionResult<(T, TokenPosition), ParserError>;

struct TokenInput {
    tokens: Vec<Token>,
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
        self.index = self.stack.pop().expect("Position stack is unexpectedly empty.");
    }

    pub fn drop(&mut self) {
        self.stack.pop().expect("Position stack is unexpectedly empty.");
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
}

pub struct Parser {
    input: TokenInput,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {input: TokenInput::new(tokens)}
    }

    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        // match self.parse_const_or_var_decl(false) {
        //     ParserResult::None => (),
        //     ParserResult::Err(error) => return ParserResult::Err(error),
        //     ParserResult::Some((decl, pos)) => {
        //         return ParserResult::Some((Statement::ConstantDeclaration(decl), pos))
        //     },
        // }

        match self.parse_const_or_var_decl(true) {
            ParserResult::None => (),
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((decl, pos)) => {
                return ParserResult::Some((Statement::VariableDeclaration(decl), pos))
            },
        }

        match self.parse_fn_decl() {
            ParserResult::None => (),
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((decl, pos)) => {
                return ParserResult::Some((Statement::FunctionDeclaration(decl), pos))
            }
        }

        match self.parse_struct_or_union_decl(false) {
            ParserResult::None => (),
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((decl, pos)) => {
                return ParserResult::Some((Statement::StructDeclaration(decl), pos))
            }
        }

        match self.parse_struct_or_union_decl(true) {
            ParserResult::None => (),
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((decl, pos)) => {
                return ParserResult::Some((Statement::UnionDeclaration(decl), pos))
            }
        }

        ParserResult::None
    }

    fn parse_const_or_var_decl(&mut self, is_var: bool) -> ParserResult<ConstantVariableDeclaration> {
        // "const" / "var" Identifier ":" Type "=" Expression ";"
        // keyword Field "=" Expression ";"

        let keyword = if is_var {
            Keyword::Var
        } else {
            Keyword::Const
        };

        // "const" / "var"
        match self.parse_keyword(keyword) {
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some(_) => (),
        }

        // Identifier ":" Type
        let field = match self.parse_field() {
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((field, _)) => field,
        };

        // "="
        match self.parse_token(TokenKind::Equal) {
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some(_) => (),
        }

        // Expression
        let expression = match self.parse_expression() {
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((expression, _)) => expression,
        };

        // ";"
        let position = match self.parse_token(TokenKind::SemiColon) {
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((_, remaining)) => remaining,
        };

        ParserResult::Some(
            (
                ConstantVariableDeclaration {
                    is_var,
                    identifier: field.identifier,
                    const_type: field.field_type,
                    value: expression,
                },
                position,
            )
        )
    }

    fn parse_fn_decl(&mut self) -> ParserResult<FunctionDeclaration> {
        ParserResult::None
    }

    fn parse_struct_or_union_decl(&mut self, is_union: bool) -> ParserResult<StructUnionDeclaration> {
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
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((identifier, _)) => identifier,
        };

        // "{"
        match self.parse_token(TokenKind::LeftBrace) {
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some(_) => (),
        }

        // ( Field ";" )* 
        loop {
            match self.parse_field() {
                ParserResult::None => break,
                ParserResult::Err(error) => return ParserResult::Err(error),
                ParserResult::Some((field, _)) => fields.push(field), 
            }

            match self.parse_token(TokenKind::SemiColon) {
                ParserResult::None => return ParserResult::Err(ParserError::UnexpectedToken),
                ParserResult::Err(error) => return ParserResult::Err(error),
                ParserResult::Some(_) => (),
            }
        }

        // "}"
        match self.parse_token(TokenKind::RightBrace) {
            ParserResult::None => ParserResult::None,
            ParserResult::Err(error) => ParserResult::Err(error),
            ParserResult::Some((_, remaining)) => ParserResult::Some(
                (StructUnionDeclaration {is_union, identifier, fields}, remaining)
            ),
        }
    }

    fn parse_expression(&mut self) -> ParserResult<Expression> {
        match self.parse_token(TokenKind::IntegerLiteral(IntegerLiteralType::Decimal, 5)) {
            ParserResult::None => ParserResult::None,
            ParserResult::Some((expr, position)) => ParserResult::Some(
                (Expression::IntegerLiteral(IntegerLiteralType::Decimal, 5), position)
            ),
            ParserResult::Err(error) => ParserResult::Err(error),
        }
    }

    fn parse_type(&mut self) -> ParserResult<Type> {
        let primitives = vec![
            (Keyword::U8, Type::U8),
            (Keyword::I8, Type::I8),
            (Keyword::U16, Type::U16),
            (Keyword::I16, Type::I16),
        ];

        for (primitive, kind) in primitives {
            match self.parse_keyword(primitive) {
                ParserResult::None => continue,
                ParserResult::Err(error) => return ParserResult::Err(error),
                ParserResult::Some((_, position)) => return ParserResult::Some((kind, position))
            }
        }

        ParserResult::None
    }

    fn parse_keyword(&mut self, target: Keyword) -> ParserResult<()> {
        self.input.push();

        if let Some(token) = self.input.next() {
            if let TokenKind::Keyword(ref keyword) = token.kind {
                println!("{:?} {:?}", keyword, token.kind);
                self.input.drop();
                ParserResult::Some(((), token.position))
            } else {
                self.input.pop();
                ParserResult::None
            }
        } else {
            self.input.drop();
            ParserResult::None
        }
    }

    fn parse_token(&mut self, target: TokenKind) -> ParserResult<()> {
        self.input.push();

        if let Some(token) = self.input.next() {
            if target == token.kind {
                self.input.drop();
                ParserResult::Some(((), token.position))
            } else {
                self.input.pop();
                ParserResult::None
            }
        } else {
            self.input.drop();
            ParserResult::None
        }
    }

    fn parse_identifier(&mut self) -> ParserResult<String> {
        self.input.push();

        if let Some(token) = self.input.next() {
            if let TokenKind::Identifier(identifier) = token.kind {
                self.input.drop();
                ParserResult::Some((identifier, token.position))
            } else {
                self.input.pop();
                ParserResult::None
            }
        } else {
            self.input.drop();
            ParserResult::None
        }
    }

    fn parse_whitespace(&mut self) -> ParserResult<bool> {
        self.input.push();

        if let Some(token) = self.input.next() {
            if let TokenKind::Whitespace(has_newline) = token.kind {
                self.input.drop();
                ParserResult::Some((has_newline, token.position))
            } else {
                self.input.pop();
                ParserResult::None
            }
        } else {
            self.input.drop();
            ParserResult::None
        }
    }

    fn parse_keyword_with_identifier(&mut self, target: Keyword) -> ParserResult<String> {
        match self.parse_keyword(target) {
            ParserResult::None => ParserResult::None,
            ParserResult::Err(error) => ParserResult::Err(error),
            ParserResult::Some(_) => self.parse_identifier(),
        }
    }

    fn parse_field(&mut self) -> ParserResult<Field> {
        // Field
        // Identifier ":" Type

        // Identifier
        let identifier = match self.parse_identifier() {
            ParserResult::None => return ParserResult::None,
            ParserResult::Err(error) => return ParserResult::Err(error),
            ParserResult::Some((identifier, _)) => identifier,
        };

        // ":"
        match self.parse_token(TokenKind::Colon) {
            ParserResult::None => ParserResult::None,
            ParserResult::Err(error) => ParserResult::Err(error),
            // Type
            ParserResult::Some(_) => {
                match self.parse_type() {
                    ParserResult::None => ParserResult::None,
                    ParserResult::Err(error) => ParserResult::Err(error),
                    ParserResult::Some((field_type, remaining)) => return ParserResult::Some(
                        (Field { identifier, field_type }, remaining)
                    ),
                }
            }
        }
    }
}
