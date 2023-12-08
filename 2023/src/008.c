#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STB_DS_IMPLEMENTATION
#include "stb_ds.h"

#define LINE_LENGTH 512
#define SEED 12049790

struct node {
    char literal[3];
    size_t left;
    size_t right;
};
struct kv {
    size_t key;
    struct node value;
};

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

void parse_input(
    FILE* textfile,
    struct kv** dict,
    int* commands,
    int* command_count
) {
    char line[LINE_LENGTH];
    int counter = -1;

    while (fgets(line, LINE_LENGTH, textfile)) {
        counter++;
        if (counter == 0) {
            *command_count = strlen(line) - 1;
            for (int i=0; i<*command_count; i++) {
                commands[i] = (line[i] == 'L') ? 0 : 1;
            }
            continue;
        }
        if (counter == 1) continue;

        char key[4] = {line[0], line[1], line[2], 0};
        char leaf_a[4] = {line[7], line[8], line[9], 0};
        char leaf_b[4] = {line[12], line[13], line[14], 0};
        size_t hash = stbds_hash_string(key, SEED);
        struct node n = {
            left: stbds_hash_string(leaf_a, SEED),
            right: stbds_hash_string(leaf_b, SEED)
        };
        for (int i=0; i<3; i++) n.literal[i] = key[i];
        hmput(*dict, hash, n);
    }
}

long lcm_2(long a, long b) {
    // gcd
    long gcd_a = a;
    long gcd_b = b;
    while (gcd_b) {
        long t = gcd_b;
        gcd_b = gcd_a % gcd_b;
        gcd_a = t;
    }
    // printf("GCD: %ld\n", gcd_a);

    return a * b / gcd_a;
}

long lcm(long* numbers, int count) {
    long lcm = numbers[0];
    for (int i=1; i<count; i++) {
        lcm = lcm_2(lcm, numbers[i]);
    }
    return lcm;
}


void first(FILE* textfile) {
    struct kv *data = NULL;
    int commands[LINE_LENGTH];
    int command_count;
    size_t first = stbds_hash_string("AAA", SEED);
    size_t target = stbds_hash_string("ZZZ", SEED);
    parse_input(textfile, &data, commands, &command_count);

    size_t cur = first;
    long command_idx = 0;
    long steps = 0;
    while (cur != target) {
        struct node n = hmget(data, cur);

        if (commands[command_idx] == 0) {
            cur = n.left;
        } else {
            cur = n.right;
        }
        command_idx = (command_idx + 1) % command_count;
        steps++;
    }
    printf("First: %ld\n", steps);
}

void second(FILE* textfile) {
    struct kv *data = NULL;
    int commands[LINE_LENGTH];
    int command_count;
    parse_input(textfile, &data, commands, &command_count);

    int start_count = 0;
    int end_count = 0;
    size_t starts[10] = {0};
    size_t ends[10] = {0};

    for (size_t i=0; i<hmlen(data); i++) {
        if (data[i].value.literal[2] == 'A') {
            starts[start_count] = data[i].key;
            start_count++;
        }
        if (data[i].value.literal[2] == 'Z') {
            ends[end_count] = data[i].key;
            end_count++;
        }
    }

    long steps[10] = {0};
    for (int i=0; i<start_count; i++) {
        size_t cur = starts[i];
        long command_idx = 0;
        while (1) {
            struct node n = hmget(data, cur);
            if (n.literal[2] == 'Z') break;

            if (commands[command_idx] == 0) {
                cur = n.left;
            } else {
                cur = n.right;
            }
            command_idx = (command_idx + 1) % command_count;
            steps[i]++;
        }
    }
    long result = lcm(steps, start_count);
    printf("Second: %ld\n", result);
}