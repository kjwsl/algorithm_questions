/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INITIAL_SIZE 5

struct ListNode {
    int val;
    struct ListNode* next;
};

struct ListNode** history = NULL;
int history_size;
int history_capacity;

void history_new(int size)
{
    printf("history_new\n");
    history          = (struct ListNode**)malloc(size * sizeof(struct ListNode**));
    history_capacity = size;
    history_size     = 0;
    memset(history, 0, size * sizeof(struct ListNode*));
}

bool history_search(struct ListNode* target)
{
    printf("history_search\n");
    for (int i = 0; i < history_size; i++) {
        if (history[i] == target) {
            return true;
        }
    }
    return false;
}

void history_resize(int size)
{
    printf("history_resize\n");
    struct ListNode** new_history = (struct ListNode**)malloc(size * sizeof(struct ListNode**));
    memset(new_history, 0, size * sizeof(struct ListNode*));

    for (int i = 0; i < history_size; i++) {
        new_history[i] = history[i];
    }

    free(history);
    history          = new_history;
    history_capacity = size;
}

void history_push(struct ListNode* node)
{
    printf("history_push\n");
    if (history_size + 1 <= history_capacity) {
        history_resize(history_capacity * 2);
    }
    history[history_size++] = node;
}

bool hasCycle(struct ListNode* head)
{
    printf("hasCycle\n");
    history_new(INITIAL_SIZE);

    while (1) {
        struct ListNode* current = head;
        if (current == NULL) {
            return false;
        }
        history_push(current);
        head = head->next;
        if (history_search(head)) {
            return true;
        }
    }
}

void ListNode_free(struct ListNode* head)
{
    struct ListNode* current = head;
    while (current != NULL) {
        struct ListNode* next = current->next;
        free(current);
        current = next;
    }
}

struct ListNode* ListNode_new(int val)
{
    printf("ListNode_new: %d\n", val);
    struct ListNode* head = (struct ListNode*)malloc(sizeof(struct ListNode));
    head->val             = val;
    head->next            = NULL;
    return head;
}

struct ListNode* ListNode_push(struct ListNode* head, int val)
{
    printf("ListNode_push: %d\n", val);
    struct ListNode* current = head;
    while (current->next != NULL) {
        current = current->next;
    }
    current->next = ListNode_new(val);
    return current->next;
}

int main(int argc, char* argv[])
{
    struct ListNode* head = ListNode_new(3);
    struct ListNode* last = head;
    last                  = ListNode_push(last, 2);
    last                  = ListNode_push(last, 0);
    last                  = ListNode_push(last, -4);
    last->next            = head->next;

    bool res              = hasCycle(head);
    printf("%s\n", res ? "true" : "false");

    return EXIT_SUCCESS;
}
