#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define LINE_LENGTH 255
#define MAX_GROUPS 50

void first(FILE*);
void second(FILE*);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    // first(textfile);
    // rewind(textfile);
    second(textfile);
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
        groups[*group_count][1] = *group_count;      
        token_i = 0;
        (*group_count)++;
        if (c == '\n') break;
    }
}

// void get_clusters(
//     int layout[LINE_LENGTH],
//     int layout_length,
//     int clusters[LINE_LENGTH][2],
//     int* cluster_count
// ) {
//     int start = -1;
//     *cluster_count = 0;
//     int len = 0;
//     for (int i=0; i<layout_length; i++) {
//         if (layout[i] > 0) {
//             if (start == -1) start = i;
//             len++;
//             if (i < layout_length - 1) continue;
//         }
//         if (start > -1) {
//             clusters[*cluster_count][0] = start;
//             clusters[*cluster_count][1] = len;
//             start = -1;
//             len = 0;
//             (*cluster_count)++;
//         }
//     }
// }

int cmp_groups(const void *a, const void *b) {
    return ((int*)b)[0] - ((int*)a)[0];
}

int test_group(
    int layout[LINE_LENGTH],
    int layout_length,
    int occupancy[LINE_LENGTH],
    int len,
    int offset
) {
    // printf("offset: %d\n", offset);
    for (int i=0; i<=len-1; i++) {
        if (layout[i+offset] == 0) return 0;
        if (occupancy[i+offset] != 0) return 0;
    }
    return 1;
}

long fit(
    int layout[LINE_LENGTH],
    int layout_length,
    int occupancy[LINE_LENGTH],
    int order[MAX_GROUPS][3],
    int groups[MAX_GROUPS][2],
    int group_count,
    int group_idx
) {
    if (group_idx == group_count) {
        for (int i=0; i<layout_length; i++) {
            if (layout[i]==2 && occupancy[i]==0) return 0;
        }
        // for (int i=0; i<layout_length; i++) printf("%d", occupancy[i]);
        // printf("\n");
        return 1;
    }
    long sum = 0;
    int len = groups[group_idx][0];
    int seq = groups[group_idx][1];

    int i_min = 0;
    int i_max = layout_length-len;

    for (int i=0; i<group_idx; i++) {
        // printf("%d|%d|%d ", order[i][0], order[i][1], order[i][2]);
        if (seq < order[i][2]) i_max = min(i_max, order[i][0] - len - 1);
        if (seq > order[i][2]) i_min = max(i_min, order[i][1] + 2);
    }
    // printf("\n");
    // printf("idx: %d len: %d, min: %d, max: %d\n", group_idx, len, i_min, i_max);
    for (int i=i_min; i<=i_max; i++) {
        // int seq_check = 1;
        // for (int j=0; j<group_idx; j++) {
        //     if (seq < order[j][2]) {
        //         if (i+len >= order[j][0]) { printf("s0"); seq_check = 0; }
        //     } else {
        //         if (i <= order[j][1]) { printf("s1"); seq_check = 0; }
        //     }
        // }
        // if (!seq_check) continue;

        if (i>0) {
            if (layout[i-1] == 2) continue;
        }
        if (i+len<layout_length-1) {
            if (layout[i+len] == 2) continue;
        }
        // printf("Testing\n");
        if (!test_group(
            layout,
            layout_length,
            occupancy,
            len,
            i
        )) continue;
        // printf("Tested\n");
        int oc[LINE_LENGTH] = {0};
        for (int j=0; j<layout_length; j++) {
            if (j < i || j > i + len - 1) {
                oc[j] = occupancy[j];
            } else {
                oc[j] = 1;
            }
            // printf("%d", oc[j]);
        }

        // printf("Idx: %d, I: %d\n", group_idx, i);
        // for (int i=0; i<layout_length; i++) printf("%d", oc[i]);
        // printf("\n");

        int or[MAX_GROUPS][3] = {0};
        for (int j=0; j<group_idx; j++) {
            or[j][0] = order[j][0];
            or[j][1] = order[j][1];
            or[j][2] = order[j][2];
        }
        or[group_idx][0] = i;
        or[group_idx][1] = i+len-1;
        or[group_idx][2] = seq;

        // printf("--%d\n", i);
        // printf("--%ld\n", sum);

        sum += fit(
            layout,
            layout_length,
            oc,
            or,
            groups,
            group_count,
            group_idx+1
        );
    }
    // printf("%ld\n", sum);
    return sum;
}

