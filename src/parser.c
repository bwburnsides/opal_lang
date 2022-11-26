#include <stdio.h>
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
    parser->current = 0;
    return parser;
}

void parser_free(Parser* self) {
    free(self);
}

ParseResult parser_expression(Parser* self) {
    return parser_equality(self);
}

ParseResult parser_equality(Parser* self) {
    ParseResult result = parser_comparison(self);
    if (result.kind == ParseResult_Error) {
        return result;
    }
    Expr* expr = result.value.match;
    Token* operator;
    Expr* right;

    while (
        parser_match(self, 2,
            Token_ExclamationEqual,
            Token_Equal
        )
    ) {
        operator = parser_previous(self);
        result = parser_comparison(self);
        if (result.kind == ParseResult_Error) {
            return result;
        }
        right = result.value.match;
        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }

    result.value.match = expr;
    return result;
}

ParseResult parser_comparison(Parser* self) {
    ParseResult result = parser_term(self);
    if (result.kind == ParseResult_Error) {
        return result;
    }

    Expr* expr = result.value.match;
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
        result = parser_term(self);
        if (result.kind == ParseResult_Error) {
            return result;
        }

        right = result.value.match;

        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }

    result.value.match = expr;
    return result;
}

ParseResult parser_term(Parser* self) {
    ParseResult result = parser_factor(self);
    if (result.kind == ParseResult_Error) {
        return result;
    }
    Expr* expr = result.value.match;
    Token* operator;
    Expr* right;

    while (
        parser_match(self, 2,
            Token_Plus,
            Token_Minus
        )
    ) {
        operator = parser_previous(self);
        result = parser_factor(self);
        if (result.kind == ParseResult_Error) {
            return result;
        }
        right = result.value.match;
        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }

    result.value.match = expr;
    return result;
}

ParseResult parser_factor(Parser* self) {
    ParseResult result = parser_unary(self);
    if (result.kind == ParseResult_Error) {
        return result;
    }
    Expr* expr = result.value.match;
    Token* operator;
    Expr* right;

    while (
        parser_match(self, 2,
            Token_FSlash,
            Token_Asterisk
        )
    ) {
        operator = parser_previous(self);
        result = parser_unary(self);
        if (result.kind == ParseResult_Error) {
            return result;
        }
        right = result.value.match;
        expr = (Expr*) binaryexpr_init(expr, operator, right);
    }

    result.value.match = expr;
    return result;
}

ParseResult parser_unary(Parser* self) {
    ParseResult result;
    Token* operator;
    Expr* right;

    if (
        parser_match(self, 2,
            Token_Exclamation,
            Token_Minus
        )
    ) {
        operator = parser_previous(self);
        result = parser_unary(self);
        if (result.kind == ParseResult_Error) {
            return result;
        }
        right = result.value.match;
        result.value.match = (Expr*) unaryexpr_init(operator, right);
        return result;
    }

    return parser_primary(self);
}

ParseResult parser_primary(Parser* self) {
    ParseResult result;
    Expr* expr;

    result.kind = ParseResult_Match;

    if (
        parser_match(self, 3,
            Token_BinIntegerLiteral,
            Token_HexIntegerLiteral,
            Token_DecIntegerLiteral
        )
    ) {
        result.value.match = (Expr*) literalexpr_init(parser_previous(self));
        return result;
    }

    if (parser_match(self, 1, Token_Identifier)) {
        result.value.match = (Expr*) identifierexpr_init(parser_previous(self));
        return result;
    }

    if (parser_match(self, 1, Token_LParen)) {
        result = parser_expression(self);
        if (result.kind == ParseResult_Error) {
            return result;
        }
        expr = result.value.match;

        if (
            parser_consume(
                self,
                Token_RParen,
                "Expect ')' after expression."
            ).kind == ParseResult_Error
        ) {
            return result;
        }
        result.value.match = (Expr*) groupingexpr_init(expr);
        return result;
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
        result.value.match = (Expr*) parser_advance(self);
        return result;
    }

    return result;
}