#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <time.h>
#define STB_DS_IMPLEMENTATION
#include "stb_ds.h"

#define LINE_LENGTH 255
#define MAX_GROUPS 50
#define SEED 12049790

struct kv {
    size_t key;
    long value;
};

void solve(FILE*, int);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    solve(textfile, 1);
    rewind(textfile);
    solve(textfile, 5);
    return 0;
}


void read_layout(
    char* line,
    int layout[LINE_LENGTH],
    int* layout_length,
    int groups[MAX_GROUPS][2],
    int* group_count
) {
    char token[LINE_LENGTH] = {0};
    int token_i = 0;
    *group_count = 0;
    *layout_length = 0;

    for (int i=0; i<LINE_LENGTH; i++) {
        char c = line[i];

        if (c == '?') layout[i] = 1;
        if (c == '#') layout[i] = 2;
        if (c == '.') layout[i] = 0;

        if (c == ' ') {
            *layout_length = i;
            continue;
        }

        if (*layout_length == 0) continue;

        if (c >= 0x30 && c <= 0x39) {
            token[token_i] = c;
            token_i++;
            continue;
        }

        token[token_i] = 0;
        groups[*group_count][0] = atoi(token);      

        token_i = 0;
        (*group_count)++;
        if (c == '\n') break;
    }
}

int test_group(
    int layout[LINE_LENGTH],
    int layout_length,
    int len,
    int offset
) {
    if (offset > 0) {
        if (layout[offset-1] == 2) return 0;
    }
    if (offset+len <= layout_length) {
        if (layout[offset+len] == 2) return 0;
    }
    for (int i=0; i<=len-1; i++) {
        if (layout[i+offset] == 0) return 0;
    }
    return 1;
}

void fit_first(
    int layout[LINE_LENGTH],
    int layout_length,
    int groups[MAX_GROUPS][2],
    int group_count,
    int group_idx,
    int offset
) {
    if (group_idx == group_count) return;

    int len = groups[group_idx][0];

    for (int i=offset; i<=layout_length-len; i++) {
        if (!test_group(layout, layout_length, len, i)) continue;

        groups[group_idx][1] = i;
        fit_first(
            layout,
            layout_length,
            groups,
            group_count,
            group_idx+1,
            i+len+1
        );
        break;
    }
}

long bubble_up(
    int layout[LINE_LENGTH],
    int layout_length,
    int base_groups[MAX_GROUPS][2],
    int base_occupancy[LINE_LENGTH],
    int group_count,
    int group_idx,
    int offset,
    struct kv** cache
) {
    // for (int i=0; i<layout_length; i++) printf("%d", base_occupancy[i]);
    // printf("\n");

    if (group_idx == -1) {
        // for (int i=0; i<layout_length; i++) {
        //     if (layout[i]==2 && base_occupancy[i]==0) return 0;
        // }
        // for (int i=0; i<layout_length; i++) printf("%d", base_occupancy[i]);
        // printf("\n");
        return 1;
    }
    long counter = 0;

    int len = base_groups[group_idx][0];
    int base = base_groups[group_idx][1];

    for (int i=base; i<=offset-len; i++) {
        int invalid = 0;
        for (int j=i+len; j<layout_length; j++) {
            if (layout[j] == 2 && base_occupancy[j] == 0) invalid = 1;
        }
        if (group_idx == 0) {
            for (int j=0; j<i; j++) {
                if (layout[j] == 2) invalid = 1;
            }
        }
        if (invalid) continue;
        if (!test_group(layout, layout_length, len, i)) continue;

        if (group_idx < group_count - 1) {
            if (base_groups[group_idx+1][1] == i+len) continue;
        }

        int groups[MAX_GROUPS][2] = {0};
        memcpy(groups, base_groups, 2 * group_count * sizeof(int));
        groups[group_idx][1] = i;

        int occupancy[LINE_LENGTH] = {0};
        memcpy(occupancy, base_occupancy, layout_length * sizeof(int));
        for (int j=i; j<i+len; j++) occupancy[j] = 1;
        // printf("Down at: %d %d\n", group_idx, i);

        int key[2] = {group_idx, i};
        size_t hash = stbds_hash_bytes(key, 2 * sizeof(int), SEED);
        size_t cached_i = hmgeti(*cache, hash);
        if (cached_i != -1) {
            counter += hmget(*cache, hash);
        } else {
            long sub = bubble_up(
                layout,
                layout_length,
                groups,
                occupancy,
                group_count,
                group_idx-1,
                i,
                cache
            );
            hmput(*cache, hash, sub);
            counter += sub;
        }
        // counter += bubble_up(
        //     layout,
        //     layout_length,
        //     groups,
        //     occupancy,
        //     group_count,
        //     group_idx-1,
        //     i,
        //     cache
        // );
    }
    return counter;
}

void solve(FILE* textfile, int iter) {
    char line[LINE_LENGTH];

    long long sum = 0;
    int counter = -1;

    clock_t time = clock();

    while (fgets(line, LINE_LENGTH, textfile)) {
        counter++;
        // if (counter!=1) continue;
        int base_layout[LINE_LENGTH] = {0};
        int layout_length = 0;
        int base_groups[MAX_GROUPS][2] = {0};
        int group_count = 0;
        read_layout(line, base_layout, &layout_length, base_groups, &group_count);

        for (int i=1; i<iter; i++) {
            for (int j=0; j<group_count; j++) {
                base_groups[i * group_count + j][0] = base_groups[j][0];
                base_groups[i * group_count + j][1] = 0;
            }
            base_layout[i * (layout_length + 1) - 1] = 1;
            for (int j=0; j<layout_length; j++) {
                base_layout[i * (layout_length + 1) + j] = base_layout[j];
            }
        }
        group_count *= iter;
        layout_length *= iter;
        layout_length += iter - 1;

        // for (int i=0; i<layout_length; i++) printf("%d", base_layout[i]);
        // printf("\n");

        int layout[LINE_LENGTH] = {0};
        memcpy(layout, base_layout, layout_length * sizeof(int));
        int groups[MAX_GROUPS][2] = {0};
        memcpy(groups, base_groups, 2 * group_count * sizeof(int));

        // for (int i=0; i<group_count; i++) printf("%d", groups[i]);
        // printf("\n");

        fit_first(
            layout,
            layout_length,
            groups,
            group_count,
            0,
            0
        );

        // for (int i=0; i<group_count; i++) printf("%d|%d ", groups[i][0], groups[i][1]);
        // printf("\n");

        int occupancy[LINE_LENGTH] = {0};

        struct kv* cache = NULL;

        long sub = bubble_up(
            layout,
            layout_length,
            groups,
            occupancy,
            group_count,
            group_count-1,
            layout_length+1,
            &cache
        );

        // printf("Counter: %d %ld\n\n", counter, sub);
        sum += sub;
        hmfree(cache);
        // break;
    }
    printf("Time: %f\n", (double)(clock() - time) / CLOCKS_PER_SEC);
    printf("Result: %lld\n", sum);
}
