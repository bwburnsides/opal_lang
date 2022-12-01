#define _CRT_SECURE_NO_WARNINGS

#include <stdlib.h>
#include <stdio.h>
#include "utils.h"
#include "lexer.h"
#include "parser.h"
#include "print_visitor.h"

#include <signal.h>


int main(int argc, char* argv[]) {
	signal(SIGSEGV, segfault_handler);

	char* fname = parse_arguments_or_exit(argc, argv);
	char* input = read_input_or_exit(fname);
	Token* token;
	Lexer* lexer;
	Parser* parser;

	lexer = lexer_init(input);
	if (lexer == NULL) {
		return EXIT_FAILURE;
	}

	while (true) {
		token = lexer_consume(lexer);
		if (token == NULL) {
			fprintf(stderr, "NULL Token\n");
			return EXIT_FAILURE;
		}

		if (token->kind == Token_EOF) {
			break;
		}
	}
	lexer_print_tokens(lexer);

	parser = parser_init(lexer->tokens);
	if (parser == NULL) {
		return EXIT_FAILURE;
	}

	ParseResult result = parser_expression(parser);
	PrintExprVisitor* AstPrinter = print_visitor_init();

	if (result.kind != ParseResult_Error) {
		printf("\nParsing was successful.\n\n");

		if (AstPrinter != NULL) {
			expr_accept(result.value.match, (ExprVisitor*) AstPrinter);
		}
	}

	// lexer_free(lexer);
	// free(input);

	return EXIT_SUCCESS;
}
