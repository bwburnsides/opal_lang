#ifndef _PRINT_VISITOR_H
#define _PRINT_VISITOR_H

#include "visitor.h"

void* _print_visit(ExprVisitor* self, Expr* expr);
void* _print_binary_expr(ExprVisitor* self, BinaryExpr* expr);
void* _print_grouping_expr(ExprVisitor* self, GroupingExpr* expr);
void* _print_literal_expr(ExprVisitor* self, LiteralExpr* expr);
void* _print_unary_expr(ExprVisitor* self, UnaryExpr* expr);

ExprVisitorClass PrintExprVisitorClass;

#endif