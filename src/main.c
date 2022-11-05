#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include "lexer.h"

int main(int argc, char** argv) {
	char test_string[] = "fn malloc(size: u16): u8*;";

	Lexer* lexer = lexer_init((char*) test_string);
	lexer_print(lexer);

	lexer_next_token(lexer);
	lexer_print(lexer);

	lexer_next_token(lexer);
	lexer_print(lexer);

	lexer_free(lexer);

	return 0;
}
