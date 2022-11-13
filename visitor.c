#include <stdlib.h>
#include <stdio.h>

#define EXPR_ACCEPT(expr, visitor_kind) (expr)->base.accept((Expression*) expr, (visitor_kind))

typedef struct ExpressionT Expression;
typedef struct ExpressionVisitorT ExpressionVisitor;

typedef enum ExpressionKindT {
    UnaryExpressionKind,
    BinaryExpressionKind,
} ExpressionKind;

typedef enum ExpressionVisitorKindT {
    PrintExpressionVisitor,
    //
    ExpressionVisitorCount
} ExpressionVisitorKind;

ExpressionVisitor* EXPRESSION_VISITORS[ExpressionVisitorCount];

typedef void (*ExpressionVisitorFunction)(ExpressionVisitor*, Expression*);

typedef struct ExpressionVisitorT {
    ExpressionVisitorKind kind;
    ExpressionVisitorFunction unary_expression;
    ExpressionVisitorFunction binary_expression;
} ExpressionVisitor;

typedef void (*ExpressionVisitorAcceptFunction)(Expression*, ExpressionVisitorKind);

typedef struct ExpressionT {
    ExpressionKind kind;
    ExpressionVisitorAcceptFunction accept;
} Expression;

typedef struct UnaryExpressionT {
    Expression base;
    char* op;
    char* right;
} UnaryExpression;

void UnaryExpressionAccept(Expression* expr, ExpressionVisitorKind visitor_kind) {
    ExpressionVisitor* visitor = EXPRESSION_VISITORS[visitor_kind];
    visitor->unary_expression(visitor, expr);
}

typedef struct BinaryExpressionT {
    Expression base;
    char* left;
    char* op;
    char* right;
} BinaryExpression;

void BinaryExpressionAccept(Expression* expr, ExpressionVisitorKind visitor_kind) {
    ExpressionVisitor* visitor = EXPRESSION_VISITORS[visitor_kind];
    visitor->binary_expression(visitor, expr);
}

void* NewExpression(ExpressionKind kind) {
    void* expr = NULL;

    switch (kind) {
        case UnaryExpressionKind:
            expr = malloc(sizeof(UnaryExpression));
            if (expr != NULL) {
                ((Expression*) expr)->accept = &UnaryExpressionAccept;
            }
            break;
        case BinaryExpressionKind:
            expr = malloc(sizeof(BinaryExpression));
            if (expr != NULL) {
                ((Expression*) expr)->accept = &BinaryExpressionAccept;
            }
            break;
        default:
            return NULL;
    }

    if (expr != NULL) {
        ((Expression*) expr)->kind = kind;
    }

    return expr;
}

ExpressionVisitor* EXPRESSION_VISITORS[ExpressionVisitorCount];

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
        EXPRESSION_VISITORS[i]->unary_expression = &DefaultVisitorHandler;
        EXPRESSION_VISITORS[i]->binary_expression = &DefaultVisitorHandler;
    }

    EXPRESSION_VISITORS[PrintExpressionVisitor]->unary_expression = &PrintUnaryExpression;
    EXPRESSION_VISITORS[PrintExpressionVisitor]->binary_expression = &PrintBinaryExpression;
}

int main(int argc, char** argv) {
    InitializeExpressionVisitors();

    UnaryExpression* uny = NewExpression(UnaryExpressionKind);
    if (uny == NULL) {
        return EXIT_FAILURE;
    }
    uny->right = "b";
    uny->op = "!";

    BinaryExpression* bin = NewExpression(BinaryExpressionKind);
    if (bin == NULL) {
        return EXIT_FAILURE;
    }
    bin->left = "a";
    bin->op = "+";
    bin->right = "c";

    EXPR_ACCEPT(uny, PrintExpressionVisitor);
    EXPR_ACCEPT(bin, PrintExpressionVisitor);

    return EXIT_SUCCESS;
}