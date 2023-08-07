#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 255

struct queue *new_queue();
void push(struct queue *q, int item);
int pop(struct queue *q);
int queue_sum(struct queue *q);

struct node {
    int value;
    struct node *next;
};

struct queue {
    struct node *head;
    struct node *tail;
    int len;
};

int main(int argc, char *argv[]) {
    FILE *textfile;
    char line[LINE_LENGTH];

    textfile = fopen(argv[1], "r");
    if (textfile == NULL) return 1;

    int count = 0;
    int prev = -1;

    struct queue *q = new_queue();
    
    while(fgets(line, LINE_LENGTH, textfile)) {
        int i = atoi(line);
        if (q->len == 3) pop(q);
        push(q, i);
        int s = queue_sum(q);
        if (s == -1) continue;
        if (prev != -1 && s - prev > 0) count++;
        prev = s;
    }
    printf("%d\n", count);
    fclose(textfile);
    return 0;
}

struct queue *new_queue() {
    struct queue *ans;
    ans = malloc(sizeof(struct queue));
    ans->head = NULL;
    ans->tail = NULL;
    ans->len = 0;
    return ans;
}

void push(struct queue *q, int item) {
    struct node *n;
    n = malloc(sizeof(struct node));
    n->value = item;
    n->next = NULL;
    if (q->head == NULL) {
        q->head = n;
    } else {
        q->tail->next = n;
    }
    q->tail = n;
    q->len++;
}

int pop(struct queue *q) {
    struct node *n;
    n = q->head;
    int val = n->value;
    q->head = n->next;
    free(n);
    q->len--;
    return val;
}

int queue_sum(struct queue *q) {
    if (q->len < 3) return -1;
    struct node *n = q->head;
    int s = n->value + n->next->value + n->next->next->value;
    return s;
}