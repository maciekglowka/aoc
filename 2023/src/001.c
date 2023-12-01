#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 255

int main(int argc, char *argv[]) {
    FILE *textfile;
    char line[LINE_LENGTH];

    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;
    int sum = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int i = 0;
        int digits[] = {-1, -1};
        while (1) {
            char c = line[i];
            i++;
            if (c == *"\n") break;
            if (c < 0x30 || c > 0x39) continue;
            if (digits[0] == -1) digits[0] = c - 0x30;
            digits[1] = c - 0x30;
        }
        sum += 10 * digits[0] + digits[1];
    }

    printf("%d\n", sum);

    return 0;
}