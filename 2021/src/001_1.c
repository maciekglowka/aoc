#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 255

int main(int argc, char *argv[]) {
    FILE *textfile;
    char line[LINE_LENGTH];

    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;

    int count = 0;
    int prev = -1;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int i = atoi(line);
        if (prev != -1 && i - prev > 0) count ++;
        prev = i;
    }

    printf("%d\n", count);

    fclose(textfile);

    return 0;
}
