#define _CRT_SECURE_NO_WARNINGS
#include <stdlib.h>
#include <stdio.h>
#include "lexer.h"


int main(int argc, char** argv) {
	char test_string[] = "fn main(argc: u8, argv: u8**): u8 {return 0;}";

	Token* token;
	Lexer* lexer = lexer_init((char*) test_string);
	if (lexer == NULL) {
		return EXIT_FAILURE;
	}

	while (true) {
		token = lexer_consume(lexer);
		if (token == NULL) {
			printf("NULL Token\n");
		}
		if (token->kind == Token_EOF) {
			break;
		}
	}

	lexer_print(lexer);
	printf("\n");
	lexer_print_tokens(lexer);
	lexer_free(lexer);

	return EXIT_SUCCESS;
}
