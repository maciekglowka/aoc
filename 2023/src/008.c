#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STB_DS_IMPLEMENTATION
#include "stb_ds.h"

#define LINE_LENGTH 512
#define SEED 12049790

struct node {
    size_t left;
    size_t right;
};
struct kv {
    size_t key;
    struct node value;
};

void first(FILE*);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    first(textfile);
    // rewind(textfile);
    // second(textfile);
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
    // int commands[LINE_LENGTH];
    // int commands_count;

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
        hmput(*dict, hash, n);
        // printf("%s %s %s\n", key, leaf_a, leaf_b);
        // printf("Inserting: %s %lu\n", key, hash);
    }
    // for (int i=0; i<commands_count; i++) printf("%d", commands[i]);
    // printf("\n");
}

void first(FILE* textfile) {
    struct kv *data = NULL;
    int commands[LINE_LENGTH];
    int command_count;
    size_t first = stbds_hash_string("AAA", SEED);
    size_t target = stbds_hash_string("ZZZ", SEED);
    // printf("Target: %lu\n", target);
    parse_input(textfile, &data, commands, &command_count);
    // printf("%lu\n", hmlen(data));
    // printf("%d\n", command_count);
    // for (size_t i=0; i<hmlen(data); i++) {
    //     printf("%lu\n", data[i].key);
    // }
    size_t cur = first;
    long command_idx = 0;
    long steps = 0;
    while (cur != target) {
        struct node n = hmget(data, cur);
        // printf("%lu, %lu", n.left, n.right);
        // break;
        if (commands[command_idx] == 0) {
            cur = n.left;
        } else {
            cur = n.right;
        }
        command_idx = (command_idx + 1) % command_count;
        // printf("%lu\n", cur);
        steps++;
    }
    printf("First: %ld\n", steps);
}