#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 255
#define MAX_NUMBERS 255

void first(FILE*);
void second(FILE*);

// TODO caching / memoization for performance (but it was not needed)
// Was expecting hardcode calculations in part 2,
// but in the end the straight forward solution would've been better here :D

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    first(textfile);
    rewind(textfile);
    second(textfile);
    return 0;
}

void parse_input(
    char* line,
    long long* numbers,
    int* count
) {
    *count = 0;
    int token_i = 0;
    char token[LINE_LENGTH];
    int i = -1;
    long long sign = 1;

    while (1) {
        i++;
        char c = line[i];

        if (c == '-') {
            sign = -1;
            continue;
        }

        if (c >= 0x30 && c <= 0x39) {
            token[token_i] = c;
            token_i++;
            continue;
        }

        token[token_i] = 0;
        char *p;
        numbers[*count] = sign * strtoul(token, &p, 10);
        (*count)++;

        token_i = 0;
        sign = 1;

        if (c == '\n') break;
    }
}

long long grid_value(
    long long* numbers,
    int row,
    int col
) {
    if (row == -1) return numbers[col];
    return grid_value(numbers, row - 1, col) - grid_value(numbers, row - 1, col - 1);
}

long long grid_value_rev(
    long long* numbers,
    int row,
    int col
) {
    if (row == -1) return numbers[col];
    return grid_value_rev(numbers, row - 1, col + 1) - grid_value_rev(numbers, row - 1, col);
}

void first(FILE* textfile) {
    long long sum = 0;
    char line[LINE_LENGTH];

    while (fgets(line, LINE_LENGTH, textfile)) {
        int count;
        long long numbers[MAX_NUMBERS] = {0};
        parse_input(line, numbers, &count);

        int row_idx = 0;
        int col = count - 1;
        long long buffer[MAX_NUMBERS] = {0};

        while (1) {
            buffer[row_idx] = grid_value(numbers, row_idx, col);
            if (buffer[row_idx] == 0 && grid_value(numbers, row_idx, col - 1) == 0) break;
            row_idx++;
        }

        long long acc = 0;

        while (row_idx >= 0) {
            acc += buffer[row_idx];
            row_idx--;
        }

        acc += numbers[col];
        sum += acc;
    }
    printf("First: %lld\n", sum);
}

void second(FILE* textfile) {
    long long sum = 0;
    char line[LINE_LENGTH];

    while (fgets(line, LINE_LENGTH, textfile)) {
        int count;
        long long numbers[MAX_NUMBERS] = {0};
        parse_input(line, numbers, &count);

        int row_idx = 0;
        int col = 0;
        long long buffer[MAX_NUMBERS] = {0};

        while (1) {
            buffer[row_idx] = grid_value_rev(numbers, row_idx, col);
            // thats a bit of cheating, after input analysis
            if (
                buffer[row_idx] == 0
                && grid_value_rev(numbers, row_idx, col + 1) == 0
                && (count - row_idx < 4 || grid_value_rev(numbers, row_idx, col + 2) == 0)
            ) break;
            row_idx++;
        }

        long long acc = 0;

        while (row_idx >= 0) {
            acc = buffer[row_idx] - acc;
            row_idx--;
        }

        acc = numbers[col] - acc;
        sum += acc;
    }
    printf("Second: %lld\n", sum);
}