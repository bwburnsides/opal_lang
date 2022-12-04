#include <stdlib.h>
#include "token.h"
#include "expression.h"
#include <stdio.h>

void* expr_accept(Expr* self, ExprVisitor* visitor) {
    return self->cls->accept(self, visitor);
}

void expr_free(Expr* self) {
    self->cls->free(self);
}

// -----------------------------------------

ExprClass BinaryExprClass = {
    BinaryExprKind,
    &binaryexpr_accept,
    &binaryexpr_free,
};

BinaryExpr* binaryexpr_init(Expr* left, Token* operator, Expr* right) {
    BinaryExpr* expr = malloc(sizeof(BinaryExpr));
    if (expr != NULL) {
        expr->cls = &BinaryExprClass;
        expr->left = left;
        expr->operator = operator;
        expr->right = right;
    }

    return expr;
}

void* binaryexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->cls->binary_expr(visitor, (BinaryExpr*) self);
}

void binaryexpr_free(Expr* self) {
    BinaryExpr* expr = (BinaryExpr*) self;

    expr_free(expr->left);
    token_free(expr->operator);
    expr_free(expr->right);
    free(expr);
}

// -----------------------------------------

ExprClass GroupingExprClass = {
    GroupingExprKind,
    &groupingexpr_accept,
    &groupingexpr_free,
};

GroupingExpr* groupingexpr_init(Expr* expr) {
    GroupingExpr* gexpr = malloc(sizeof(GroupingExpr));
    if (gexpr != NULL) {
        gexpr->cls = &GroupingExprClass;
        gexpr->expr = expr;
    }

    return gexpr;
}

void* groupingexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->cls->grouping_expr(visitor, (GroupingExpr*) self);
}

void groupingexpr_free(Expr* self) {
    GroupingExpr* expr = (GroupingExpr*) self;

    expr_free(expr->expr);
    free(expr);
}

// -----------------------------------------

ExprClass LiteralExprClass = {
    LiteralExprKind,
    &literalexpr_accept,
    &literalexpr_free,
};

LiteralExpr* literalexpr_init(Token* value) {
    LiteralExpr* expr = malloc(sizeof(LiteralExpr));
    if (expr != NULL) {
        expr->cls = &LiteralExprClass;
        expr->value = value;
    }

    return expr;
}

void* literalexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->cls->literal_expr(visitor, (LiteralExpr*) self);
}

void literalexpr_free(Expr* self) {
    LiteralExpr* expr = (LiteralExpr*) self;

    token_free(expr->value);
    free(expr);
}

// -----------------------------------------

ExprClass IdentifierExprClass = {
    IdentifierExprKind,
    &identifierexpr_accept,
    &identifierexpr_free,
};

IdentifierExpr* identifierexpr_init(Token* value) {
    IdentifierExpr* expr = malloc(sizeof(IdentifierExpr));
    if (expr != NULL) {
        expr->cls = &IdentifierExprClass;
        expr->value = value;
    }

    return expr;
}

void* identifierexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->cls->identifier_expr(visitor, (IdentifierExpr*) self);
}

void identifierexpr_free(Expr* self) {
    IdentifierExpr* expr = (IdentifierExpr*) self;

    token_free(expr->value);
    free(expr);
}
// -----------------------------------------

ExprClass UnaryExprClass = {
    UnaryExprKind,
    &unaryexpr_accept,
    &unaryexpr_free,
};

UnaryExpr* unaryexpr_init(Token* operator, Expr* right) {
    UnaryExpr* expr = malloc(sizeof(UnaryExpr));
    if (expr != NULL) {
        expr->cls = &UnaryExprClass;
        expr->operator = operator;
        expr->right = right;
    }

    return expr;
}

void* unaryexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->cls->unary_expr(visitor, (UnaryExpr*) self);
}

void unaryexpr_free(Expr* self) {
    UnaryExpr* expr = (UnaryExpr*) self;

    token_free(expr->operator);
    expr_free(expr->right);
    free(expr);
}
// -----------------------------------------

ExprClass CallExprClass = {
    CallExprKind,
    &callexpr_accept,
    &callexpr_free,
};

CallExpr* callexpr_init(
    Expr* callee,
    Token* paren,
    Expr** arguments,
    size_t arg_count
) {
    CallExpr* expr = malloc(sizeof(CallExpr));
    if (expr != NULL) {
        expr->cls = &CallExprClass;
        expr->callee = callee;
        expr->paren = paren;
        expr->arguments = arguments;
        expr->arg_count = arg_count;
    }

    return expr;
}

void* callexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->cls->call_expr(visitor, (CallExpr*) self);
}

void callexpr_free(Expr* self) {
    CallExpr* expr = (CallExpr*) self;

    expr_free(expr->callee);
    token_free(expr->paren);

    for (int idx = 0; expr->arguments[idx] != NULL; idx++) {
        expr_free(expr->arguments[idx]);
    }
    free(expr->arguments);
}