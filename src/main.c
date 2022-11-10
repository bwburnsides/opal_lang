#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include "lexer.h"


int main(int argc, char** argv) {
	char test_string[] = "fn main(argc: u8, argv: u8**): u8 {return 0;}";
	Token* token;
	Lexer* lexer = lexer_init((char*) test_string);
	if (lexer == NULL) {
		return -1;
	}

	for (int i = 0; i < 10; i++) {
		printf("Looped\n");
		token = lexer_consume(lexer);
		if (token == NULL) {
			printf("NULL Token\n");
		}
		token_print(token);
	}

	lexer_print(lexer);

	lexer_free(lexer);
	return 0;
}
