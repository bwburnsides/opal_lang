#include <stdlib.h>
#include "visitor.h"

void* visit(ExprVisitor* self, Expr* expr) {
    return expr->accept(expr, self);
}

void* visit_binary_expr(ExprVisitor* self, BinaryExpr* expr) {
    return self->klass.binary_expr(self, expr);
}

void* visit_grouping_expr(ExprVisitor* self, GroupingExpr* expr) {
    return self->klass.grouping_expr(self, expr);
}

void* visit_literal_expr(ExprVisitor* self, LiteralExpr* expr) {
    return self->klass.literal_expr(self, expr);
}

void* visit_unary_expr(ExprVisitor* self, UnaryExpr* expr) {
    return self->klass.unary_expr(self, expr);
}
