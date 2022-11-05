#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include <stdbool.h>
#include <string.h>

#include "lexer.h"
#include "token.h"

Lexer* lexer_init(char* content) {
	Lexer* lexer = malloc(sizeof(Lexer));

	if (lexer != NULL) {
		lexer->index = 0;
		lexer->content = content;
		lexer->current = lexer->content[0];
	}

	return lexer;
}

void lexer_free(Lexer* lexer) {
	free(lexer);
}

int lexer_print(Lexer* lexer) {
	return printf(
		"Lexer(\n   index = %i\n   current = \'%c\'\n   content = \"%s\"\n)\n",
		lexer->index, lexer->current, lexer->content
	);
}

Token* lexer_next_token(Lexer* lexer) {
	while (lexer->current != '\0') {
		while (lexer->current == ' ' || lexer->current == '\n') {
			lexer_advance(lexer);
		}

		if (isalpha(lexer->current)) {
			return lexer_collect_name(lexer);
		}

		return NULL;
	}
}

void lexer_advance(Lexer* lexer) {
    if (lexer->current != '\0') {
        lexer->index += 1;
        lexer->current = lexer->content[lexer->index];
    }
}

Token* lexer_collect_name(Lexer* lexer) {
	size_t start = lexer->index;
	size_t end = lexer->index;
	char* name;
	Token* token;
	TokenKind kind;
	bool is_keyword = false;

	lexer_advance(lexer);

	while (isalnum(lexer->current) || lexer->current == '_') {
		lexer_advance(lexer);
	}

	end = lexer->index;
	name = calloc(end - start + 1, sizeof(char));

	for (size_t i = start; i < end; i++) {
		name[i] = lexer->content[i];
	}
	name[end - start] = '\0';

	for (int i = 0; i < KEYWORD_TOKEN_COUNT; i++) {
		if (strcmp(name, KEYWORD_TOKENS[i]) == 0) {
			free(name);
			name = KEYWORD_TOKENS[i];
			is_keyword = true;
			break;
		}
	}

	if (is_keyword) {
		kind = Token_Keyword;
	} else {
		kind = Token_Identifier;
	}

	token = malloc(sizeof(Token));
	if (token != NULL) {
		token->value = name;
		token->kind = kind;
	}

	return token;
}
