#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255

void first(FILE*);
void second(FILE*);
void get_symbols(int*, char*);
void mask_add(int*, int*);
int mask_check(int*, int, int);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    first(textfile);
    rewind(textfile);
    second(textfile);

    return 0;
}

void first(FILE* textfile) {
    char line[LINE_LENGTH];
    char next[LINE_LENGTH];
    int sum = 0;
    int prev_symbols[LINE_LENGTH] = {0};

    while (fgets(line, LINE_LENGTH, textfile)) {
        int current_symbols[LINE_LENGTH] = {0};
        int next_symbols[LINE_LENGTH] = {0};
        int mask[LINE_LENGTH] = {0};
        get_symbols(current_symbols, line);

        char *has_next = fgets(next, LINE_LENGTH, textfile);
        if (has_next) {
            int rewind = strlen(next);
            fseek(textfile, -rewind, SEEK_CUR);
            get_symbols(next_symbols, next);
        }

        mask_add(mask, prev_symbols);
        mask_add(mask, current_symbols);
        mask_add(mask, next_symbols);
        for (int i=0; i<LINE_LENGTH; i++) prev_symbols[i] = current_symbols[i];

        char token[LINE_LENGTH] = {0};
        int token_i = 0;
        int token_start = 0;

        for (int i=0; i<LINE_LENGTH; i++) {
            char c = line[i];
            if (c >= 0x30 && c <= 0x39) {
                token[token_i] = c;
                token_i++;
                if (token_start == 0) token_start = i;
                continue;
            }

            if (token_i == 0) continue;
            int val = atoi(token);

            int mask_start = (token_start == 0 ) ? 0 : token_start - 1;
            if (mask_check(mask, mask_start, i)) sum += val;


            token_i = 0;
            token_start = 0;
            for (int i=0; i<LINE_LENGTH; i++) token[i] = 0;
        }
    }
    printf("First: %d\n", sum);
}

void second(FILE* textfile) {
    char buffer[3][LINE_LENGTH] = {'.'};
    int sum = 0;

    while (fgets(buffer[1], LINE_LENGTH, textfile)) {
        for (int line_i=0; line_i<LINE_LENGTH; line_i++) {
            if (buffer[1][line_i] != '*') continue;
            int parts[6] = {0};
            int part_count = 0;

            char *has_next = fgets(buffer[2], LINE_LENGTH, textfile);
            if (has_next) {
                int rewind = strlen(buffer[2]);
                fseek(textfile, -rewind, SEEK_CUR);
            } else {
                for (int i=0; i<LINE_LENGTH; i++) buffer[2][i] = '.';
            }

            for (int b=0; b<=2; b++) {
                char token[LINE_LENGTH] = {0};
                int token_i = 0;
                int token_start = -1;
                for (int i=0; i<LINE_LENGTH; i++) {
                    char c = buffer[b][i];
                    if (c >= 0x30 && c <= 0x39) {
                        token[token_i] = c;
                        token_i++;
                        if (token_start == -1) token_start = i;
                        continue;
                    }

                    if (token_start == -1) continue;
                    int val = atoi(token);
                    if (
                        abs(token_start - line_i) <= 1
                        || abs(line_i - i) < 1
                        || (token_start < line_i && i - 1 >= line_i)
                     ) {
                        // printf("%d\n", val);
                        parts[part_count] = val;
                        part_count++;
                    }

                    token_i = 0;
                    token_start = -1;
                    for (int k=0; k<LINE_LENGTH; k++) token[k] = 0;
                    if (c == '\n') break;
                }
            }
            if (part_count == 2) {
                sum += parts[0] * parts[1];
            }
        }
        for (int i=0; i<LINE_LENGTH; i++) buffer[0][i] = buffer[1][i];
    }
    printf("Second: %d\n", sum);
}

void get_symbols(int *out, char *line) {
    for (int i=0; i<LINE_LENGTH; i++) {out[i] = 0;}
    for (int i=0; i<LINE_LENGTH; i++) {
        if (
            (line[i] >= 0x30 && line[i] <= 0x39)
            || line[i] == '.'
        ) {
            continue;
        }
        if (line[i] == '\n' || line[i] == 0) break;
        out[i] = 1;
    }
}

void mask_add(int *a, int *b) {
    for (int i=0; i<LINE_LENGTH; i++) {
        a[i] += b[i];
    }
}

int mask_check(int *mask, int start, int end) {
    for (int i=start; i<=end; i++) {
        if (mask[i] > 0) return 1;
    }
    return 0;
}