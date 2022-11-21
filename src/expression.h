#ifndef _EXPRESSION_H
#define _EXPRESSION_H

#include "token.h"
#include "visitor.h"

// TODO: Only public members of this header should be listed in the header. Move struct
// implementations (where necessary) and other declarations into *.c as static functions.

typedef struct ExprVisitor_t ExprVisitor;
typedef struct Expr_t Expr;

typedef enum ExprKind_t {
    BinaryExprKind,
    GroupingExprKind,
    LiteralExprKind,
    UnaryExprKind,
} ExprKind;

typedef struct ExprClass_t {
    ExprKind kind;
    void* (*accept)(Expr* self, ExprVisitor* visitor);
} ExprClass;

ExprClass BinaryExprClass;
ExprClass GroupingExprClass;
ExprClass LiteralExprClass;
ExprClass UnaryExprClass;

typedef struct Expr_t Expr;
typedef struct Expr_t {
    ExprClass* class;
} Expr;

void* expr_accept(Expr* self, ExprVisitor* visitor);

// -----------------------------------------

typedef struct BinaryExpr_t { ExprClass* class;
    Expr* left;
    Token* operator;
    Expr* right;
} BinaryExpr;

BinaryExpr* binaryexpr_init(Expr* left, Token* operator, Expr* right);
void* binaryexpr_accept(Expr* self, ExprVisitor* visitor);

// -----------------------------------------

typedef struct GroupingExpr_t { ExprClass* class;
    Expr* expr;
} GroupingExpr;

GroupingExpr* groupingexpr_init(Expr* expr);
void* groupingexpr_accept(Expr* self, ExprVisitor* visitor);

// -----------------------------------------

typedef struct LiteralExpr_t { ExprClass* class;
    Token* value;
} LiteralExpr;

LiteralExpr* literalexpr_init(Token* value);
void* literalexpr_accept(Expr* self, ExprVisitor* visitor);

// -----------------------------------------

typedef struct UnaryExpr_t { ExprClass* class;
    Token* operator;
    Expr* right;
} UnaryExpr;

UnaryExpr* unaryexpr_init(Token* operator, Expr* right);
void* unaryexpr_accept(Expr* self, ExprVisitor* visitor);

#endif