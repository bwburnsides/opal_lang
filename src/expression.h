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
    IdentifierExprKind,
    UnaryExprKind,
    CallExprKind,
} ExprKind;

typedef struct ExprClass_t {
    ExprKind kind;
    void* (*accept)(Expr* self, ExprVisitor* visitor);
    void (*free)(Expr* self);
} ExprClass;

ExprClass BinaryExprClass;
ExprClass GroupingExprClass;
ExprClass LiteralExprClass;
ExprClass IdentifierExprClass;
ExprClass UnaryExprClass;
ExprClass CallExprClass;

typedef struct Expr_t Expr;
typedef struct Expr_t {
    ExprClass* cls;
} Expr;

void* expr_accept(Expr* self, ExprVisitor* visitor);
void expr_free(Expr* self);

// -----------------------------------------

typedef struct BinaryExpr_t { ExprClass* cls;
    Expr* left;
    Token* operator;
    Expr* right;
} BinaryExpr;

BinaryExpr* binaryexpr_init(Expr* left, Token* operator, Expr* right);
void* binaryexpr_accept(Expr* self, ExprVisitor* visitor);
void binaryexpr_free(Expr* self);

// -----------------------------------------

typedef struct GroupingExpr_t { ExprClass* cls;
    Expr* expr;
} GroupingExpr;

GroupingExpr* groupingexpr_init(Expr* expr);
void* groupingexpr_accept(Expr* self, ExprVisitor* visitor);
void groupingexpr_free(Expr* self);

// -----------------------------------------

typedef struct LiteralExpr_t { ExprClass* cls;
    Token* value;
} LiteralExpr;

LiteralExpr* literalexpr_init(Token* value);
void* literalexpr_accept(Expr* self, ExprVisitor* visitor);
void literalexpr_free(Expr* self);

// -----------------------------------------

typedef struct IdentifierExpr_t { ExprClass* cls;
    Token* value;
} IdentifierExpr;

IdentifierExpr* identifierexpr_init(Token* value);
void* identifierexpr_accept(Expr* self, ExprVisitor* visitor);
void identifierexpr_free(Expr* self);

// -----------------------------------------

typedef struct UnaryExpr_t { ExprClass* cls;
    Token* operator;
    Expr* right;
} UnaryExpr;

UnaryExpr* unaryexpr_init(Token* operator, Expr* right);
void* unaryexpr_accept(Expr* self, ExprVisitor* visitor);
void unaryexpr_free(Expr* self);

// -----------------------------------------

typedef struct CallExpr_t { ExprClass* cls;
    Expr* callee;
    Token* paren;
    Expr** arguments;
    size_t arg_count;
} CallExpr;

CallExpr* callexpr_init(Expr* callee, Token* paren, Expr** arguments, size_t count);
void* callexpr_accept(Expr* self, ExprVisitor* visitor);
void callexpr_free(Expr* self);

#endif