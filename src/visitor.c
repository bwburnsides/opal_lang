#include <stdlib.h>
#include "visitor.h"
#include <stdio.h>

void* visit(ExprVisitor* self, Expr* expr) {
    return expr->cls->accept(expr, self);
}

void* visit_binary_expr(ExprVisitor* self, BinaryExpr* expr) {
    return self->cls->binary_expr(self, expr);
}

void* visit_grouping_expr(ExprVisitor* self, GroupingExpr* expr) {
    return self->cls->grouping_expr(self, expr);
}

void* visit_literal_expr(ExprVisitor* self, LiteralExpr* expr) {
    return self->cls->literal_expr(self, expr);
}

void* visit_identifier_expr(ExprVisitor* self, IdentifierExpr* expr) {
    return self->cls->identifier_expr(self, expr);
}

void* visit_unary_expr(ExprVisitor* self, UnaryExpr* expr) {
    return self->cls->unary_expr(self, expr);
}

void* visit_call_expr(ExprVisitor* self, CallExpr* expr) {
    return self->cls->call_expr(self, expr);
}