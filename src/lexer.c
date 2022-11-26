#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>
#include <stdbool.h>
#include <string.h>
#include <ctype.h>

#include "lexer.h"
#include "token.h"

static bool ishex(char c);
static bool isbin(char c);
static bool isdec(char c);
static Token* lexer_collect_hex_integer(Lexer* lexer, char* buffer);
static Token* lexer_collect_bin_integer(Lexer* lexer, char* buffer);
static Token* lexer_collect_dec_integer(Lexer* lexer, char* buffer);

Lexer* lexer_init(char* content) {
	Lexer* lexer = malloc(sizeof(Lexer));

	if (lexer == NULL) {
		return NULL;
	}

	lexer->index = 0;
	lexer->current = content[0];
	lexer->content = content;

	lexer->tokens = calloc(INIT_TOKEN_CAP, sizeof(Token*));
	if (lexer->tokens == NULL) {
		free(lexer);
		return NULL;
	}

	lexer->capacity = INIT_TOKEN_CAP;
	lexer->count = 0;

	return lexer;
}

void lexer_free(Lexer* lexer) {
	for (int i = 0; i < lexer->count; i++) {
		free(lexer->tokens[i]);
	}

	free(lexer->tokens);
	free(lexer);
}

int lexer_print(Lexer* lexer) {
	return printf(
		"Lexer(\n   index = %i\n   current = \'%c\'\n   capacity = %i\n   count = %i\n)\n",
		lexer->index, lexer->current, lexer->capacity, lexer->count
	);
}

int lexer_print_tokens(Lexer* lexer) {
	for (int i = 0; i < lexer->count; i++) {
		printf("Index %d: ", i);
		token_print(lexer->tokens[i]);
	}
}

Token* lexer_consume(Lexer* lexer) {
	Token* token = lexer_collect_token(lexer);
	lexer_append(lexer, token);
	return token;
}

void lexer_append(Lexer* lexer, Token* token) {
	if (lexer->count > lexer->capacity) {
		lexer->tokens = realloc(
			lexer->tokens,
			(lexer->capacity * 2) * sizeof(Token*)
		);
	}
	lexer->tokens[lexer->count++] = token;
}

Token* lexer_collect_token(Lexer* lexer) {
	Token* token;
	if (lexer->current == '\0') {
		token = malloc(sizeof(Token));
		if (token == NULL) {
			return NULL;
		}
		token->kind = Token_EOF;
		token->value = "EOF";
		token->value_is_dynamic = false;
		return token;
	}

	if (isspace(lexer->current)) {
		lexer_advance(lexer);
		return lexer_collect_token(lexer);
	}

	if (isalpha(lexer->current)) {
		return lexer_collect_name(lexer);
	}

	if (isdigit(lexer->current)) {
		return lexer_collect_integer(lexer);
	}

	// Need to capture character and string literals

	return lexer_collect_primitive(lexer);
}

