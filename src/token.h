#ifndef _TOKEN_H
#define _TOKEN_H

#define KEYWORD_TOKEN_COUNT (22)

typedef enum TokenKind_t {
	Token_Keyword,
	Token_Identifier,
	Token_IntegerLiteral,
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
	Token_Dot
} TokenKind;

typedef struct Token_t {
	TokenKind kind;
	char* value;
} Token;

char* KEYWORD_TOKENS[22];

Token* token_init(TokenKind kind, char* value);
void token_free(Token* token);

#endif