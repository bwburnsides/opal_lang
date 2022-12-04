#ifndef _PARSER_H
#define _PARSER_H

#include <stdlib.h>
#include <stdbool.h>
#include "token.h"
#include "expression.h"

// TODO: Only public members of this header should be listed in the header. Move struct
// implementations (where necessary) and other declarations into *.c as static functions.

typedef struct Parser_t {
    Token** tokens;
    int current;
} Parser;

typedef enum ParseError_t {
    ParseError_UnexpectedToken,
    ParseError_UnexpectedEOF,
} ParseError;

typedef struct ParseExprResult_t {
    enum {
        ParseExprResult_Match,
        ParseExprResult_NoMatch,
        ParseExprResult_Error
    } kind;

    union {
        ParseError error;
        Expr* match;
    } value;

    char* msg;
} ParseExprResult;

Parser* parser_init(Token** tokens);
void parser_free(Parser* self);

/*
Expression => Equality;
Equality   => Comparison (("!=" | "==") Comparison)*
Comparison => Term ((">" | ">=" | "<" | "<=") Term)*
Term       => Factor (("-" | "+") Factor)*
Factor     => Unary (("*" | "/" | "%") Unary)*
Unary      => ("!" | "-") Unary
            | Call
Call       => Primary ("(" Arguments? ")")*
Primary    => Literal | Identifier | "(" Expression ")"

Arguments  => Expression ("," Expression)*
*/

ParseExprResult parser_expression(Parser* self);
ParseExprResult parser_equality(Parser* self);
ParseExprResult parser_comparison(Parser* self);
ParseExprResult parser_term(Parser* self);
ParseExprResult parser_factor(Parser* self);
ParseExprResult parser_unary(Parser* self);
ParseExprResult parser_call(Parser* self);
ParseExprResult parser_primary(Parser* self);

bool parser_match(Parser* self, size_t count, ...);
bool parser_check(Parser* self, TokenKind kind);
Token* parser_advance(Parser* self);
bool parser_at_end(Parser* self);
Token* parser_peek(Parser* self);
Token* parser_previous(Parser* self);
ParseExprResult parser_consume(Parser* self, TokenKind kind, char* msg);

#endif