#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 255
#define MAX_GALAXIES 512

void first(FILE*);
// void second(FILE*);

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    first(textfile);
    // rewind(textfile);
    // second(textfile);
    return 0;
}

int cmp_x(const void *a, const void *b) {
    return ((int*)a)[0] - ((int*)b)[0];
}

int manhattan(int ax, int ay, int bx, int by) {
    return abs(bx-ax) + abs(by-ay);
}

void first(FILE* textfile) {
    char line[LINE_LENGTH];
    int galaxies_count = 0;
    int galaxies[MAX_GALAXIES][2];

    int row = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int has_galaxy = 0;
        for (int i=0; i<LINE_LENGTH; i++) {
            char c = line[i];
            if (c == '\n') {
                if (!has_galaxy) row++;
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

    qsort(galaxies, galaxies_count, 2 * sizeof(int), cmp_x);

    int acc = 0;
    int prev = 0;
    for (int i=0; i<galaxies_count; i++) {
        int d = galaxies[i][0] - prev;
        if (d>0) d--;
        acc += d;
        prev = galaxies[i][0];
        galaxies[i][0] += acc;
    }

    long sum = 0;
    for (int i=0; i<galaxies_count; i++) {
        for (int j=0; j<galaxies_count; j++) {
            if (i==j) continue;
            sum += (long)manhattan(galaxies[i][0], galaxies[i][1], galaxies[j][0], galaxies[j][1]);
        }
    }

    // for (int i=0; i<galaxies_count; i++) printf("G: %d %d; ", galaxies[i][0], galaxies[i][1]);
    printf("First: %ld\n", sum / 2);
}