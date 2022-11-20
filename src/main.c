#define _CRT_SECURE_NO_WARNINGS
#include <stdlib.h>
#include <stdio.h>
#include "lexer.h"

void usage() {
	fprintf(stderr, "usage: opal <input_fname>\n");
}

char* read_input(char* fname) {
	char* buffer = 0;
	long length;
	FILE* f = fopen(fname, "rb");

	if (f == NULL) {
		return NULL;
	}

	fseek(f, 0, SEEK_END);
	length = ftell(f);
	fseek(f, 0, SEEK_SET);

	buffer = malloc((length + 1) * sizeof(char));
	buffer[length] = '\0';

	if (buffer == NULL) {
		fclose(f);
		return NULL;
	}

	fread(buffer, sizeof(char), length, f);
	fclose(f);

	return buffer;
}

int main(int argc, char** argv) {
	char* fname;
	char* input;
	Token* token;
	Lexer* lexer;

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
