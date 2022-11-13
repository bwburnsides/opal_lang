#include <stdlib.h>
#include <stdio.h>

typedef struct ExpressionT Expression;
typedef struct ExpressionVisitorT ExpressionVisitor;

typedef enum ExpressionKindT {
    UnaryExpressionKind,
    BinaryExpressionKind,
} ExpressionKind;

typedef void (*ExpressionVisitorAcceptFunction)(Expression*, ExpressionVisitor*);

typedef struct ExpressionT {
    ExpressionKind kind;
    ExpressionVisitorAcceptFunction accept;
} Expression;

typedef struct UnaryExpressionT {
    Expression base;
    char* op;
    char* right;
} UnaryExpression;

typedef struct BinaryExpressionT {
    Expression base;
    char* left;
    char* op;
    char* right;
} BinaryExpression;

void ExpressionVisitorAccept(Expression* expr, ExpressionVisitor* visitor) {
    switch (expr->kind) {
        case UnaryExpressionKind:
            break;
        case BinaryExpressionKind:
            break;
        default:
            return;
    }
}

void* NewExpression(ExpressionKind kind) {
    void* expr = NULL;

    switch (kind) {
        case UnaryExpressionKind:
            expr = malloc(sizeof(UnaryExpression));
            break;
        case BinaryExpressionKind:
            expr = malloc(sizeof(BinaryExpression));
            break;
        default:
            return NULL;
    }

    if (expr != NULL) {
        ((Expression*) expr)->kind = kind;
        ((Expression*) expr)->accept = &ExpressionVisitorAccept;
    }

    return expr;
}

typedef enum ExpressionVisitorKindT {
    PrintExpressionVisitor,
    ExpressionVisitorCount
} ExpressionVisitorKind;

typedef void (*ExpressionVisitorFunction)(ExpressionVisitor*, Expression*);

typedef struct ExpressionVisitorT {
    ExpressionVisitorKind kind;
    void (*visit)(ExpressionVisitor* visitor, Expression* expr);
    ExpressionVisitorFunction unary_expression;
    ExpressionVisitorFunction binary_expression;
} ExpressionVisitor;

ExpressionVisitor* EXPRESSION_VISITORS[ExpressionVisitorCount];

void ExpressionVisit(ExpressionVisitor* visitor, Expression* expr) {
    switch (expr->kind) {
        case UnaryExpressionKind:
            return visitor->unary_expression(visitor, expr);
        case BinaryExpressionKind:
            return visitor->binary_expression(visitor, expr);
        default:
            return;
    }
}

void PrintUnaryExpression(ExpressionVisitor* visitor, Expression* expr) {
    UnaryExpression* uny = (UnaryExpression*) expr;
    printf(
        "UnaryExpression(\n\top=\"%s\"\n\tright=\"%s\"\n)\n",
        uny->op, uny->right
    );
}

void PrintBinaryExpression(ExpressionVisitor* visitor, Expression* expr) {
    BinaryExpression* bin = (BinaryExpression*) expr;
    printf(
        "BinaryExpression(\n\tleft=\"%s\"\n\top=\"%s\"\n\tright=\"%s\"\n)\n",
        bin->left, bin->op, bin->right
    );
}

void Visit(ExpressionVisitorKind kind, Expression* expr) {
    if (kind == ExpressionVisitorCount) {
        return;
    }

    EXPRESSION_VISITORS[kind]->visit(EXPRESSION_VISITORS[kind], expr);
}

void DefaultVisitorHandler(ExpressionVisitor* visitor, Expression* expr) {
    fprintf(
        stdout,
        "Unaccepted visit of ExpressionKind %d by ExpressionVisitor %d\n",
        expr->kind, visitor->kind
    );
    exit(-1);
}

void InitializeExpressionVisitors() {
    for (int i = 0; i < ExpressionVisitorCount; i++) {
        EXPRESSION_VISITORS[i] = malloc(sizeof(ExpressionVisitor));
        if (EXPRESSION_VISITORS[i] == NULL) {
            exit(-1);
        }
        EXPRESSION_VISITORS[i]->kind = i;
        EXPRESSION_VISITORS[i]->visit = &ExpressionVisit;
        EXPRESSION_VISITORS[i]->unary_expression = &DefaultVisitorHandler;
        EXPRESSION_VISITORS[i]->binary_expression = &DefaultVisitorHandler;
    }

    EXPRESSION_VISITORS[PrintExpressionVisitor]->unary_expression = &PrintUnaryExpression;
    EXPRESSION_VISITORS[PrintExpressionVisitor]->binary_expression = &PrintBinaryExpression;
}

int main(int argc, char** argv) {
    InitializeExpressionVisitors();

    UnaryExpression* uny = NewExpression(UnaryExpressionKind);
    if (uny == NULL) { return -1; }
    uny->right = "b";
    uny->op = "!";

    BinaryExpression* bin = NewExpression(BinaryExpressionKind);
    if (bin == NULL) { return -1; }
    bin->left = "a";
    bin->op = "+";
    bin->right = "c";

    Visit(PrintExpressionVisitor, (Expression*) bin);
    Visit(PrintExpressionVisitor, (Expression*) uny);

    return 0;
}