#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define LINE_LENGTH 255

void first(FILE*);
void second(FILE*);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    first(textfile);
    // rewind(textfile);
    // second(textfile);
    return 0;
}

int max(int a, int b) {
    if (a > b) return a;
    return b;
}

int min(int a, int b) {
    if (a < b) return a;
    return b;
}

void read_layout(
    char* line,
    int layout[LINE_LENGTH],
    int* layout_length,
    int groups[LINE_LENGTH],
    int* group_count
) {
    char token[LINE_LENGTH] = {0};
    int token_i = 0;
    *group_count = 0;
    *layout_length = 0;

    for (int i=0; i<LINE_LENGTH; i++) {
        char c = line[i];

        if (c == '?') layout[i] = -1;
        if (c == '#') layout[i] = 0;
        if (c == '.') layout[i] = 1;

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
        groups[*group_count] = atoi(token);      
        token_i = 0;
        *group_count += 1;
        if (c == '\n') break;
    }
}

int test_layout(
    int layout[LINE_LENGTH],
    int layout_length,
    int groups[LINE_LENGTH],
    int group_count
) {
    int group_idx = 0;
    int group = 0;
    // printf("\n");
    for (int i=0; i<layout_length; i++) {
        if (layout[i] == 0) { 
            group++;
            if (i!=layout_length-1) continue;
        }
        if (group != 0) {
            // printf("G: %d, I: %d, GG: %d\n", group, group_idx, groups[group_idx]);
            if (groups[group_idx] != group) return 0;
            group_idx++;
            group = 0;
        }
    }

    // printf("OUT\n");
    if (group_idx != group_count) return 0;
    return 1;
}

int test_replacements(
    int layout[LINE_LENGTH],
    int layout_length,
    int groups[LINE_LENGTH],
    int group_count,
    int unknowns[LINE_LENGTH],
    int unknown_count,
    int unknown_offset
) {
    // for (int i=0; i<unknown_count; i++) printf("%d ", unknowns[i]);
    // printf("\n");
    // for (int i=0; i<layout_length; i++) printf("%d ", layout[i]);
    // printf("\n");
    int tests = 0;
    for (__uint128_t i=0; i<(__uint128_t)pow(2, (__uint128_t)unknown_count); i++) {
        int tmp[LINE_LENGTH] = {0};
        memcpy(tmp, layout, layout_length * sizeof(int));
        for (int j=0; j<unknown_count; j++) {
            int v = 0;
            if (i & (1 << j)) v = 1;
            tmp[unknowns[j] - unknown_offset] = v;
        }
        tests += test_layout(tmp, layout_length, groups, group_count);
    }
    return tests;
}

void first(FILE* textfile) {
    char line[LINE_LENGTH];

    int sum = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int layout[LINE_LENGTH] = {0};
        int layout_length = 0;
        int groups[LINE_LENGTH] = {0};
        int group_count = 0;
        read_layout(line, layout, &layout_length, groups, &group_count);

        int unknown_count = 0;
        int unknowns[LINE_LENGTH] = {0};
        for (int i=0; i<layout_length; i++) {
            if (layout[i] != -1) continue;
            unknowns[unknown_count] = i;
            unknown_count++;
        }

        int sub = test_replacements(
            layout,
            layout_length,
            groups,
            group_count,
            unknowns,
            unknown_count,
            0
        );
        printf("%d\n", sub);
        sum += sub;
    }
    printf("First: %d\n", sum);
}


// void second(FILE* textfile) {
//     char line[LINE_LENGTH];
//     long long sum = 0;

//     while (fgets(line, LINE_LENGTH, textfile)) {
//         int layout[LINE_LENGTH] = {0};
//         int layout_length = 0;
//         int groups[LINE_LENGTH] = {0};
//         int group_count = 0;

//         read_layout(line, &layout[1], &layout_length, groups, &group_count);

//         int unknowns[LINE_LENGTH] = {0};
//         int unknown_count = 0;

//         for (int i=1; i<=layout_length; i++) {
//             if (layout[i] != -1) continue;
//             unknowns[unknown_count + 1] = i;
//             unknown_count++;
//         }
//         unknowns[0] = 0;
//         unknowns[unknown_count + 1] = layout_length + 1;
//         // for (int i=0; i<unknown_count+2; i++) printf("%d ", unknowns[i]);
//         // printf("\n");

