#include <stdlib.h>
#include <stdio.h>
#include "print_visitor.h"

ExprVisitorClass PrintExprVisitorClass = {
    _print_visit,
    _print_binary_expr,
    _print_grouping_expr,
    _print_literal_expr,
    _print_unary_expr,
};

void* _print_visit(ExprVisitor* self, Expr* expr) {
    return NULL;
}

void* _print_binary_expr(ExprVisitor* self, BinaryExpr* expr) {
    return NULL;
}

void* _print_grouping_expr(ExprVisitor* self, GroupingExpr* expr) {
    return NULL;
}

void* _print_literal_expr(ExprVisitor* self, LiteralExpr* expr) {
    return NULL;
}

void* _print_unary_expr(ExprVisitor* self, UnaryExpr* expr) {
    return NULL;
}