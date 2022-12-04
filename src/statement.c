#include <stdlib.h>
#include "statement.h"


void* stmt_accept(Stmt* self, StmtVisitor* visitor) {
    return self->cls->accept(self, visitor);
}

void stmt_free(Stmt* self) {
    self->cls->free(self);
}

// -------------------------------------------------------------------------------------

StmtClass FunctionDeclStmtClass = {
    FunctionDeclStmtKind,
    &functiondecl_stmt_accept,
    &functiondecl_stmt_free,
};

FunctionDeclStmt* functiondecl_stmt_init(Token* name, Token** params, Stmt** body) {
    FunctionDeclStmt* decl = malloc(sizeof(FunctionDeclStmt));
    if (decl != NULL) {
        decl->cls = &FunctionDeclStmtClass;
        decl->name = name;
        decl->params = params;
        decl->body = body;
    }
    return decl;
}

void* functiondecl_stmt_accept(Stmt* self, StmtVisitor* visitor) {
    return NULL;
}

void functiondecl_stmt_free(Stmt* self) {
    FunctionDeclStmt* stmt = (FunctionDeclStmt*) self;
    token_free(stmt->name);
    for (int idx = 0; stmt->params[idx] != NULL; idx++) {
        token_free(stmt->params[idx]);
    }
    for (int idx = 0; stmt->body[idx] != NULL; idx++) {
        stmt_free(stmt->body[idx]);
    }
    free(stmt);
}

// -------------------------------------------------------------------------------------

StmtClass ExpressionStmtClass = {
    ExpressionStmtKind,
    &expression_stmt_accept,
    &expression_stmt_free,
};

ExpressionStmt* expression_stmt_init(Expr* expr) {
    ExpressionStmt* stmt = malloc(sizeof(ExpressionStmt));
    if (stmt != NULL) {
        stmt->cls = &ExpressionStmtClass;
        stmt->expr = expr;
    }
    return stmt;
}

void expression_stmt_free(Stmt* self) {
    ExpressionStmt* stmt = (ExpressionStmt*) self;
    expr_free(stmt->expr);
    free(stmt);
}

void* expression_stmt_accept(Stmt* self, StmtVisitor* visitor) {
    return NULL;
}