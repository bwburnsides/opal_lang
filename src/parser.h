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

typedef struct ParseResult_t {
    enum {
        ParseResult_Match,
        ParseResult_NoMatch,
        ParseResult_Error
    } kind;

    union {
        ParseError error;
        Token* match;
    } value;
} ParseResult;

Parser* parser_init(Token** tokens);
void parser_free(Parser* self);

Expr* parser_expression(Parser* self);
Expr* parser_equality(Parser* self);
Expr* parser_comparison(Parser* self);
Expr* parser_term(Parser* self);
Expr* parser_factor(Parser* self);
Expr* parser_unary(Parser* self);
Expr* parser_primary(Parser* self);

bool parser_match(Parser* self, size_t count, ...);
bool parser_check(Parser* self, TokenKind kind);
Token* parser_advance(Parser* self);
bool parser_at_end(Parser* self);
Token* parser_peek(Parser* self);
Token* parser_previous(Parser* self);
ParseResult parser_consume(Parser* self, TokenKind kind, char* msg);

#endif