#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255
#define MAX_DIM 255

// second part assumes that left hand is internal
// if not working try checking for the right hand

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

int read_layout(char* line, int row, char* data) {
    int start = -1;
    for (int i=0; i<MAX_DIM; i++) {
        char c = line[i];
        if (c == '\n') break;
        data[i] = c;
        if (c == 'S') start = i;
    }
    return start;
}

int can_pass(char layout[MAX_DIM][MAX_DIM], int max_dim, int row_s, int col_s, int row_t, int col_t) {
    if (row_t < 0 || col_t < 0 || row_t >= max_dim || col_t >= max_dim) return 0;
    char c = layout[row_s][col_s];
    if (row_s == row_t) {
        int dc = col_t - col_s;
        if (dc == 1) return c == '-' || c == 'L' || c == 'F';
        if (dc == -1) return c == '-' || c == '7' || c == 'J';
    }
    if (col_s == col_t) {
        int dr = row_t - row_s;
        if (dr == 1) return c == '|' || c == 'F' || c == '7';
        if (dr == -1) return c == '|' || c == 'L' || c == 'J';
    }
    return 0;
}

int max(int a, int b) {
    if (a > b) return a;
    return b;
}

void first(FILE* textfile) {
    char line[LINE_LENGTH];
    char layout[MAX_DIM][MAX_DIM];
    int rows = 0;
    int start_row;
    int start_col;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int s = read_layout(line, rows, layout[rows]);
        if (s!=-1) {
            start_col = s;
            start_row = rows;
        };
        rows++;
    }

    int queue[MAX_DIM * MAX_DIM][2];
    int field[MAX_DIM][MAX_DIM] = {0};

    field[start_row][start_col] = 1;

    int queue_len = 0;
    queue[0][0] = start_row;
    queue[0][1] = start_col;

    // initial queue
    for (int r=-1; r<=1; r++) {
            for (int c=-1; c<=1; c++) {
                int tr = start_row + r;
                int tc = start_col + c;
                if (can_pass(layout, rows, tr, tc, start_row, start_col)) {
                    queue[queue_len][0] = tr;
                    queue[queue_len][1] = tc;
                    field[tr][tc] = 2;
                    queue_len++;
                }
            }
    }

    while (queue_len > 0) {
        queue_len--;
        int row_c = queue[queue_len][0];
        int col_c = queue[queue_len][1];
        int dist = field[row_c][col_c];

        for (int r=-1; r<=1; r++) {
            for (int c=-1; c<=1; c++) {
                int tr = row_c + r;
                int tc = col_c + c;
                if (field[tr][tc] > 0 && field[tr][tc] < dist + 1) continue;
                if (can_pass(layout, rows, row_c, col_c, tr, tc)) {
                    queue[queue_len][0] = tr;
                    queue[queue_len][1] = tc;
                    field[tr][tc] = dist + 1;
                    queue_len++;
                }
            }
        }
    }

    int result = 0;
    for (int i=0; i<rows; i++) {
        for (int j=0; j<rows; j++) {
            result = max(result, field[i][j]);
        }
    }
    result--;

    printf("First: %d\n", result);
}

void mark_rl(char field[MAX_DIM][MAX_DIM], int max_rows, int max_cols, int row, int col, int dr, int dc) {
    int lr = row - dc;
    int lc = col + dr;
    int rr = row + dc;
    int rc = col - dr;

    // printf("lr; %d, lc: %d, rr: %d, rc: %d, fl: %c, fr: %c \n", lr, lc, rr, rc, field[lr][lc], field[rr][rc]);
    if (lr >= 0 && lc >= 0 && lr < max_rows && lc < max_cols) if (field[lr][lc] != '.') field[lr][lc] = 'L';
    if (rr >= 0 && rc >= 0 && rr < max_rows && rc < max_cols) if (field[rr][rc] != '.') field[rr][rc] = 'R';
}

void flood_fill(char field[MAX_DIM][MAX_DIM], int max_rows, int max_cols, int row, int col, char sign) {
    int directions[4][2]  = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};

    int queue[MAX_DIM * MAX_DIM][2];
    int queue_len = 1;
    queue[0][0] = row;
    queue[0][1] = col;

    while (queue_len > 0) {
        int cr = queue[0][0];
        int cc = queue[0][1];
        queue_len--;

        for (int i=0; i<4; i++) {
            int r = cr + directions[i][0];
            int c = cc + directions[i][1];
            if (r >= 0 && c >= 0 && r < max_rows && c < max_cols) if (field[r][c] == 0) {
                queue[queue_len][0] = r;
                queue[queue_len][1] = c;
                field[r][c] = sign;

                queue_len++;
            }
        }
    }
}

void fill_every(char field[MAX_DIM][MAX_DIM], int max_rows, int max_cols, char sign) {
    for (int r=0; r<max_rows; r++) {
        for (int c=0; c<max_cols; c++) {
            if (field[r][c] != sign) continue;
            flood_fill(field, max_rows, max_cols, r, c, sign);
        }
    }
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];
    char layout[MAX_DIM][MAX_DIM];
    int rows = 0;
    int cols = 0;
    int start_row;
    int start_col;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int s = read_layout(line, rows, layout[rows]);
        if (s!=-1) {
            start_col = s;
            start_row = rows;
        };
        cols = max(cols, strlen(line));
        rows++;
    }
    cols ++;

    int dr;
    int dc;
    int directions[4][2]  = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};
    for (int i=0; i<4; i++) {
        if (!can_pass(layout, rows, start_row + directions[i][0], start_col + directions[i][1], start_row, start_col)) continue;
        dr = directions[i][0];
        dc = directions[i][1];
        break;
    }

    char field[MAX_DIM][MAX_DIM] = {0};
    int cr = start_row;
    int cc = start_col;

    while (1) {
        field[cr][cc] = '.';

        cr += dr;
        cc += dc;

        mark_rl(field, rows, cols, cr, cc, dr, dc);

        char c = layout[cr][cc];

        if (c == 'L' && dc == -1) { dr = -1; dc = 0; }
        if (c == 'L' && dr == 1) { dr = 0; dc = 1; }

        if (c == 'J' && dc == 1) { dr = -1; dc = 0; }
        if (c == 'J' && dr == 1) { dr = 0; dc = -1; }

        if (c == '7' && dc == 1) { dr = 1; dc = 0; }
        if (c == '7' && dr == -1) { dr = 0; dc = -1; }

        if (c == 'F' && dc == -1) { dr = 1; dc = 0; }
        if (c == 'F' && dr == -1) { dr = 0; dc = 1; }

        mark_rl(field, rows, cols, cr, cc, dr, dc);

        if (cr == start_row && cc == start_col) break;
    }

    fill_every(field, rows, cols, 'L');

    for (int i=0; i<rows; i++) {
        for (int j=0; j<cols; j++) {
            printf("%c", (field[i][j] > 0) ? field[i][j] : ' ');
        }
        printf("\n");
    }

    int result = 0;

    for (int r=0; r<rows; r++) {
        for (int c=0; c<cols; c++) {
            if (field[r][c] == 'L') result++;
        }
    }

    printf("Second: %d\n", result);
}