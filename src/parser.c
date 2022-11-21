#include <stdlib.h>
#include <stdbool.h>
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

Expr* parser_comparison(Parser* self) {
    return NULL;
}

Expr* parser_equality(Parser* self) {
    Expr* expr = parser_comparison(self);
    Token* operator;
    BinaryExpr* right;

    while (parser_match(self, Token_ExclamationEqual, Token_Equal)) {
        operator = parser_previous(self);
        right = (BinaryExpr*) parser_comparison(self);
        expr = (Expr*) binaryexpr_init(expr, operator, (Expr*)right);
    }
}

bool parser_match(Parser* self, ...) {
    return false;
}
bool parser_check(Parser* self, TokenKind kind) {
    false;
}
Token* parser_advance(Parser* self) {
    return NULL;
}
bool parser_at_end(Parser* self) {
    return false;
}
Token* parser_peek(Parser* self) {
    return NULL;
}
Token* parser_previous(Parser* self) {
    return NULL;
}
