#ifndef _UTILS_H
#define _UTILS_H

void usage();
char* read_input(char* fname);
char* parse_arguments_or_exit(int argc, char* argv[]);
char* read_input_or_exit(char* fname);
void segfault_handler(int s);

#endif