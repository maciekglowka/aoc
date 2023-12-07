#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255
#define HAND_SIZE 5
#define BID_IDX HAND_SIZE
#define SCORE_IDX HAND_SIZE + 1
// hand size + bid + score
#define ROW_SIZE HAND_SIZE + 2
#define MAX_ROWS 1000
// this includes non existing 0 & 1 for easier indexing
#define CARD_TYPES 15
#define JOKER -1

void first(FILE*);
void second(FILE*);
void get_row(char*, int*, int);
void get_counts(int*, int*);
int get_counts_with_jokers(int*, int*);
int score_hand(int*);
int score_hand_with_jokers(int*, int);
int cmp_hands(const void*, const void*);

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
    int row_count = 0;
    int rows[MAX_ROWS][ROW_SIZE];

    while (fgets(line, LINE_LENGTH, textfile)) {
        get_row(line, rows[row_count], 11);
        int counts[5] = {0};
        get_counts(rows[row_count], counts);
        rows[row_count][SCORE_IDX] = score_hand(counts);
        // printf("%d\n", score);
        row_count++;
    }

    qsort(rows, row_count, 7 * sizeof(int), cmp_hands);
    int sum = 0;
    for (int i=0; i<row_count; i++) {
        sum += (i + 1) * rows[i][BID_IDX];
    }
    printf("First: %d\n", sum);
    // for (int i=0; i<row_count; i++) {
    //     for (int j=0; j<ROW_SIZE; j++) printf("%d ", rows[i][j]);
    //     printf("\n");
    // }
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];
    int row_count = 0;
    int rows[MAX_ROWS][ROW_SIZE];

    while (fgets(line, LINE_LENGTH, textfile)) {
        get_row(line, rows[row_count], JOKER);
        int counts[5] = {0};
        int jokers = get_counts_with_jokers(rows[row_count], counts);
        rows[row_count][SCORE_IDX] = score_hand_with_jokers(counts, jokers);
        // printf("%d\n", score);
        row_count++;
    }

    qsort(rows, row_count, 7 * sizeof(int), cmp_hands);
    int sum = 0;
    for (int i=0; i<row_count; i++) {
        sum += (i + 1) * rows[i][BID_IDX];
    }
    printf("Second: %d\n", sum);
    // for (int i=0; i<row_count; i++) {
    //     for (int j=0; j<ROW_SIZE; j++) printf("%d ", rows[i][j]);
    //     printf("\n");
    // }
}

int cmp_hands(const void *a, const void *b) {
    if (((int*)a)[SCORE_IDX] > ((int*)b)[SCORE_IDX]) return 1;
    if (((int*)a)[SCORE_IDX] < ((int*)b)[SCORE_IDX]) return -1;

    for (int i=0; i<HAND_SIZE; i++) {
        if (((int*)a)[i] > ((int*)b)[i]) return 1;
        if (((int*)a)[i] < ((int*)b)[i]) return -1;
    }
    return 0;
}

int score_hand_with_jokers(int *counts, int jokers) {
    // hard coding ftw
    if (counts[0] == 5) return 6;

    if (counts[0] == 4 && jokers == 1) return 6;
    if (counts[0] == 4) return 5;

    if (counts[0] == 3 && jokers == 2) return 6;
    if (counts[0] == 3 && jokers == 1) return 5;
    if (counts[0] == 3 && counts[1] == 2) return 4;
    if (counts[0] == 3) return 3;

    if (counts[0] == 2 && jokers == 3) return 6;
    if (counts[0] == 2 && jokers == 2) return 5;
    if (counts[0] == 2 && jokers == 1 && counts[1] == 2) return 4;
    if (counts[0] == 2 && jokers == 1) return 3;
    if (counts[0] == 2 && counts[1] == 2) return 2;
    if (counts[0] == 2) return 1;

    if (jokers == 5) return 6;
    if (jokers == 4) return 6;
    if (jokers == 3) return 5;
    if (jokers == 2) return 3;
    if (jokers == 1) return 1;

    return 0;
}

int score_hand(int *counts) {
    if (counts[0] == 5) return 6;
    if (counts[0] == 4) return 5;
    if (counts[0] == 3 && counts[1] == 2) return 4;
    if (counts[0] == 3) return 3;
    if (counts[0] == 2 && counts[1] == 2) return 2;
    if (counts[0] == 2) return 1;
    return 0;
}

int cmp(const void *a, const void *b) {
    return *(int*)b - *(int*)a;
}

int get_counts_with_jokers(int *row, int *out) {
    int counts[CARD_TYPES] = {0};
    int jokers = 0;
    for (int i=0; i<HAND_SIZE; i++) {
        if (row[i] != JOKER) {
            counts[row[i]]++;
        } else {
            jokers++;
        }
    }
    qsort(counts, CARD_TYPES, sizeof(int), cmp);
    // for (int i=0; i<CARD_TYPES; i++) printf("%d ", counts[i]);
    // printf("\n");
    for (int i=0; i<HAND_SIZE; i++) out[i] = counts[i];
    return jokers;
}

void get_counts(int *row, int *out) {
    int counts[CARD_TYPES] = {0};
    for (int i=0; i<HAND_SIZE; i++) {
        counts[row[i]]++;
    }
    qsort(counts, CARD_TYPES, sizeof(int), cmp);
    for (int i=0; i<HAND_SIZE; i++) out[i] = counts[i];
}

void get_row(char *line, int *out, int j_value) {
    for (int i=0; i<5; i++) {
        char c = line[i];
        if (c >= 0x30 && c <= 0x39) {
            out[i] = c - 0x30;
        } else {
            if (c == 'T') out[i] = 10;
            if (c == 'J') out[i] = j_value;
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

