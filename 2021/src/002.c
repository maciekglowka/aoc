#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255

int main(int argc, char *argv[]) {
    FILE *textfile;
    char line[LINE_LENGTH];

    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;

    int x0 = 0;
    int y0 = 0;
    int x1 = 0;
    int y1 = 0;
    int aim = 0;

    while(fgets(line, LINE_LENGTH, textfile)) {
        char* cmd;
        char* dist_st;
        char *st = line;
        cmd = strsep(&st, " ");
        dist_st = strsep(&st, "\n");
        int dist = atoi(dist_st);

        if (*cmd == *"forward") {
            x0 += dist;
            x1 += dist;
            y1 += dist * aim;
        };
        if (*cmd == *"up") {
            y0 -= dist;
            aim -= dist;
        };
        if (*cmd == *"down") { 
            y0 += dist;
            aim += dist;
        }
    }

    printf("x0: %d\n", x0);
    printf("y0: %d\n", y0);
    printf("result0: %d\n", x0 * y0);

    printf("x1: %d\n", x1);
    printf("y1: %d\n", y1);
    printf("result1: %d\n", x1 * y1);

    return 0;
}