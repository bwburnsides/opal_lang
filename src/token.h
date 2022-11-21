#ifndef _TOKEN_H
#define _TOKEN_H

#include <stdbool.h>

// TODO: Only public members of this header should be listed in the header. Move struct
// implementations (where necessary) and other declarations into *.c as static functions.

#define KEYWORD_TOKEN_COUNT (22)

#define HEX_CHARS "0123456789abcdef"
#define BIN_CHARS "01"

typedef enum TokenKind_t {
	Token_Keyword,
	Token_Identifier,
	Token_DecIntegerLiteral,
	Token_HexIntegerLiteral,
	Token_BinIntegerLiteral,
	Token_CharLiteral,
	Token_StringLiteral,
	Token_LSquare,
	Token_RSquare,
	Token_LParen,
	Token_RParen,
	Token_LCurly,
	Token_RCurly,
	Token_LAngle,
	Token_RAngle,
	Token_LChevron,
	Token_RChevron,
	Token_Colon,
	Token_SemiColon,
	Token_Comma,
	Token_Plus,
	Token_Minus,
	Token_Exclamation,
	Token_Ampersand,
	Token_DoubleAmpersand,
	Token_Bar,
	Token_DoubleBar,
	Token_Caret,
	Token_Equal,
	Token_DoubleEqual,
	Token_ExclamationEqual,
	Token_LessThan,
	Token_LessThanEqual,
	Token_GreaterThan,
	Token_GreaterThanEqual,
	Token_PlusEqual,
	Token_MinusEqual,
	Token_AsteriskEqual,
	Token_FSlashEqual,
	Token_PercentEqual,
	Token_LChevronEqual,
	Token_RChevronEqual,
	Token_AmpersandEqual,
	Token_BarEqual,
	Token_CaretEqual,
	Token_Asterisk,
	Token_FSlash,
	Token_Percent,
	Token_Dot,
	Token_EOF
} TokenKind;

typedef struct Token_t {
	TokenKind kind;
	char* value;
	bool value_is_dynamic;
} Token;

char* KEYWORD_TOKENS[22];

Token* token_init(TokenKind kind, char* value);
void token_free(Token* token);
int token_print(Token* token);
char* token_kind_repr(TokenKind kind);
bool token_str_is_keyword(char* str, char** keyword);

#endif