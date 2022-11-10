#include "token.h"
#include <stdlib.h>
#include <stdio.h>

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
	if (token->value_is_dynamic) {
		free(token->value);
	}
	free(token);
}

char* token_kind_repr(TokenKind kind) {
	switch (kind) {
		case Token_Keyword:
			return "Keyword";
		case Token_Identifier:
			return "Identifier";
		case Token_DecIntegerLiteral:
			return "DecIntegerLiteral";
		case Token_HexIntegerLiteral:
			return "HexIntegerLiteral";
		case Token_BinIntegerLiteral:
			return "BinIntegerLiteral";
		case Token_CharLiteral:
			return "CharLiteral";
		case Token_StringLiteral:
			return "StringLiteral";
		case Token_LSquare:
			return "LSquare";
		case Token_RSquare:
			return "RSquare";
		case Token_LParen:
			return "LParen";
		case Token_RParen:
			return "RParen";
		case Token_LCurly:
			return "LCurly";
		case Token_RCurly:
			return "RCurly";
		case Token_LAngle:
			return "LAngle";
		case Token_RAngle:
			return "RAngle";
		case Token_LChevron:
			return "LChevron";
		case Token_RChevron:
			return "RChevron";
		case Token_Colon:
			return "Colon";
		case Token_SemiColon:
			return "SemiColon";
		case Token_Comma:
			return "Comma";
		case Token_Plus:
			return "Plus";
		case Token_Minus:
			return "Minus";
		case Token_Exclamation:
			return "Exclamation";
		case Token_Ampersand:
			return "Ampersand";
		case Token_DoubleAmpersand:
			return "DoubleAmpersand";
		case Token_Bar:
			return "Bar";
		case Token_DoubleBar:
			return "DoubleBar";
		case Token_Caret:
			return "Caret";
		case Token_Equal:
			return "Equal";
		case Token_DoubleEqual:
			return "DoubleEqual";
		case Token_ExclamationEqual:
			return "ExclamationEqual";
		case Token_LessThan:
			return "LessThan";
		case Token_LessThanEqual:
			return "LessThanEqual";
		case Token_GreaterThan:
			return "GreaterThan";
		case Token_GreaterThanEqual:
			return "GreaterThanEqual";
		case Token_PlusEqual:
			return "PlusEqual";
		case Token_MinusEqual:
			return "MinusEqual";
		case Token_AsteriskEqual:
			return "AsteriskEqual";
		case Token_FSlashEqual:
			return "FSlashEqual";
		case Token_PercentEqual:
			return "PercentEqual";
		case Token_LChevronEqual:
			return "LChevronEqual";
		case Token_RChevronEqual:
			return "RChevronEqual";
		case Token_AmpersandEqual:
			return "AmpersandEqual";
		case Token_BarEqual:
			return "BarEqual";
		case Token_CaretEqual:
			return "CaretEqual";
		case Token_Asterisk:
			return "Asterisk";
		case Token_FSlash:
			return "FSlash";
		case Token_Percent:
			return "Percent";
		case Token_Dot:
			return "Dot";
		default:
			return "UNKNOWN";
	}
}

int token_print(Token* token) {
	return printf(
		"Token(\n   kind = %s\n   value = \"%s\"\n)\n",
		token_kind_repr(token->kind), token->value
	);
}