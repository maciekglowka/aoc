#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255
#define HAND_SIZE 5
#define BID_IDX HAND_SIZE
#define ROW_SIZE HAND_SIZE + 1
#define MAX_ROWS 1000

void first(FILE*);
void get_row(char*, int*);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    first(textfile);
    // rewind(textfile);
    // second(textfile);
    return 0;
}

void first(FILE* textfile) {
    char line[LINE_LENGTH];
    int row_count = 0;
    int rows[MAX_ROWS][ROW_SIZE];

    while (fgets(line, LINE_LENGTH, textfile)) {
        get_row(line, rows[row_count]);
        row_count++;
    }
}

void get_row(char *line, int *out) {
    for (int i=0; i<5; i++) {
        char c = line[i];
        if (c >= 0x30 && c <= 0x39) {
            out[i] = c - 0x30;
        } else {
            if (c == 'T') out[i] = 10;
            if (c == 'J') out[i] = 11;
            if (c == 'Q') out[i] = 12;
            if (c == 'K') out[i] = 13;
            if (c == 'A') out[i] = 14;
        }
    }

    int token_i = 6;
    char token[LINE_LENGTH] = {0};

    while(1) {
        char c = line[token_i];
        if (c<0x30 || c>0x39) break;
        token[token_i - 6] = c;
        token_i++;
    }

    out[BID_IDX] = atoi(token);
    // for (int i=0; i<ROW_SIZE; i++) printf("%d ", out[i]);
    // printf("\n");
}