Token* lexer_collect_primitive(Lexer* lexer) {
	Token* token = malloc(sizeof(Token));
	if (token == NULL) {
		return NULL;
	}

	token->value_is_dynamic = false;

	switch (lexer->current) {
		case '[':
			token->value = "[";
			token->kind = Token_LSquare;
			break;
		case ']':
			token->value = "]";
			token->kind = Token_RSquare;
			break;
		case '(':
			token->value = "(";
			token->kind = Token_LParen;
			break;
		case ')':
			token->value = ")";
			token->kind = Token_RParen;
			break;
		case '{':
			token->value = "{";
			token->kind = Token_LCurly;
			break;
		case '}':
			token->value = "}";
			token->kind = Token_RCurly;
			break;
		case '<':
			if (lexer_peek(lexer) == '<') {
				token->value = "<<";
				token->kind = Token_LChevron;
				lexer_advance(lexer);
			} else {
				token->value = "<";
				token->kind = Token_LAngle;
			}
			break;
		case '>':
			if (lexer_peek(lexer) == '>') {
				token->value = ">>";
				token->kind = Token_RChevron;
				lexer_advance(lexer);
			} else {
				token->value = ">";
				token->kind = Token_RAngle;
			}
			break;
		case ':':
			token->value = ":";
			token->kind = Token_Colon;
			break;
		case ';':
			token->value = ";";
			token->kind = Token_SemiColon;
			break;
		case ',':
			token->value = ",";
			token->kind = Token_Comma;
			break;
		case '+':
			if (lexer_peek(lexer) == '=') {
				token->value = "+=";
				token->kind = Token_PlusEqual;
				lexer_advance(lexer);
			} else {
				token->value = "+";
				token->kind = Token_Plus;
			}
			break;
		case '-':
			if (lexer_peek(lexer) == '=') {
				token->value = "-=";
				token->kind = Token_MinusEqual;
				lexer_advance(lexer);
			} else {
				token->value = "-";
				token->kind = Token_Minus;
			}
			break;
		case '!':
			if (lexer_peek(lexer) == '=') {
				token->value = "!=";
				token->kind = Token_ExclamationEqual;
				lexer_advance(lexer);
			} else {
				token->value = "!";
				token->kind = Token_Exclamation;
			}
			break;
		case '&':
			if (lexer_peek(lexer) == '=') {
				token->value = "&=";
				token->kind = Token_AmpersandEqual;
				lexer_advance(lexer);
			} else if (lexer_peek(lexer) == '&') {
				token->value = "&&";
				token->kind = Token_DoubleAmpersand;
				lexer_advance(lexer);
			} else {
				token->value = "&";
				token->kind = Token_Ampersand;
			}
			break;
		case '|':
			if (lexer_peek(lexer) == '|') {
				token->value = "||";
				token->kind = Token_DoubleBar;
				lexer_advance(lexer);
			} else {
				token->value = "|";
				token->kind = Token_Bar;
			}
			break;
		case '^':
			if (lexer_peek(lexer) == '=') {
				token->value = "^=";
				token->kind = Token_CaretEqual;
				lexer_advance(lexer);
			} else {
				token->value = "^";
				token->kind = Token_Caret;
			}
			break;
		case '=':
			if (lexer_peek(lexer) == '=') {
				token->value = "==";
				token->kind = Token_DoubleEqual;
				lexer_advance(lexer);
			} else {
				token->value = "=";
				token->kind = Token_Equal;
			}
			break;
		case '*':
			if (lexer_peek(lexer) == '=') {
				token->value = "*=";
				token->kind = Token_AsteriskEqual;
				lexer_advance(lexer);
			} else {
				token->value = "*";
				token->kind = Token_Asterisk;
			}
			break;
		case '/':
			if (lexer_peek(lexer) == '=') {
				token->value = "/=";
				token->kind = Token_FSlashEqual;
				lexer_advance(lexer);
			} else {
				token->value = "/";
				token->kind = Token_FSlash;
			}
			break;
		case '%':
			if (lexer_peek(lexer) == '=') {
				token->value = "%=";
				token->kind = Token_PercentEqual;
				lexer_advance(lexer);
			} else {
				token->value = "%";
				token->kind = Token_Percent;
			}
			break;
		case '.':
			token->value = ".";
			token->kind = Token_Dot;
			break;

		default:
			printf(
				"Lexer error: unrecognized ASCII character 0x%02X at index %d.\n",
				lexer->current,
				lexer->index
			);
			exit(EXIT_FAILURE);
	}

	lexer_advance(lexer);
	return token;
}

char lexer_peek(Lexer* lexer) {
	if (lexer->current == '\0') {
		return '\0';
	}
	return lexer->content[lexer->index + 1];
}

void lexer_advance(Lexer* lexer) {
    if (lexer->current != '\0') {
        lexer->index += 1;
        lexer->current = lexer->content[lexer->index];
    }
}

bool token_str_is_keyword(char* str, char** keyword) {
	for (int i = 0; i < KEYWORD_TOKEN_COUNT; i++) {
		if (strcmp(str, KEYWORD_TOKENS[i]) == 0) {
			*keyword = KEYWORD_TOKENS[i];
			return true;
		}
	}
	return false;
}

