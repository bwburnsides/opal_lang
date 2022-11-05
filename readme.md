# Opal Lang

Opal is a simple imperative language targeting the BW8 architecture. It has a static type system with weak type safety. It takes syntax cues from C and Rust.

## Examples

    type char = u8;

    const SOME_CONST: u8 = 2;

    fn some_function(arg: u8): u8 {
        val: u8 = arg + SOME_CONST;
        return val + 1;
    }

    struct SomeStruct {
        field1: s16,
        field2: char*,
        field3: u8[10],
    }

## Grammar

### Identifier
    [a-zA-Z_][a-zA-Z0-9_]*

### IntegerLiteral
      "0x" [0-9a-fA-F_]+
    | "0b" [01_]+
    | [0-9_]+

### CharLiteral
    "'" (     "\b"
            | "\f"
            | "\n"
            | "\r"
            | "\t"
            | "\\"
            | "\'"
            | ASCIIChar
    ) "'"

### StringLiteral
    "\"" (    "\b"
            | "\f"
            | "\n"
            | "\r"
            | "\t"
            | "\\"
            | "\""
            | ASCIIChar
    )* "\""

### Type
      "u8"
    | "i8"
    | "u16"
    | "i16"
    | Identifier
    | Type "*"
    | Type "[" IntegerLiteral "]"
    | "fn" "(" (Type ",")*       ")" (":" Type)?
    | "fn" "(" (Type ",")+ "..." ")" (":" Type)?

### UnaryOperator
      "+"
    | "-"
    | "!"
    | "&"
    | "*"

### BinaryOperator
      "+"
    | "-"
    | "*"
    | "/"
    | "%"
    | "<<"
    | ">>"
    | "&&"
    | "||"
    | "&"
    | "|"
    | "^"
    | "=="
    | "!="
    | "<"
    | "<="
    | ">"
    | ">="

### AssignmentOperator
      "="
    | "+="
    | "-="
    | "*="
    | "/="
    | "%="
    | "<<="
    | ">>="
    | "&="
    | "|="
    | "^="

### Expression
      IntegerLiteral
    | CharLiteral
    | StringLiteral
    | Identifier
    | "sizeof(" Type ")"
    | Identifier "(" ( Expression "," )* ")"
    | Expression "[" Expression "]"
    | Expression "." Identifier
    | "<" Type ">" Expression
    | UnaryOperator Expression
    | Expression BinaryOperator Expression
    | Expression AssignmentOperator Expression
    | "(" Expression ")"

### TypeDeclaration
    "type" Identifier "=" Type ";"

### ConstantDeclaration
    "const" Identifier ":" Type "=" Expression ";"

### VariableDeclaration
      Identifier ":" Type "=" Expression ";"
    | Identifier ":" Type ";"

### AssignmentStatement
    Expression AssignmentOperator Expression ";"

### IfStatement
      "if" Expression "{" Statement* "}"
    | "if" Expression "{" Statement* "}" "else" "{" Statement* "}"

### WhileLoop
    "while" Expression "{" Statement* "}"

### ForLoop
      "for" Identifier ":" Type "=" Expression ":" Expression                "{" Statement* "}"
    | "for" Identifier ":" Type "=" Expression ":" Expression ":" Expression "{" Statement* "}"

### CaseStatement
      "case" Expression ":" Statement*
    | "default"         ":" Statement*

### SwitchStatement
    "switch" Expression "{" CaseStatement* "}"

### BreakStatement
    "break" ";"

### ContinueStatement
    "continue" ";"

### ReturnStatement
      "return" ";"
    | "return" Expression ";"

### Statement
      TypeDeclaration
    | ConstantDeclaration
    | VariableDeclaration
    | AssignmentStatement
    | IfStatement
    | WhileLoop
    | ForLoop
    | SwitchStatement
    | BreakStatement
    | ContinueStatement
    | ReturnStatement

### Field
    Identifier ":" Type

### FunctionPrototype
      "fn" Identifier "(" ( Field "," )*       ")" (":" Type)?
    | "fn" Identifier "(" ( Field "," )+ "..." ")" (":" Type)? 

### FunctionDeclaration
      FunctionPrototype ";"
    | FunctionPrototype "{" Statement* "}"
    | "extern" FunctionPrototype ";"

### StructDeclaration
      "struct" Identifier ";"
    | "struct" Identifier "{" ( Field "," )* "}"

### UnionDeclaration
      "union" Identifier ";"
    | "union" Identifier "{" ( Field "," )* "}"

### EnumDeclaration
      "enum" Identifier ";"
    | "enum" Identifier "{" ( ( Identifier | Identifier "=" Expression ) "," )* "}"

### TopLevelStatement
      TypeDeclaration
    | ConstantDeclaration
    | VariableDeclaration
    | FunctionDeclaration
    | StructDeclaration
    | UnionDeclaration
    | EnumDeclaration

### Program
    TopLevelStatement*
