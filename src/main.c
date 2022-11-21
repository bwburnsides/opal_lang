#define _CRT_SECURE_NO_WARNINGS
#include <stdlib.h>
#include <stdio.h>
#include "utils.h"
#include "lexer.h"
#include "parser.h"
#include "print_visitor.h"

#include "expression.h"

int main(int argc, char* argv[]) {
	Token* left_lit_tok = token_init(Token_BinIntegerLiteral, "4");
	LiteralExpr *left_lit_expr = literalexpr_init(left_lit_tok);

	Token* right_lit_tok = token_init(Token_BinIntegerLiteral, "5");
	LiteralExpr *right_lit_expr = literalexpr_init(right_lit_tok);

	Token* operator = token_init(Token_Plus, "+");

	BinaryExpr *bin_expr = binaryexpr_init(
		(Expr*) left_lit_expr, operator, (Expr*) right_lit_expr
	);

	ExprVisitor printer = {PrintExprVisitorClass};
	visit_binary_expr(&printer, bin_expr);

	return EXIT_SUCCESS;
}

int real_main(int argc, char* argv[]) {
	char* fname;
	char* input;
	Token* token;
	Lexer* lexer;
	Parser* parser;

	if (argc < 2) {
		usage();
		exit(EXIT_FAILURE);
	}

	fname = argv[1];
	if (fname == NULL) {
		usage();
		exit(EXIT_FAILURE);
	}

	input = read_input(fname);
	if (input == NULL) {
		perror("Unable to open input_fname. Received error");
		return EXIT_FAILURE;
	}

	lexer = lexer_init((char*) input);
	if (lexer == NULL) {
		return EXIT_FAILURE;
	}

	while (true) {
		token = lexer_consume(lexer);
		if (token == NULL) {
			printf("NULL Token\n");
			break;
		}
		if (token->kind == Token_EOF) {
			break;
		}
	}

	lexer_print(lexer);
	printf("\n");
	lexer_print_tokens(lexer);

	lexer_free(lexer);
	free(input);

	return EXIT_SUCCESS;
}
