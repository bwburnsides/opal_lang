#ifndef _LEXER_H
#define _LEXER_H

#include <stddef.h>
#include "token.h"

#define MAX_INT_LENGTH (100)
#define INIT_TOKEN_CAP (1000)

typedef struct Lexer_t {
	size_t index;
	char current;
	char* content;
	Token** tokens;
	size_t capacity;
	size_t count;
} Lexer;

Lexer* lexer_init(char* content);
void lexer_free(Lexer* lexer);
int lexer_print(Lexer* lexer);
int lexer_print_tokens(Lexer* lexer);
Token* lexer_consume(Lexer* lexer);
void lexer_append(Lexer* lexer, Token* token);

void lexer_advance(Lexer* lexer);
char lexer_peek(Lexer* lexer);

Token* lexer_collect_token(Lexer* lexer);
Token* lexer_collect_name(Lexer* lexer);
Token* lexer_collect_string(Lexer* lexer);
Token* lexer_collect_integer(Lexer* lexer);
Token* lexer_collect_primitive(Lexer* lexer);

#endif