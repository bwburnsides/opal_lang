#include <stdio.h>
#include <stdlib.h>
#include "utils.h"


void usage() {
	fprintf(stderr, "usage: opal <input_fname>\n");
}

char* parse_arguments_or_exit(int argc, char* argv[]) {
	if (argc < 2) {
		usage();
		exit(EXIT_FAILURE);
	}

	if (argv[1] == NULL) {
		usage();
		exit(EXIT_FAILURE);
	}

	return argv[1];
}

char* read_input_or_exit(char* fname) {
	char* buffer = 0;
	long length;
	FILE* f = fopen(fname, "rb");

	if (f == NULL) {
		perror("Unable to open input_fname. Received error");
		exit(EXIT_FAILURE);
	}

	fseek(f, 0, SEEK_END);
	length = ftell(f);
	fseek(f, 0, SEEK_SET);

	buffer = malloc((length + 1) * sizeof(char));
	buffer[length] = '\0';

	if (buffer == NULL) {
		perror("Unable to allocate input buffer. Received error");
		fclose(f);
		exit(EXIT_FAILURE);
	}

	fread(buffer, sizeof(char), length, f);
	fclose(f);

	return buffer;
}


void segfault_handler(int s) {
  printf("Segmentation Fault\n");
  exit(EXIT_FAILURE);
}