#include "token.h"
#include <stdlib.h>

char* KEYWORD_TOKENS[22] = {
	"u8",
	"i8",
	"u16",
	"i16",
	"fn",
	"sizeof",
	"type",
	"const",
	"if",
	"else",
	"while",
	"for",
	"case",
	"default",
	"switch",
	"break",
	"continue",
	"return",
	"extern",
	"struct",
	"union",
	"enum"
};

Token* token_init(TokenKind kind, char* value) {
	Token* token = malloc(sizeof(Token));

	if (token != NULL) {
		token->kind = kind;
		token->value = value;
	}

	return token;
}

void token_free(Token* token) {
	if (token->kind == Token_Identifier) {
		free(token->value);
	}
	free(token);
}