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

Lexer* lexer_init(char* content) {
	Lexer* lexer = malloc(sizeof(Lexer));

	if (lexer == NULL) {
		return NULL;
	}

	lexer->index = 0;
	lexer->content = content;
	lexer->current = lexer->content[0];
	lexer->capacity = 0;
	lexer->count = 0;

	lexer->tokens = calloc(INIT_TOKEN_CT, sizeof(Token*));
	if (lexer->tokens == NULL) {
		return NULL;
	}

	return lexer;
}

void lexer_free(Lexer* lexer) {
	free(lexer->tokens);
	free(lexer);
}

int lexer_print(Lexer* lexer) {
	return printf(
		"Lexer(\n   index = %i\n   current = \'%c\'\n   content = \"%s\"\n   capacity = %i\n    count = %i\n)\n",
		lexer->index, lexer->current, lexer->content, lexer->capacity, lexer->count
	);
}

Token* lexer_consume(Lexer* lexer) {
	Token* token = lexer_collect_token(lexer);
	token_print(token);
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

	while (lexer->current == ' ' || lexer->current == '\n') {
		lexer_advance(lexer);
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
			printf("Lexer error: unrecognized character \'%c\'", lexer->current);
			exit(-1);
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
	char* integer = calloc(MAX_INT_LENGTH, sizeof(char));
	if (integer == NULL) {
		return NULL;
	}
	memset(integer, '\0', sizeof(char));

	int idx = 0;
	char peek = lexer_peek(lexer);

	Token* token = malloc(sizeof(Token));
	if (token == NULL) {
		return NULL;
	}
	token->value = integer;
	token->value_is_dynamic = true;

	if (lexer->current == '0') {
		switch (peek) {
			case 'x':
				lexer_advance(lexer);
				lexer_advance(lexer);

				token->value[idx++] = '0';
				token->value[idx++] = 'x';

				for (idx = 2; ishex(lexer->current) && idx < MAX_INT_LENGTH; idx++) {
					token->value[idx] = lexer->current;
					lexer_advance(lexer);
				}
				if (lexer->current != ' ') {
					goto cleanup;
				}

				token->kind = Token_HexIntegerLiteral;

				return token;
			case 'b':
				lexer_advance(lexer);
				lexer_advance(lexer);

				token->value[idx++] = '0';
				token->value[idx++] = 'b';

				for (idx = 2; ishex(lexer->current) && idx < MAX_INT_LENGTH; idx++) {
					token->value[idx] = lexer->current;
					lexer_advance(lexer);
				}
				if (lexer->current != ' ') {
					goto cleanup;
				}

				token->kind = Token_BinIntegerLiteral;

				return token;

			default:
				if (isspace(peek) || peek == ';') {
					lexer_advance(lexer);
					token->value[idx++] = '0';
					token->kind = Token_DecIntegerLiteral;
					return token;
				}

				goto cleanup;
		}
	}

	for (idx = 0; isdigit(lexer->current) && idx < MAX_INT_LENGTH; idx++) {
		token->value[idx] = lexer->current;
		lexer_advance(lexer);
	}
	if (!isspace(lexer->current)) {
		goto cleanup;
	}

	token->kind = Token_DecIntegerLiteral;

	return token;

	cleanup:
		token_free(token);
		return NULL;
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