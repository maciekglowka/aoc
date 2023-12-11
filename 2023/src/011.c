#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 255
#define MAX_GALAXIES 512

void solve(FILE*, long);
// void second(FILE*);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    solve(textfile, 1);
    rewind(textfile);
    solve(textfile, 10);
    // second(textfile);
    return 0;
}

int cmp_x(const void *a, const void *b) {
    return ((long*)a)[0] - ((long*)b)[0];
}

long manhattan(long ax, long ay, long bx, long by) {
    return abs(bx-ax) + abs(by-ay);
}

void solve(FILE* textfile, long offset) {
    char line[LINE_LENGTH];
    int galaxies_count = 0;
    long galaxies[MAX_GALAXIES][2];

    long row = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int has_galaxy = 0;
        for (long i=0; i<LINE_LENGTH; i++) {
            char c = line[i];
            if (c == '\n') {
                if (!has_galaxy) row+=offset;
                row++;
                break;
            }
            if (c == '#') {
                has_galaxy++;
                galaxies[galaxies_count][0] = i;
                galaxies[galaxies_count][1] = row;
                galaxies_count++;
            }
        }
    }

    qsort(galaxies, galaxies_count, 2 * sizeof(long), cmp_x);

    long acc = 0;
    long prev = 0;
    for (int i=0; i<galaxies_count; i++) {
        long d = galaxies[i][0] - prev;
        if (d>0) d--;
        acc += d * offset;
        prev = galaxies[i][0];
        galaxies[i][0] += acc;
    }

    long long sum = 0;
    for (int i=0; i<galaxies_count; i++) {
        for (int j=0; j<galaxies_count; j++) {
            if (i==j) continue;
            sum += (long long)manhattan(galaxies[i][0], galaxies[i][1], galaxies[j][0], galaxies[j][1]);
        }
    }

    for (int i=0; i<galaxies_count; i++) printf("G: %ld %ld; ", galaxies[i][0], galaxies[i][1]);
    printf("Result: %lld\n", sum / 2);
}
