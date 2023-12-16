#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STB_DS_IMPLEMENTATION
#include "stb_ds.h"

#define MAX_COLS 255
#define MAX_ROWS 255
#define MAX_BEAMS 255
#define SEED 12412

struct beam {
    int x;
    int y;
    int vx;
    int vy;
};

struct kv {
    size_t key;
    int value;
};

long max(long a, long b) {
    if (a > b) return a;
    return b;
}

int validate(
    struct beam cur,
    int rows,
    int cols
) {
    if (cur.x<0 || cur.y<0) return 0;
    if (cur.x>=cols || cur.y>=rows) return 0;
    return 1;
}

int get_next(
    struct beam cur,
    char layout[MAX_COLS][MAX_ROWS],
    int rows,
    int cols,
    struct beam out[2]
) {
    char c = layout[cur.x][cur.y];
    int cont = 0;
    int split = 0;
    int left = 0;
    int right = 0;

    if (c == '.') cont = 1;
    if (c == '-' && cur.vy == 0) cont = 1;
    if (c == '|' && cur.vx == 0) cont = 1;
    if (c == '-' && cur.vy != 0) split = 1;
    if (c == '|' && cur.vx != 0) split = 1;

    if (c == '/') {
        if (cur.vx != 0) {
            left = 1;
        } else {
            right = 1;
        }
    }
    if (c == '\\') {
        if (cur.vx != 0) {
            right = 1;
        } else {
            left = 1;
        }
    }

    if (left) {
        out[0].vx = cur.vy;
        out[0].vy = -cur.vx;
        out[0].x = cur.x + out[0].vx;
        out[0].y = cur.y + out[0].vy;
        return 1;
    }
    if (right) {
        out[0].vx = -cur.vy;
        out[0].vy = cur.vx;
        out[0].x = cur.x + out[0].vx;
        out[0].y = cur.y + out[0].vy;
        return 1;
    }
    if (cont) {
        out[0].x = cur.x + cur.vx;
        out[0].y = cur.y + cur.vy;
        out[0].vx = cur.vx;
        out[0].vy = cur.vy;
        return 1;
    }
    if (split) {
        out[0].vx = -cur.vy;
        out[0].vy = cur.vx;
        out[0].x = cur.x + out[0].vx;
        out[0].y = cur.y + out[0].vy;

        out[1].vx = cur.vy;
        out[1].vy = -cur.vx;
        out[1].x = cur.x + out[1].vx;
        out[1].y = cur.y + out[1].vy;
        return 2;
    }

    return 0;
}

long get_energized(
    char layout[MAX_COLS][MAX_ROWS],
    int rows,
    int cols,
    int x,
    int y,
    int vx,
    int vy
) {
    struct beam beams[MAX_BEAMS] = {0};
    int beams_count = 1;
    beams[0].x = x;
    beams[0].y = y;
    beams[0].vx = vx;
    beams[0].vy = vy;

    int energized[MAX_COLS][MAX_ROWS] = {0};
    struct kv* cache = NULL;

    while (beams_count) {
        beams_count--;
        size_t hash = stbds_hash_bytes(&beams[beams_count], sizeof(struct beam), SEED);
        if (hmgeti(cache, hash) != -1) continue;
        hmput(cache, hash, 0);

        energized[beams[beams_count].x][beams[beams_count].y] = 1;
        struct beam next[2];
        int next_count = get_next(
            beams[beams_count],
            layout,
            rows,
            cols,
            next
        );
        for (int i=0; i<next_count; i++) {
            if (!validate(next[i], rows, cols)) continue;
            memcpy(&beams[beams_count], &next[i], sizeof(struct beam));
            beams_count++;
        }
    }
    long sum = 0;
    for (int r=0; r<rows; r++) {
        for (int c=0; c<cols; c++) {
            // printf("%d", energized[c][r]);
            if (energized[c][r]) sum++;
        }
        // printf("\n");
    }
    hmfree(cache);
    return sum;
}

void first(FILE* textfile) {
    char line[MAX_COLS];
    char layout[MAX_COLS][MAX_ROWS] = {0};

    int rows = 0;
    int cols = 0;

    while (fgets(line, MAX_COLS+1, textfile)) {
        for (int i=0; i<MAX_COLS+1; i++) {
            char c = line[i];
            if (c == '\n') {
                break;
            }
            layout[i][rows] = c;
            cols = i+1;
        }
        rows++;
    }

    // for (int r=0; r<rows; r++) {
    //     for (int c=0; c<cols; c++) {
    //         printf("%c", layout[c][r]);
    //     }
    //     printf("\n");
    // }
    long sum = get_energized(
        layout,
        rows,
        cols,
        0,
        0,
        1,
        0
    );
    printf("First: %ld\n", sum);
}

void second(FILE* textfile) {
    char line[MAX_COLS];
    char layout[MAX_COLS][MAX_ROWS] = {0};

    int rows = 0;
    int cols = 0;

    while (fgets(line, MAX_COLS+1, textfile)) {
        for (int i=0; i<MAX_COLS+1; i++) {
            char c = line[i];
            if (c == '\n') {
                break;
            }
            layout[i][rows] = c;
            cols = i+1;
        }
        rows++;
    }

    long sum = 0;

    for (int x=0; x<rows; x++) {
        long sub_0 = get_energized(
            layout,
            rows,
            cols,
            x,
            0,
            0,
            1
        );
        long sub_1 = get_energized(
            layout,
            rows,
            cols,
            x,
            rows-1,
            0,
            -1
        );
        sum = max(sum, max(sub_0, sub_1));
    }
    for (int y=0; y<cols; y++) {
        long sub_0 = get_energized(
            layout,
            rows,
            cols,
            0,
            y,
            1,
            0
        );
        long sub_1 = get_energized(
            layout,
            rows,
            cols,
            cols-1,
            y,
            -1,
            0
        );
        sum = max(sum, max(sub_0, sub_1));
    }

    printf("Second: %ld\n", sum);
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