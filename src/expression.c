#include <stdlib.h>
#include "token.h"
#include "expression.h"

BinaryExpr* binaryexpr_init(Expr* left, Token* operator, Expr* right) {
    BinaryExpr* expr = malloc(sizeof(BinaryExpr));
    if (expr != NULL) {
        expr->base.kind = BinaryExprKind;

        expr->left = left;
        expr->operator = operator;
        expr->right = right;
    }

    return expr;
}

GroupingExpr* groupingexpr_init(Expr* expr) {
    GroupingExpr* gexpr = malloc(sizeof(GroupingExpr));
    if (gexpr != NULL) {
        gexpr->base.kind = GroupingExprKind;

        gexpr->expr = expr;
    }

    return gexpr;
}

LiteralExpr* literalexpr_init(Token* value) {
    LiteralExpr* expr = malloc(sizeof(LiteralExpr));
    if (expr != NULL) {
        expr->base.kind = LiteralExprKind;

        expr->value = value;
    }

    return expr;
}

UnaryExpr* unaryexpr_init(Token* operator, Expr* right) {
    UnaryExpr* expr = malloc(sizeof(UnaryExpr));
    if (expr != NULL) {
        expr->base.kind = UnaryExprKind;

        expr->operator = operator;
        expr->right = right;
    }

    return expr;
}
