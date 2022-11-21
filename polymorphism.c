#include <stdio.h>

typedef struct Expr_t Expr;
typedef struct ExprVisitor_t ExprVisitor;
typedef struct Token_t Token;

typedef struct ExprClass_t {
    void* (*accept)(Expr* self, ExprVisitor* visitor);
} ExprClass;

ExprClass BinaryExprClass;
ExprClass UnaryExprClass;

typedef struct Expr_t {
    ExprClass class;
} Expr;

void* expr_accept(Expr* self, ExprVisitor* visitor) {
    return self->class.accept(self, visitor);
}

typedef struct BinExpr_t {
    ExprClass* class;
    Expr* left;
    Token* operator;
    Expr* right;
} BinExpr;

BinExpr* bin_expr_init(Expr* left, Token* operator, Expr* right) {
    BinExpr* expr = malloc(sizeof(BinExpr));
    if (expr != NULL) {
        expr->class = &BinaryExprClass;
        expr->left = left;
        expr->operator = operator;
        expr->right = right;
    }
    return expr;
}

void* _bin_expr_accept(Expr* self, ExprVisitor* visitor) {
    BinExpr* expr = (BinExpr*) self;

    expr_accept(expr->left, visitor);
    expr_accept(expr->right, visitor);
    return NULL;
}

typedef struct UnyExpr_t {
    ExprClass* class;
    Token* operator;
    Expr* right;
} UnyExpr;

UnyExpr* uny_expr_init(Token* operator, Expr* right) {
    UnyExpr* expr = malloc(sizeof(UnyExpr));
    if (expr != NULL) {
        expr->class = &UnaryExprClass;
        expr->operator = operator;
        expr->right = right;
    }
    return expr;
}

void* _uny_expr_accept(Expr* self, ExprVisitor* visitor) {
    UnyExpr* expr = (UnyExpr*) self;
    expr_accept(expr->right, visitor);
    return NULL;
}

BinaryExprClass = {&_bin_expr_accept};
UnaryExprClass = {&_uny_expr_accept};