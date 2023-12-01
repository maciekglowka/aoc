#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 255

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

void first(FILE* textfile) {
    int sum = 0;
    char line[LINE_LENGTH];

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

    printf("First: %d\n", sum);
}

void second(FILE* textfile) {
    int sum = 0;
    char line[LINE_LENGTH];
    char *tokens[] = {
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    };

    while (fgets(line, LINE_LENGTH, textfile)) {
        int i = 0;
        int digits[] = {-1, -1};
        char buf[LINE_LENGTH] = {0};
        int buf_i = 0;

        while (1) {
            int digit = -1;
            char c = line[i];
            i++;
            if (c == *"\n") break;

            if (c >= 0x30 && c <= 0x39) {
                digit = c - 0x30;
                for (int i=0; i<LINE_LENGTH; i++) {buf[i] = 0;}
                buf_i = 0;
            } else {
                buf[buf_i] = c;
                buf_i++;

                int pos = 0;
                
                for (int j=0; j<10; j++) {
                    char *p = strstr(buf, tokens[j]);
                    if (!p) continue;

                    if (digits[0] == -1) {
                        digit = j;
                        break;
                    }

                    // find last occurrence
                    while (1) {
                        char *n = strstr(p + 1, tokens[j]);
                        if (!n) break;
                        p = n;
                    }

                    if (p - buf < pos) continue;
                    pos = p - buf;
                    digit = j;
                }
            }
            if (digits[0] == -1) digits[0] = digit;
            if (digit != -1) digits[1] = digit;
        }
        sum += 10 * digits[0] + digits[1];
    }

    printf("Second: %d\n", sum);
}