Token* lexer_collect_name(Lexer* lexer) {
	size_t start = lexer->index;
	size_t end = lexer->index;
	char* name;
	Token* token;
	TokenKind kind;
	char INVALID_KEYWORD[] = "INVALID_KEYWORD";
	char* invalid_keyword = (char*) INVALID_KEYWORD;
	char** keyword = &invalid_keyword;

	token = malloc(sizeof(Token));
	if (token == NULL) {
		return NULL;
	}

	lexer_advance(lexer);
	while (isalnum(lexer->current) || lexer->current == '_') {
		lexer_advance(lexer);
	}

	end = lexer->index;
	name = calloc(end - start + 1, sizeof(char));
	if (name == NULL) {
		return NULL;
	}

	int j = 0;
	for (size_t i = start; i < end; i++) {
		name[j++] = lexer->content[i];
	}
	name[end - start] = '\0';

	if (token_str_is_keyword(name, keyword)) {
		free(name);
		token->kind = Token_Keyword;
		token->value = *keyword;
		token->value_is_dynamic = false;
	} else {
		token->kind = Token_Identifier;
		token->value = name;
		token->value_is_dynamic = true;
	}

	return token;
}

Token* lexer_collect_integer(Lexer* lexer) {
	char* integer_buffer = calloc(MAX_INT_LENGTH, sizeof(char));
	if (integer_buffer == NULL) {
		return NULL;
	}
	memset(integer_buffer, '\0', sizeof(char));

	char c = lexer_peek(lexer);

	if (lexer->current == '0' && c == 'x') {
		lexer_advance(lexer);
		lexer_advance(lexer);
		return lexer_collect_hex_integer(lexer, integer_buffer);	
	}

	if (lexer->current == '0' && c == 'b') {
		lexer_advance(lexer);
		lexer_advance(lexer);
		return lexer_collect_bin_integer(lexer, integer_buffer);
	}

	return lexer_collect_dec_integer(lexer, integer_buffer);
}

static Token* lexer_collect_hex_integer(Lexer* lexer, char* buffer) {
	// Lexer should be pointing at the hex digit immediately following the 'x' in the literal

	Token* token = malloc(sizeof(Token));
	if (token == NULL) {
		return NULL;
	}

	int idx = 0;

	while (ishex(lexer->current) && idx < MAX_INT_LENGTH) {
		buffer[idx++] = lexer->current;
		lexer_advance(lexer);
	}

	token->kind = Token_HexIntegerLiteral;
	token->value = buffer;
	token->value_is_dynamic = true;

	return token;
}

static Token* lexer_collect_bin_integer(Lexer* lexer, char* buffer) {
	Token* token = malloc(sizeof(Token));
	if (token == NULL) {
		return NULL;
	}

	int idx = 0;
	while (isbin(lexer->current) && idx < MAX_INT_LENGTH) {
		buffer[idx++] = lexer->current;
		lexer_advance(lexer);
	}

	token->kind = Token_BinIntegerLiteral;
	token->value = buffer;
	token->value_is_dynamic = true;

	return token;
}

static Token* lexer_collect_dec_integer(Lexer* lexer, char* buffer) {
	Token* token = malloc(sizeof(Token));
	if (token == NULL) {
		return NULL;
	}

	int idx = 0;
	while (isdec(lexer->current) && idx < MAX_INT_LENGTH) {
		buffer[idx++] = lexer->current;
		lexer_advance(lexer);
	}

	token->kind = Token_DecIntegerLiteral;
	token->value = buffer;
	token->value_is_dynamic = true;

	return token;
}

static bool ishex(char c) {
	char* hex = "0123456789abcdefABCDEF_";
	for (int i = 0; i < 23; i++) {
		if (c == hex[i]) {
			return true;
		}
	}
	return false;
}

static bool isbin(char c) {
	char* bin = "01_";
	for (int i = 0; i < 3; i++) {
		if (c == bin[i]) {
			return true;
		}
	}
	return false;
}

static bool isdec(char c) {
	char* dec = "0123456789_";
	for (int i = 0; i < 10; i++) {
		if (c == dec[i]) {
			return true;
		}
	}
	return false;
}
