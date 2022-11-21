#ifndef _VISITOR_H
#define _VISITOR_H

#include "expression.h"

typedef struct Expr_t Expr;
typedef struct BinaryExpr_t BinaryExpr;
typedef struct GroupingExpr_t GroupingExpr;
typedef struct LiteralExpr_t LiteralExpr;
typedef struct UnaryExpr_t UnaryExpr;

typedef struct ExprVisitor_t ExprVisitor;

typedef struct ExprVisitorClass_t {
    void* (*visit)(ExprVisitor* self, Expr* expr);
    void* (*binary_expr)(ExprVisitor* self, BinaryExpr* expr);
    void* (*grouping_expr)(ExprVisitor* self, GroupingExpr* expr);
    void* (*literal_expr)(ExprVisitor* self, LiteralExpr* expr);
    void* (*unary_expr)(ExprVisitor* self, UnaryExpr* expr);
} ExprVisitorClass;

typedef struct ExprVisitor_t {
    ExprVisitorClass klass;
} ExprVisitor;

void* visit(ExprVisitor* self, Expr* expr);
void* visit_binary_expr(ExprVisitor* self, BinaryExpr* expr);
void* visit_grouping_expr(ExprVisitor* self, GroupingExpr* expr);
void* visit_literal_expr(ExprVisitor* self, LiteralExpr* expr);
void* visit_unary_expr(ExprVisitor* self, UnaryExpr* expr);

#endif