void first(FILE* textfile) {
    char line[LINE_LENGTH];

    long long sum = 0;
    int counter = -1;

    while (fgets(line, LINE_LENGTH, textfile)) {
        counter++;
        // if (counter>2) break;
        int layout[LINE_LENGTH] = {0};
        int layout_length = 0;
        int groups[MAX_GROUPS][2] = {0};
        int group_count = 0;
        read_layout(line, layout, &layout_length, groups, &group_count);

        // int clusters[LINE_LENGTH][2] = {0};
        // int cluster_count = 0;

        // get_clusters(layout, layout_length, clusters, &cluster_count);

        for (int i=0; i<layout_length; i++) printf("%d", layout[i]);
        printf("\n");
        // for (int i=0; i<cluster_count; i++) printf("%d|%d ", clusters[i][0], clusters[i][1]);
        // printf("\n");
        // for (int i=0; i<group_count; i++) printf("%d|%d ", groups[i][0], groups[i][1]);
        // printf("\n\n");

        qsort(groups, group_count, 2 * sizeof(int), cmp_groups);
        int occupancy[LINE_LENGTH] = {0};
        int order[MAX_GROUPS][3] = {0};

        long sub = fit(
            layout,
            layout_length,
            occupancy,
            order,
            groups,
            group_count,
            0
        );
        printf("%ld\n\n", sub);
        sum += sub;
        // for (int i=0; i<group_count; i++) printf("%d|%d ", groups[i][0], groups[i][1]);
        // printf("\n\n");

        // for (int i=0; i<group_count; i++) {

        // }
        // break;
    }
    printf("First: %lld\n", sum);
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];

    long long sum = 0;
    int counter = -1;

    while (fgets(line, LINE_LENGTH, textfile)) {
        counter++;
        // if (counter!=1) continue;;
        int layout[LINE_LENGTH] = {0};
        int layout_length = 0;
        int groups[MAX_GROUPS][2] = {0};
        int group_count = 0;
        read_layout(line, layout, &layout_length, groups, &group_count);

        int iter = 5;

        for (int i=1; i<iter; i++) {
            for (int j=0; j<group_count; j++) {
                groups[i * group_count + j][0] = groups[j][0];
                groups[i * group_count + j][1] = groups[j][1] + i * iter + 1;
            }
            layout[i * (layout_length + 1) - 1] = 1;
            for (int j=0; j<layout_length; j++) {
                layout[i * (layout_length + 1) + j] = layout[j];
            }
        }
        group_count *= iter;
        layout_length *= iter;
        layout_length += iter - 1;

        // for (int i=0; i<group_count; i++) printf("%d|%d ", groups[i][0], groups[i][1]);
        // printf("\n");
        // for (int i=0; i<layout_length; i++) printf("%d", layout[i]);
        // printf("\n");

        qsort(groups, group_count, 2 * sizeof(int), cmp_groups);

        int occupancy[LINE_LENGTH] = {0};
        int order[MAX_GROUPS][3] = {0};

        long sub = fit(
            layout,
            layout_length,
            occupancy,
            order,
            groups,
            group_count,
            0
        );
        printf("%ld\n\n", sub);
        sum += sub;
        // break;
    }
    printf("Second: %lld\n", sum);
}

// void second(FILE* textfile) {
//     char line[LINE_LENGTH];

//     unsigned long long sum = 0;
//     int counter = -1;

//     while (fgets(line, LINE_LENGTH, textfile)) {
//         counter++;
//         printf("Counter: %d ", counter);
//         // if (counter>2) break;
//         int layout[LINE_LENGTH] = {0};
//         int layout_length = 0;
//         int groups[MAX_GROUPS][2] = {0};
//         int group_count = 0;
//         read_layout(line, layout, &layout_length, groups, &group_count);

//         int iter = 5;
//         printf("Iter: %d\n", iter);

//         for (int i=1; i<iter; i++) {
//             for (int j=0; j<group_count; j++) {
//                 groups[i * group_count + j][0] = groups[j][0];
//                 groups[i * group_count + j][1] = groups[j][1] + i * iter + 1;
//             }
//             layout[i * (layout_length + 1) - 1] = 1;
//             for (int j=0; j<layout_length; j++) {
//                 layout[i * (layout_length + 1) + j] = layout[j];
//             }
//         }
//         group_count *= iter;
//         layout_length *= iter;
//         layout_length += iter - 1;

//         int base_groups[MAX_GROUPS][2] = {0};
//         // memcmp(half_groups, groups, group_count * sizeof(int));
//         for (int i=0; i<group_count/iter; i++) {base_groups[i][0] = groups[i][0]; base_groups[i][1] = groups[i][1]; }

//         qsort(groups, group_count, 2 * sizeof(int), cmp_groups);
//         qsort(base_groups, group_count/iter, 2 * sizeof(int), cmp_groups);

//         // for (int i=0; i<group_count; i++) printf("%d|%d ", groups[i][0], groups[i][1]);
//         // printf("\n");
//         // for (int i=0; i<group_count/iter; i++) printf("%d|%d ", base_groups[i][0], base_groups[i][1]);
//         // printf("\n");

//         int occupancy[LINE_LENGTH] = {0};
//         int order[MAX_GROUPS][3] = {0};

//         long tests_base = fit(
//             layout,
//             (layout_length-1)/iter,
//             occupancy,
//             order,
//             base_groups,
//             group_count/iter,
//             0
//         );

//         // printf("%ld\n", tests_half);
//         for (int i=0; i<LINE_LENGTH; i++) occupancy[LINE_LENGTH] = 0;
//         for (int i=0; i<MAX_GROUPS; i++) { order[i][0] = 0; order[i][1] = 0; order[i][2] = 0; }

//         long tests = fit(
//             layout,
//             layout_length,
//             occupancy,
//             order,
//             groups,
//             group_count,
//             0
//         );

//         long long sub = tests * tests / tests_base;
//         // long a = tests / tests_base;
//                 // long long sub = (long long)pow(tests, 4) / tests_base;
//         // long long sub = (long long)tests * (long long)pow(a, 3);
//         // printf("%lld\n", (long long)pow(a, 4));
//         // long long sub = (long long)pow(a, 4) * (long long)tests_base;
//         printf("Tb:%ld T:%ld Total: %lld\n", tests_base, tests, sub);
//         // printf("%lld\n\n", sub);
//         // sum += sub;
//         printf("%ld\n", tests % tests_base);
//         if (tests % tests_base) sum++;
//     }
//     printf("Second: %llu\n", sum);
// }
