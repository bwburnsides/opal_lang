#include <stdlib.h>
#include <stdbool.h>
#include <stdarg.h>
#include "parser.h"
#include "expression.h"

Parser* parser_init(Token** tokens) {
    Parser* parser = malloc(sizeof(Parser));
    if (parser == NULL) {
        return NULL;
    }
    parser->tokens = tokens;
    return parser;
}

void parser_free(Parser* self) {
    free(self);
}

Expr* parser_expression(Parser* self) {
    return parser_equality(self);
}

Expr* parser_equality(Parser* self) {
    Expr* expr = parser_comparison(self);
    Token* operator;
    Expr* right;

    while (
        parser_match(self, 2,
            Token_ExclamationEqual,
            Token_Equal
        )
    ) {
        operator = parser_previous(self);
        right = parser_comparison(self);
        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }
}

Expr* parser_comparison(Parser* self) {
    Expr* expr = parser_term(self);
    Token* operator;
    Expr* right;

    while (
        parser_match(self, 4,
            Token_GreaterThan,
            Token_GreaterThanEqual,
            Token_LessThan,
            Token_LessThanEqual
        )
    ) {
        operator = parser_previous(self);
        right = parser_term(self);
        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }

    return expr;
}

Expr* parser_term(Parser* self) {
    Expr* expr = parser_factor(self);
    Token* operator;
    Expr* right;

    while (
        parser_match(self, 2,
            Token_Plus,
            Token_Minus
        )
    ) {
        operator = parser_previous(self);
        right = parser_factor(self);
        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }

    return expr;
}

Expr* parser_factor(Parser* self) {
    Expr* expr = parser_unary(self);
    Token* operator;
    Expr* right;

    while (
        parser_match(self, 2,
            Token_FSlash,
            Token_Asterisk
        )
    ) {
        operator = parser_previous(self);
        right = parser_unary(self);
        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }

    return expr;
}

Expr* parser_unary(Parser* self) {
    Token* operator;
    Expr* right;

    if (
        parser_match(self, 2,
            Token_Exclamation,
            Token_Minus
        )
    ) {
        operator = parser_previous(self);
        right = parser_unary(self);
        return (Expr*) unaryexpr_init(operator, right);
    }

    return parser_primary(self);
}

Expr* parser_primary(Parser* self) {
    Expr* expr;

    if (
        parser_match(self, 3,
            Token_BinIntegerLiteral,
            Token_HexIntegerLiteral,
            Token_DecIntegerLiteral
        )
    ) {
        return (Expr*) literalexpr_init(parser_previous(self));
    }

    if (parser_match(self, 1, Token_LParen)) {
        expr = parser_expression(self);
        parser_consume(self, Token_RParen, "Expect ')' after expression.");
        return (Expr*) groupingexpr_init(expr);
    }
}

bool parser_match(Parser* self, size_t count, ...) {
    va_list args;
    bool matched = false;
    TokenKind kind;

    va_start(args, count);
    for (int i = 0; i < count; i++) {
        kind = va_arg(args, TokenKind);

        if (parser_check(self, kind)) {
            parser_advance(self);
            matched = true;
            break;
        }
    }

    va_end(args);
    return matched;
}

bool parser_check(Parser* self, TokenKind kind) {
    if (parser_at_end(self)) {
        return false;
    }
    return parser_peek(self)->kind == kind;
}

Token* parser_advance(Parser* self) {
    if (!parser_at_end(self)) {
        self->current++;
    }
    return parser_previous(self);
}

bool parser_at_end(Parser* self) {
    return parser_peek(self)->kind == Token_EOF;
}

Token* parser_peek(Parser* self) {
    return self->tokens[self->current];
}

Token* parser_previous(Parser* self) {
    return self->tokens[self->current - 1];
}

ParseResult parser_consume(Parser* self, TokenKind kind, char* msg) {
    ParseResult result;
    result.kind = ParseResult_Error;
    result.value.error = ParseError_UnexpectedToken;

    if (parser_check(self, kind)) {
        result.kind = ParseResult_Match;
        result.value.match = parser_advance(self);
        return result;
    }

    return result;
}