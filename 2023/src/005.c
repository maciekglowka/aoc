#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255
#define MAX_ROWS 100
#define MAX_SEEDS 100
#define SECTION_COUNT 7
#define SEED 120415
#define HASH_LEN 20

struct tuple {
    unsigned long a;
    unsigned long b;
};

struct seed {
    int initial_section;
    unsigned long start_value;
    unsigned long end_value;
};

struct section {
    int length;
    unsigned long data[MAX_ROWS][3];
};

void first(FILE*);
void second(FILE*);
unsigned long map(unsigned long, struct section*);
struct tuple map_with_max(unsigned long, unsigned long, struct section*);
int get_numbers(char*, unsigned long*);

unsigned long min(unsigned long a, unsigned long b) {
    if (a < b) return a;
    return b;
}
unsigned long max(unsigned long a, unsigned long b) {
    if (a > b) return a;
    return b;
}

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
    unsigned long seeds[MAX_SEEDS] = {0};
    int seed_count = 0;

    struct section sections[SECTION_COUNT];
    int current_section = -1;
    int section_row = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        if (strlen(line) == 1) {
            current_section++;
            section_row = 0;
            sections[current_section].length = 0;
            continue;
        }


        if (seed_count == 0) {
            seed_count = get_numbers(line, seeds);
            continue;
        }

        unsigned long data[3];
        int count = get_numbers(line, data);
        if (count == 0) continue;

        for (int j=0; j<3; j++) sections[current_section].data[section_row][j] = data[j];
        section_row++;
        sections[current_section].length++;
    }

    unsigned long result = ULONG_MAX;

    for (int i=0; i<seed_count; i++) {
        unsigned long value = seeds[i];
        for (int j=0; j<SECTION_COUNT; j++) {
            value = map(value, &sections[j]);
        }
        result = min(result, value);
    }

    printf("First: %lu\n", result);
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];
    int seed_count = 0;
    struct seed seeds[MAX_SEEDS];

    struct section sections[SECTION_COUNT];
    int current_section = -1;
    int section_row = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        if (strlen(line) == 1) {
            current_section++;
            section_row = 0;
            sections[current_section].length = 0;
            continue;
        }

        if (seed_count == 0) {
            unsigned long temp_values[MAX_SEEDS] = {0};
            seed_count = get_numbers(line, temp_values) / 2;
            for (int j=0; j<seed_count; j++) {
                seeds[j].initial_section = 0;
                seeds[j].start_value = temp_values[j * 2];
                seeds[j].end_value = seeds[j].start_value + temp_values[j * 2 + 1] - 1;
            }
            continue;
        }

        unsigned long data[3];
        int count = get_numbers(line, data);
        if (count == 0) continue;

        for (int j=0; j<3; j++) sections[current_section].data[section_row][j] = data[j];
        section_row++;
        sections[current_section].length++;
    }

    unsigned long result = ULONG_MAX;

    while (seed_count > 0)
    {
        struct seed s = seeds[seed_count - 1];

        unsigned long value = s.start_value;
        unsigned long top = s.end_value;
        seed_count--;

        for (int j=s.initial_section; j<SECTION_COUNT; j++) {
            struct tuple mapped = map_with_max(value, top, &sections[j]);
            if (mapped.b < top) {
                seeds[seed_count].start_value = mapped.b + 1;
                seeds[seed_count].end_value = top;
                seeds[seed_count].initial_section = j;

                seed_count++;
                top = mapped.b;
            }
            value = mapped.a;
            top = map(top, &sections[j]);
        }
        result = min(result, value);
    }

    printf("Second: %lu\n", result);
}

unsigned long map(unsigned long value, struct section *s) {
    for (int i=0; i<s->length; i++) {
        unsigned long start = s->data[i][1];
        if (value < start || value >= start + s->data[i][2]) continue;
        unsigned long d = start - s->data[i][0];
        return value - d;
    }
    return value;
}


struct tuple map_with_max(unsigned long value, unsigned long top, struct section *s) {
    unsigned long max_end = 0;
    unsigned long next_start = ULONG_MAX;
    struct tuple t;
    for (int i=0; i<s->length; i++) {
        unsigned long start = s->data[i][1];
        unsigned long end = start + s->data[i][2];

        if (start > value) {
            next_start = min(next_start, start);
        }
        max_end = max(max_end, end);

        if (value < start || value >= end) continue;
        unsigned long d = start - s->data[i][0];
        t.a = value - d;
        t.b = s->data[i][1] + s->data[i][2] - 1;
        return t;
    }

    t.a = value;
    if (value > max_end) {
        t.b = top;
    } else if (value < next_start) {
        t.b = next_start - 1;
    } else {
        t.b = value;
    }
    return t;
}


int get_numbers(char *line, unsigned long *out) {
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

        if (token_i > 0) {
            char *p;
            unsigned long value = strtoul(token, &p, 10);
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
