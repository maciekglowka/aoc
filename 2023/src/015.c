#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TOKEN 255
#define BOXES 256
#define MAX_LENSES 255
#define MAX_LABEL 10

struct lens {
    char label[MAX_LABEL];
    int focal;
};

struct box {
    struct lens lenses[MAX_LENSES];
    int len;
};

int hasher(char* string) {
    int cur = 0;
    int i = 0;
    while (1) {
        if (string[i] == 0) break;
        cur += string[i];
        cur *= 17;
        cur = cur % 256;
        i++;
    }
    return cur;
}

void first(char* text) {
    char token[MAX_TOKEN];
    int token_i = 0;
    long cur = -1;
    long sum = 0;

    while (1) {
        cur++;
        if (text[cur] == ',' || text[cur] == '\n') {
            token[token_i] = 0;
            sum += hasher(token);
            token_i = 0;
            if (text[cur] == '\n') break;
            continue;
        }
        token[token_i] = text[cur];
        token_i++;
    }
    printf("First: %ld\n", sum);
}

void second(char* text) {
    char token[MAX_TOKEN];
    int token_i = 0;
    long cur = -1;
    long sum = 0;

    char label[MAX_LABEL] = {0};
    char op = ' ';

    struct box boxes [BOXES] = {0};

    while (1) {
        cur++;
        if (text[cur] == ',' || text[cur] == '\n') {
            token[token_i] = 0;
            int focal = atoi(token);
            int idx = hasher(label);

            token_i = 0;

            struct box* box = &boxes[idx];

            int lens_idx = -1;
            for (int i=0; i<box->len; i++) {
                if (strcmp(label, box->lenses[i].label)==0) {
                    lens_idx = i;
                    break;
                }
            }

            if (op == '=') {
                if (lens_idx != -1) {
                    box->lenses[lens_idx].focal = focal;
                } else {
                    strcpy(box->lenses[box->len].label, label);
                    box->lenses[box->len].focal = focal;
                    box->len++;
                }
            }
            if (op == '-' && lens_idx != -1) {
                int shift = box->len - lens_idx;
                memcpy(&box->lenses[lens_idx], &box->lenses[lens_idx+1], shift * sizeof(struct lens));
                box->len--;
            }

            if (text[cur] == '\n') break;
            continue;
        }
        if (text[cur] == '=' || text[cur] == '-') {
            for (int i=0; i<token_i; i++) {
                label[i] = token[i];
            }
            label[token_i] = 0;
            token_i = 0;
            op = text[cur];
            continue;
        }
        token[token_i] = text[cur];
        token_i++;
    }

    for (int b=0; b<BOXES; b++) {
        struct box* box = &boxes[b];
        for (int i=0; i<box->len; i++) {
            sum += (b+1)*(i+1)*box->lenses[i].focal;
        }
    }

    printf("Second: %ld\n", sum);
}

int main(int argc, char *argv[]) {
    FILE *textfile;
    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;

    fseek(textfile, 0, SEEK_END);
    long size = ftell(textfile);
    rewind(textfile);

    char *text = malloc(size + 1);
    fread(text, size, 1, textfile);

    first(text);
    second(text);
    return 0;
}