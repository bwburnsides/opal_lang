#ifndef _STATEMENT_H
#define _STATEMENT_H

#include "token.h"
#include "expression.h"

typedef struct StmtVisitor_t StmtVisitor;
typedef struct Stmt_t Stmt;

typedef enum {
    ExpressionStmtKind,
    FunctionDeclStmtKind,
} StmtKind;

typedef struct {
    StmtKind kind;
    void* (*accept)(Stmt* self, StmtVisitor* visitor);
    void (*free)(Stmt* self);
} StmtClass;
void* stmt_accept(Stmt* self, StmtVisitor* visitor);
void stmt_free(Stmt* self);

typedef struct Stmt_t { StmtClass* cls; } Stmt;

// ------------------------------------------------------

StmtClass FunctionDeclStmtClass;
typedef struct { StmtClass* cls;
    Token* name;
    Token** params;
    Stmt** body;
} FunctionDeclStmt;

FunctionDeclStmt* functiondecl_stmt_init(Token* name, Token** params, Stmt** body);
void functiondecl_stmt_free(Stmt* self);
void* functiondecl_stmt_accept(Stmt* self, StmtVisitor* visitor);

// ------------------------------------------------------

StmtClass ExpressionStmtClass;
typedef struct { StmtClass* cls;
    Expr* expr;
} ExpressionStmt;

ExpressionStmt* expression_stmt_init(Expr* expr);
void expression_stmt_free(Stmt* self);
void* expression_stmt_accept(Stmt* self, StmtVisitor* visitor);

#endif // _STATEMENT_H