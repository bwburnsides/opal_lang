#include <stdlib.h>
#include <stdio.h>
#include "print_visitor.h"


static void* _print_binary_expr(ExprVisitor* self, BinaryExpr* expr);
static void* _print_grouping_expr(ExprVisitor* self, GroupingExpr* expr);
static void* _print_literal_expr(ExprVisitor* self, LiteralExpr* expr);
static void* _print_unary_expr(ExprVisitor* self, UnaryExpr* expr);
static void _print_indention(ExprVisitor* self);

typedef struct PrintExprVisitor_t {
    ExprVisitorClass* class;
    size_t indent_level;
} PrintExprVisitor;

ExprVisitorClass PrintExprVisitorClass = {
    &_print_binary_expr,
    &_print_grouping_expr,
    &_print_literal_expr,
    &_print_unary_expr,
};

PrintExprVisitor* printvisitor_init() {
    PrintExprVisitor* visitor = malloc(sizeof(PrintExprVisitor));
    if (visitor != NULL) {
        visitor->class = &PrintExprVisitorClass;
        visitor->indent_level = 0;
    }

    return visitor;
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
    printf("\n");

    ((PrintExprVisitor*) self)->indent_level--;
    return NULL;
}

static void* _print_grouping_expr(ExprVisitor* self, GroupingExpr* expr) {
    return NULL;
}

static  void* _print_literal_expr(ExprVisitor* self, LiteralExpr* expr) {
    printf("LiteralExpr: %s", expr->value->value);
    return NULL;
}

static void* _print_unary_expr(ExprVisitor* self, UnaryExpr* expr) {
    return NULL;
}

static void _print_indention(ExprVisitor* self) {
    PrintExprVisitor* printer = (PrintExprVisitor*) self;
    for (int i = 0; i < (printer->indent_level * 3); i++) {
        printf(" ");
    }
}