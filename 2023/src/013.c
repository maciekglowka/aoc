#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_COLS 32
#define MAX_ROWS 32

int check_reflection_v(
    char layout[MAX_ROWS][MAX_COLS],
    int rows,
    int start
) {
    int offset = 0;
    while(1) {
        for (int i=0; i<rows; i++) {
            if (layout[i][start+offset]!=layout[i][start-offset-1]) return 0;
        }
        offset++;
        if (start-offset == 0) break;
        if (layout[0][start+offset] == 0) break;
    }
    return 1;
}

int check_reflection_h(
    char layout[MAX_ROWS][MAX_COLS],
    int rows,
    int start
) {
    int offset = 0;
    while(1) {
        if (strcmp(layout[start+offset], layout[start-offset-1]) != 0) return 0;
        offset++;
        if (start-offset == 0) break;
        if (start+offset == rows) break;
    }
    return 1;
}

int get_reflection_v(
    char layout[MAX_ROWS][MAX_COLS],
    int rows
) {
    int c = 1;
    while (1) {
        if (check_reflection_v(layout, rows, c)) return c;
        c++;
        if (layout[0][c] == 0) break;
    }
    return -1;
}

int get_reflection_h(
    char layout[MAX_ROWS][MAX_COLS],
    int rows
) {
    for (int r=1; r<rows; r++) {
        if (check_reflection_h(layout, rows, r)) return r;
    }
    return -1;
}

void first(FILE* textfile) {
    char line[MAX_COLS];
    long sum = 0;

    while (1) {
        char layout[MAX_ROWS][MAX_COLS] = {0};

        int rows = 0;
        int cols = 0;

        while (fgets(line, MAX_COLS+1, textfile)) {
            int len = strlen(line);
            if (len<2) break;
            strcpy(layout[rows], line);
            layout[rows][len-1] = 0;

            rows++;
        }

        int h = get_reflection_h(layout, rows); 
        int v = get_reflection_v(layout, rows); 
        printf("H: %d V: %d\n", h, v);

        if (v!=-1) sum += v;
        if (h!=-1) sum += 100 * h;

        char *has_next = fgets(line, MAX_COLS+1, textfile);
        if (has_next) {
            int rewind = strlen(line);
            fseek(textfile, -rewind, SEEK_CUR);
        } else {
            break;
        }
    }
    printf("First: %ld\n", sum);
}

// NO DRY

int check_smudge_v(
    char layout[MAX_ROWS][MAX_COLS],
    int rows,
    int start
) {
    int offset = 0;
    int diff = 0;
    while(1) {
        for (int i=0; i<rows; i++) {
            if (layout[i][start+offset]!=layout[i][start-offset-1]) diff++;
        }
        offset++;
        if (diff>1) return 0;
        if (start-offset == 0) break;
        if (layout[0][start+offset] == 0) break;
    }
    return diff == 1;
}

int check_smudge_h(
    char layout[MAX_ROWS][MAX_COLS],
    int rows,
    int start
) {
    int offset = 0;
    int diff = 0;
    while(1) {
        int i = 0;
        while(1) {
            if (layout[start+offset][i] != layout[start-offset-1][i]) diff++;
            i++;
            if (layout[0][i] == 0) break;
        }
        offset++;
        if (diff>1) return 0;
        if (start-offset == 0) break;
        if (start+offset == rows) break;
    }
    return diff == 1;
}


int get_smudge_v(
    char layout[MAX_ROWS][MAX_COLS],
    int rows
) {
    int c = 1;
    while (1) {
        if (check_smudge_v(layout, rows, c)) return c;
        c++;
        if (layout[0][c] == 0) break;
    }
    return -1;
}

int get_smudge_h(
    char layout[MAX_ROWS][MAX_COLS],
    int rows
) {
    for (int r=1; r<rows; r++) {
        if (check_smudge_h(layout, rows, r)) return r;
    }
    return -1;
}

void second(FILE* textfile) {
    char line[MAX_COLS];
    long sum = 0;

    while (1) {
        char layout[MAX_ROWS][MAX_COLS] = {0};

        int rows = 0;
        int cols = 0;

        while (fgets(line, MAX_COLS+1, textfile)) {
            int len = strlen(line);
            if (len<2) break;
            strcpy(layout[rows], line);
            layout[rows][len-1] = 0;

            rows++;
        }

        int h = get_smudge_h(layout, rows); 
        int v = get_smudge_v(layout, rows); 
        printf("H: %d V: %d\n", h, v);

        if (v!=-1) sum += v;
        if (h!=-1) sum += 100 * h;

        char *has_next = fgets(line, MAX_COLS+1, textfile);
        if (has_next) {
            int rewind = strlen(line);
            fseek(textfile, -rewind, SEEK_CUR);
        } else {
            break;
        }
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