//         int base = test_replacements(
//             &layout[1],
//             layout_length,
//             groups,
//             group_count,
//             &unknowns[1],
//             unknown_count,
//             1
//         );
//         int r = base;
//         if (layout[1] != 0) {
//             r = test_replacements(
//                 &layout[1],
//                 layout_length+1,
//                 groups,
//                 group_count,
//                 &unknowns[1],
//                 unknown_count+1,
//                 1
//             );
//         }
//         int l = base;
//         if (layout[layout_length] != 0) {
//             l = test_replacements(
//                 layout,
//                 layout_length+1,
//                 groups,
//                 group_count,
//                 unknowns,
//                 unknown_count+1,
//                 0
//             );
//         }

//         printf("T:%d Tr:%d Tl: %d\n", base, r, l);

//         // int a = tests / tests_half;
//         long long total = base * (long long)pow(max(r, l), 4);
//         printf("%lld\n", total);
//         // printf("Th:%d T:%d Total: %d\n", tests_half, tests, total);
//         // printf("\n\n");
//         sum += total;
//     }
//     printf("Second: %lld\n", sum);
// }


void second(FILE* textfile) {
    char line[LINE_LENGTH];
    int counter = 0;

    long long sum = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        counter++;
        // if (counter > 2) continue;
        int layout[LINE_LENGTH] = {0};
        int layout_length = 0;
        int groups[LINE_LENGTH] = {0};
        int group_count = 0;
        read_layout(line, layout, &layout_length, groups, &group_count);

        for (int i=1; i<2; i++) {
            for (int j=0; j<group_count; j++) {
                groups[i * group_count + j] = groups[j];
            }
            layout[i * (layout_length + 1) - 1] = -1;
            for (int j=0; j<layout_length; j++) {
                layout[i * (layout_length + 1) + j] = layout[j];
            }
        }
        group_count *= 2;
        layout_length *= 2;
        layout_length += 1;

        int unknown_count = 0;
        int unknowns[LINE_LENGTH] = {0};
        for (int i=0; i<layout_length; i++) {
            if (layout[i] != -1) continue;
            unknowns[unknown_count] = i;
            unknown_count++;
        }
        // for (int i=0; i<layout_length; i++) printf("%d ", layout[i]);
        // printf("\n");
        // for (int i=0; i<(layout_length-1)/2; i++) printf("%d ", layout[i]);
        // printf("\n");
        // for (int i=0; i<group_count; i++) printf("%d ", groups[i]);
        // printf("\n");
        // printf("%d %d %d\n", layout_length, group_count, unknown_count);

        // printf("T:%d Tr:%d Tl: %d\n", tests_base, tests_r, tests_l);

        int tests_half = test_replacements(
            layout,
            (layout_length-1)/2,
            groups,
            group_count/2,
            unknowns,
            (unknown_count-1)/2,
            0
        );
        int tests = test_replacements(
            layout,
            layout_length,
            groups,
            group_count,
            unknowns,
            unknown_count,
            0
        );

        // // printf("%s", line);
        // for (int i=0; i<layout_length; i++) printf("%d ", layout[i]);
        // printf("\n");
        // for (int i=0; i<group_count; i++) printf("%d ", groups[i]);
        // printf("\n");

        // int tests = 0;

        // for (__uint128_t i=0; i<(__uint128_t)pow(2, (__uint128_t)unknown_count); i++) {
        //     int tmp[LINE_LENGTH] = {0};
        //     memcpy(tmp, layout, layout_length * sizeof(int));
        //     for (int j=0; j<unknown_count; j++) {
        //         int v = 0;
        //         if (i & (1 << j)) v = 1;
        //         tmp[unknowns[j]] = v;
        //     }
        //     // for (int i=0; i<layout_length; i++) printf("%d ", tmp[i]);
        //     tests += test_layout(tmp, layout_length, groups, group_count);
        //     // printf("T:%d\n", tests);
        // }
        // // printf("L: %d\n", layout_length);
        // // printf("G: %d\n", group_count);
        // // for (int i=0; i<group_count; i++) printf("%d ", groups[i]);

        int a = tests / tests_half;
        long long total = (long long)tests * (long long)pow(a, 3);
        printf("Th:%d T:%d Total: %lld\n", tests_half, tests, total);
        // printf("\n\n");
        sum += total;
        // break;
    }
    printf("Second: %lld\n", sum);
}