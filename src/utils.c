#include <stdio.h>
#include <stdlib.h>
#include "utils.h"


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
