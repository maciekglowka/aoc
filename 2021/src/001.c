#include <stdio.h>
#include <stdlib.h>

#define DATA_PATH "../inputs/001.txt"
#define LINE_LENGTH 255

int main() {
    FILE *textfile;
    char line[LINE_LENGTH];

    textfile = fopen(DATA_PATH, "r");
    if (textfile == NULL) return 1;

    int count = 0;
    int prev = -1;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int i = atoi(line);
        if (prev != -1 && i - prev > 0) count ++;
        prev = i;
    }

    printf("\n%d\n\n", count);

    fclose(textfile);

    return 0;
}
