#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define LINE_LENGTH 255
#define MAX_COLUMNS 10

struct tuple { 
    long a;
    long b;
};

void first(FILE*);
void second(FILE*);
int get_numbers(char*, long*, int);
struct tuple get_bounds(long, long);

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
    long time[MAX_COLUMNS] = {0};
    long record[MAX_COLUMNS] = {0};
    int column_count = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        if (column_count == 0) {
            column_count = get_numbers(line, time, 0);
            continue;
        }
        get_numbers(line, record, 0);
        break;
    }

    long result = 1;

    for (int i=0; i<column_count; i++) {
        struct tuple t = get_bounds(time[i], record[i]);
        result *= t.b - t.a - 1;
    }

    printf("First: %ld\n", result);
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];
    long time[MAX_COLUMNS] = {0};
    long record[MAX_COLUMNS] = {0};
    int column_count = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        if (column_count == 0) {
            column_count = get_numbers(line, time, 1);
            continue;
        }
        get_numbers(line, record, 1);
        break;
    }

    long result = 1;

    for (int i=0; i<column_count; i++) {
        struct tuple t = get_bounds(time[i], record[i]);
        result *= t.b - t.a - 1;
    }

    // for (int i=0; i<column_count; i++) printf("%ld ", time[i]);
    // printf("\n");
    // for (int i=0; i<column_count; i++) printf("%ld ", record[i]);
    // printf("\n");
    printf("Second: %ld\n", result);
}

double min(double a, double b) {
    if (a < b) return a;
    return b;
}
double max(double a, double b) {
    if (a > b) return a;
    return b;
}

struct tuple get_bounds(long time, long record) {
    double d = (double)time * (double)time - 4.0 * (double)record;
    struct tuple t = { .a = 0, .b =0 };

    if (d > 0) {
        double a = ((double)time + sqrt((double)d)) / 2.0;
        double b = ((double)time - sqrt((double)d) ) / 2.0;
        t.a = floor(min(a, b));
        t.b = ceil(max(a, b));
    }
    // printf("time: %ld record: %ld, d: %f %ld %ld\n", time, record, d, t.a, t.b);
    return t;
}

int get_numbers(char *line, long *out, int skip_whitespace) {
    char token[LINE_LENGTH] = {0};
    int token_i = 0;
    int count = 0;

    for (int i=0; i<LINE_LENGTH; i++) {
        char c = line[i];
        if (c >= 0x30 && c <= 0x39) {
            token[token_i] = c;
            token_i++;
            continue;
        }

        if (skip_whitespace && c == ' ') continue;

        if (token_i > 0) {
            char *p;
            long value = strtoul(token, &p, 10);
            out[count] = value;
            count++;
        }

        // reset token
        token_i = 0;
        for (int i=0; i<LINE_LENGTH; i++) {token[i] = 0;}
        if (c == '\n') break;
    }
    return count;
}
