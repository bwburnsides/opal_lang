#include <stdlib.h>
#include <stdio.h>
#include "visitor.h"
#include "print_visitor.h"


static void* _print_binary_expr(ExprVisitor* self, BinaryExpr* expr);
static void* _print_grouping_expr(ExprVisitor* self, GroupingExpr* expr);
static void* _print_literal_expr(ExprVisitor* self, LiteralExpr* expr);
static void* _print_identifier_expr(ExprVisitor* self, IdentifierExpr* expr);
static void* _print_unary_expr(ExprVisitor* self, UnaryExpr* expr);
static void* _print_call_expr(ExprVisitor* self, CallExpr* expr);

static void _print_indention(ExprVisitor* self);
static void _indent(size_t tabs);

typedef struct PrintExprVisitor_t {
    ExprVisitorClass* class;
    size_t indent_level;
} PrintExprVisitor;

ExprVisitorClass PrintExprVisitorClass = {
    &_print_binary_expr,
    &_print_grouping_expr,
    &_print_literal_expr,
    &_print_identifier_expr,
    &_print_unary_expr,
    &_print_call_expr,
};

PrintExprVisitor* print_visitor_init() {
    PrintExprVisitor* visitor = malloc(sizeof(PrintExprVisitor));
    if (visitor != NULL) {
        visitor->class = &PrintExprVisitorClass;
        visitor->indent_level = 0;
    }

    return visitor;
}

void print_visitor_free(PrintExprVisitor* self) {
    free(self);
}

static void _print_indention(ExprVisitor* self) {
    PrintExprVisitor* printer = (PrintExprVisitor*) self;
    for (int i = 0; i < (printer->indent_level * 3); i++) {
        printf(" ");
    }
}

static void _indent(size_t tabs) {
    for (int i = 0; i < (tabs * 3); i++) {
        printf(" ");
    }
}

static void* _print_binary_expr(ExprVisitor* self, BinaryExpr* expr) {
    ((PrintExprVisitor*) self)->indent_level++;

    printf("BinaryExpr:\n");
    _print_indention(self);

    visit(self, expr->left);

    printf("\n");
    _print_indention(self);

    printf("Operator: %s\n", expr->operator->value);
    _print_indention(self);

    visit(self, expr->right);

    ((PrintExprVisitor*) self)->indent_level--;
    return NULL;
}

static void* _print_grouping_expr(ExprVisitor* self, GroupingExpr* expr) {
    ((PrintExprVisitor*) self)->indent_level++;

    printf("GroupingExpr:\n");
    _print_indention(self);
    visit(self, expr->expr);

    ((PrintExprVisitor*) self)->indent_level--;
    return NULL;
}

static  void* _print_literal_expr(ExprVisitor* self, LiteralExpr* expr) {
    printf("LiteralExpr: %s", expr->value->value);
    return NULL;
}

static void* _print_identifier_expr(ExprVisitor* self, IdentifierExpr* expr) {
    printf("IdentifierExpr: %s", expr->value->value);
    return NULL;
}

static void* _print_unary_expr(ExprVisitor* self, UnaryExpr* expr) {
    ((PrintExprVisitor*) self)->indent_level++;

    printf("UnaryExpr:\n");
    _print_indention(self);
    printf("Operator: %s\n", expr->operator->value);
    _print_indention(self);
    visit(self, expr->right);

    ((PrintExprVisitor*) self)->indent_level--;
    return NULL;
}

static void* _print_call_expr(ExprVisitor* self, CallExpr* expr) {
    ((PrintExprVisitor*) self)->indent_level++;

    printf("CallExpr:\n");

    _print_indention(self);
    printf("Callee:\n");
    _print_indention(self);
    _print_indention(self);
    visit(self, expr->callee);

    printf("\n");
    _print_indention(self);
    printf("Argument Count: %d", expr->arg_count);

    printf("\n");
    _print_indention(self);
    printf("Arguments:\n");

    ((PrintExprVisitor*) self)->indent_level++;
    for (int idx = 0; idx < expr->arg_count; idx++) {
        _print_indention(self);
        visit(self, expr->arguments[idx]);
        printf("\n");
    }
    ((PrintExprVisitor*) self)->indent_level--;

    ((PrintExprVisitor*) self)->indent_level--;
    return NULL;
}
