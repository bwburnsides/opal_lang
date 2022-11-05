#ifndef _LEXER_H
#define _LEXER_H

#include <stddef.h>

#include "token.h"

typedef struct Lexer_t {
	size_t index;
	char current;
	char* content;
} Lexer;

Lexer* lexer_init(char* content);
void lexer_free(Lexer* lexer);
int lexer_print(Lexer* lexer);
Token* lexer_next_token(Lexer* lexer);
void lexer_advance(Lexer* lexer);
Token* lexer_collect_name(Lexer* lexer);
Token* lexer_collect_string(Lexer* lexer);

#endif