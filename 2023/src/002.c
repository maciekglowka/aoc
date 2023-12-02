#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255
#define MAX_DRAWS 10

int max(int, int);
void parse_line(int[][3], char[LINE_LENGTH]);
void print_draws(int[][3]);
void first(FILE*);
void second(FILE*);

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
    int game = 1;
    int sum = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int draws[MAX_DRAWS][3] = {0};
        int possible = 1;
        parse_line(draws, line);
        // print_draws(draws);
        for (int i=0; i<MAX_DRAWS; i++) {
            if (draws[i][0] > 12) possible = 0;
            if (draws[i][1] > 13) possible = 0;
            if (draws[i][2] > 14) possible = 0;
        }

        if (possible) sum += game;
        game++;
    }

    printf("First: %d\n", sum);
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];
    int game = 1;
    int sum = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int draws[MAX_DRAWS][3] = {0};
        int maxes[3] = {0};
        parse_line(draws, line);
        for (int i=0; i<MAX_DRAWS; i++) {
            for (int j=0; j<3; j++) {
                maxes[j] = max(draws[i][j], maxes[j]);
            }
        }
        game++;
        int power = maxes[0] * maxes[1] * maxes[2];
        sum += power;
    }

    printf("Second: %d\n", sum);
}

void reset_token(char *token) {
    for (int i=0; i<LINE_LENGTH; i++) {token[i] = 0;}
}

void print_draws(int draws[MAX_DRAWS][3]) {
    for (int i=0; i<MAX_DRAWS; i++) {
        printf("[");
        for (int j=0; j<3; j++) {
            printf("%d,", draws[i][j]);
        }
        printf("] ");
    }
    printf("\n");
}

void parse_line(int draws[][3], char line[LINE_LENGTH]) {
    char token[LINE_LENGTH] = {0};
    int token_i = 0;
    int draw_i = 0;
    int count = 0;

    for (int line_i=0; line_i<LINE_LENGTH; line_i++) {
        char c = line[line_i];

        if (c != ' ' && c != ':' && c!= ';' && c != '\n') {
            token[token_i] = c;
            token_i++;
            continue;
        }

        if (token[0] >= 0x30 && token[0] <= 0x39) {
            count = atoi(token);
        } else {
            if (token[0] == 'r') draws[draw_i][0] = count;
            if (token[0] == 'g') draws[draw_i][1] = count;
            if (token[0] == 'b') draws[draw_i][2] = count;
            // printf("%c%d\n", token[0], count);
        }

        reset_token(token);
        token_i = 0;
        if (c == ';') draw_i++;
        if (c == '\n') break;
    }
}

int max(int a, int b) {
    if (a > b) return a;
    return b;
}
