#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define LINE_LENGTH 255
#define ARR_LENGTH 100
#define MAX_CARDS 200

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
    char line[LINE_LENGTH];
    int sum = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int i = 10;
        int winners[ARR_LENGTH] = {0};
        int mode = 0;
        int count = 0;

        while (i<LINE_LENGTH) {
            if (line[i] == '|') {
                mode = 1;
                i-=1;
                continue;
            };
            char buf[3] = {line[i], line[i+1], 0};
            int value = atoi(buf);
            
            if (mode == 0) {
                winners[value] = 1;
            } else {
                if (winners[value] == 1) count ++;
            }

            if (line[i+2] == '\n') break;
            i+=3;
        }
        if (count > 0) sum += pow(2, count - 1);
    }
    printf("First: %d\n", sum);
}

void second(FILE* textfile) {
    char line[LINE_LENGTH];
    int sum = 0;
    int card_count[MAX_CARDS] = {0};
    int current_line = 0;

    while (fgets(line, LINE_LENGTH, textfile)) {
        int i = 10;
        int winners[ARR_LENGTH] = {0};
        int mode = 0;
        int count = 0;
        card_count[current_line] += 1;

        while (i<LINE_LENGTH) {
            if (line[i] == '|') {
                mode = 1;
                i-=1;
                continue;
            };
            char buf[3] = {line[i], line[i+1], 0};
            int value = atoi(buf);
            
            if (mode == 0) {
                winners[value] = 1;
            } else {
                if (winners[value] == 1) count++;
            }

            if (line[i+2] == '\n') break;
            i+=3;
        }

        sum += card_count[current_line];
        
        for (int j=0; j<card_count[current_line]; j++) {
            for (int i=1; i<=count; i++) {
                card_count[current_line + i] += 1;
            }
        }
        current_line++;
    }
    printf("Second: %d\n", sum);
}