#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255
#define MAX_ROWS 255
#define INITIAL 200
#define TARGET 1000000000

void slide_v(
    char layout[MAX_ROWS][LINE_LENGTH],
    int rows,
    int cols,
    int dir
) {
    int min_r = 0;
    int max_r = rows;
    int r = 1;
    if (dir==1) {
        min_r = rows - 1;
        max_r = -1;
        r = rows - 2;
    }

    while (r != max_r) {
        for(int c = 0; c<cols; c++) {
            if (layout[r][c] != 'O') continue;
            int i = r+dir;
            while (1) {
                if (layout[i][c] != '.') break;
                layout[i-dir][c] = '.';
                layout[i][c] = 'O';
                if (i==min_r) break;
                i+=dir;
            }
        }
        r -= dir;
    }
}

void slide_h(
    char layout[MAX_ROWS][LINE_LENGTH],
    int rows,
    int cols,
    int dir
) {
    int min_c = 0;
    int max_c = cols;
    int c = 1;
    if (dir==1) {
        min_c = cols - 1;
        max_c = -1;
        c = cols - 2;
    }

    while (c != max_c) {
        for(int r = 0; r<rows; r++) {
            if (layout[r][c] != 'O') continue;
            int i = c+dir;
            while (1) {
                if (layout[r][i] != '.') break;
                layout[r][i-dir] = '.';
                layout[r][i] = 'O';
                if (i==min_c) break;
                i+=dir;
            }
        }
        c -= dir;
    }
}

int eq(
    char a[MAX_ROWS][LINE_LENGTH],
    char b[MAX_ROWS][LINE_LENGTH],
    int rows,
    int cols
) {
    for (int r=0; r<rows; r++) {
        for (int c=0; c<cols; c++) {
            if (a[r][c] != b[r][c]) return 0;
        }
    }
    return 1;
}

long score(
    char layout[MAX_ROWS][LINE_LENGTH],
    int rows,
    int cols
) {
    long sum = 0;
    for(int r = 0; r<rows; r++) {
        for(int c = 0; c<cols; c++) {
            if (layout[r][c] != 'O') continue;
            sum += rows-r;
        }
    }
    return sum;
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];
    char layout[MAX_ROWS][LINE_LENGTH] = {0};
    char origin[MAX_ROWS][LINE_LENGTH] = {0};
    int rows = 0;
    int cols = 0;

    // parse
    while (fgets(line, LINE_LENGTH, textfile)) {
        int l = strlen(line);
        if (l>1) { cols = l; } else { break; }

        for (int i=0; i<cols-1; i++) {
            layout[rows][i] = line[i];
        }
        rows++;
    }

    long counter = INITIAL;
    long cache[INITIAL] = {0};

    for (int i=0; i<counter; i++) {
        slide_v(layout, rows, cols, -1);
        slide_h(layout, rows, cols, -1);
        slide_v(layout, rows, cols, 1);
        slide_h(layout, rows, cols, 1);
        cache[i] = score(layout, rows, cols);
    }

    memcpy(origin, layout, MAX_ROWS * LINE_LENGTH * sizeof(char));

    while (1) {
        slide_v(layout, rows, cols, -1);
        slide_h(layout, rows, cols, -1);
        slide_v(layout, rows, cols, 1);
        slide_h(layout, rows, cols, 1);
        counter++;

        if (eq(layout, origin, rows, cols)) break;
    }

    long cycle = counter - INITIAL;
    long offset = (TARGET - counter) % cycle;
    long i = INITIAL - cycle + offset - 1;

    // print

    // for(int r = 0; r<rows; r++) {
    //     for(int c = 0; c<cols; c++) {
    //         printf("%c", layout[r][c]);
    //     }
    //     printf("\n");
    // }
    printf("Second: %ld\n", cache[i]);
}

void first(FILE* textfile) {
    char line[LINE_LENGTH];
    char layout[MAX_ROWS][LINE_LENGTH] = {0};
    int rows = 0;
    int cols = 0;

    // parse
    while (fgets(line, LINE_LENGTH, textfile)) {
        int l = strlen(line);
        if (l>1) { cols = l; } else { break; }

        for (int i=0; i<cols-1; i++) {
            layout[rows][i] = line[i];
        }
        rows++;
    }

    // slide
    slide_v(layout, rows, cols, -1);

    // count

    long sum = score(layout, rows, cols);

    // print

    // for(int r = 0; r<rows; r++) {
    //     for(int c = 0; c<cols; c++) {
    //         printf("%c", layout[r][c]);
    //     }
    //     printf("\n");
    // }
    printf("First: %ld\n", sum);
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