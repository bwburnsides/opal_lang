#ifndef _PRINT_VISITOR_H
#define _PRINT_VISITOR_H

#include "visitor.h"

typedef struct PrintExprVisitor_t PrintExprVisitor;

PrintExprVisitor* print_visitor_init();
void print_visitor_free(PrintExprVisitor* self);

#endif