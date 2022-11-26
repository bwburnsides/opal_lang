#include <stdlib.h>
#include "token.h"
#include "expression.h"
#include <stdio.h>

void* expr_accept(Expr* self, ExprVisitor* visitor) {
    return self->class->accept(self, visitor);
}

// -----------------------------------------

ExprClass BinaryExprClass = {
    BinaryExprKind,
    &binaryexpr_accept
};

BinaryExpr* binaryexpr_init(Expr* left, Token* operator, Expr* right) {
    BinaryExpr* expr = malloc(sizeof(BinaryExpr));
    if (expr != NULL) {
        expr->class = &BinaryExprClass;
        expr->left = left;
        expr->operator = operator;
        expr->right = right;
    }

    return expr;
}

void* binaryexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->class->binary_expr(visitor, (BinaryExpr*) self);
}

// -----------------------------------------

ExprClass GroupingExprClass = {
    GroupingExprKind,
    &groupingexpr_accept
};

GroupingExpr* groupingexpr_init(Expr* expr) {
    GroupingExpr* gexpr = malloc(sizeof(GroupingExpr));
    if (gexpr != NULL) {
        gexpr->class = &GroupingExprClass;
        gexpr->expr = expr;
    }

    return gexpr;
}

void* groupingexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->class->grouping_expr(visitor, (GroupingExpr*) self);
}

// -----------------------------------------

ExprClass LiteralExprClass = {
    LiteralExprKind,
    &literalexpr_accept
};

LiteralExpr* literalexpr_init(Token* value) {
    LiteralExpr* expr = malloc(sizeof(LiteralExpr));
    if (expr != NULL) {
        expr->class = &LiteralExprClass;
        expr->value = value;
    }

    return expr;
}

void* literalexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->class->literal_expr(visitor, (LiteralExpr*) self);
}

// -----------------------------------------

ExprClass IdentifierExprClass = {
    IdentifierExprKind,
    &identifierexpr_accept
};

IdentifierExpr* identifierexpr_init(Token* value) {
    IdentifierExpr* expr = malloc(sizeof(IdentifierExpr));
    if (expr != NULL) {
        expr->class = &IdentifierExprClass;
        expr->value = value;
    }

    return expr;
}

void* identifierexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->class->identifier_expr(visitor, (IdentifierExpr*) self);
}

// -----------------------------------------

ExprClass UnaryExprClass = {
    UnaryExprKind,
    &unaryexpr_accept
};

UnaryExpr* unaryexpr_init(Token* operator, Expr* right) {
    UnaryExpr* expr = malloc(sizeof(UnaryExpr));
    if (expr != NULL) {
        expr->class = &UnaryExprClass;
        expr->operator = operator;
        expr->right = right;
    }

    return expr;
}

void* unaryexpr_accept(Expr* self, ExprVisitor* visitor) {
    return visitor->class->unary_expr(visitor, (UnaryExpr*) self);
}
