#ifndef _EXPRESSION_H
#define _EXPRESSION_H

#include "token.h"
#include "visitor.h"

typedef struct ExprVisitor_t ExprVisitor;

typedef enum ExprKind_t {
    BinaryExprKind,
    GroupingExprKind,
    LiteralExprKind,
    UnaryExprKind
} ExprKind;

typedef struct Expr_t Expr;

typedef struct Expr_t {
    ExprKind kind;
    void* (*accept)(Expr* self, ExprVisitor* visitor);
} Expr;

// -----------------------------------------

typedef struct BinaryExpr_t { Expr base;
    Expr* left;
    Token* operator;
    Expr* right;
} BinaryExpr;

BinaryExpr* binaryexpr_init(Expr* left, Token* operator, Expr* right);
void* binaryexpr_accept(Expr* self, ExprVisitor* visitor);

// -----------------------------------------

typedef struct GroupingExpr_t { Expr base;
    Expr* expr;
} GroupingExpr;

GroupingExpr* groupingexpr_init(Expr* expr);
void* groupingexpr_accept(Expr* self, ExprVisitor* visitor);

// -----------------------------------------

typedef struct LiteralExpr_t { Expr base;
    Token* value;
} LiteralExpr;

LiteralExpr* literalexpr_init(Token* value);
void* literalexpr_accept(Expr* self, ExprVisitor* visitor);

// -----------------------------------------

typedef struct UnaryExpr_t { Expr base;
    Token* operator;
    Expr* right;
} UnaryExpr;

UnaryExpr* unaryexpr_init(Token* operator, Expr* right);
void* unaryexpr_accept(Expr* self, ExprVisitor* visitor);

#endif