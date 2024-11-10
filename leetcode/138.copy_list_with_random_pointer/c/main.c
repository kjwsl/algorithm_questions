/**
 * Definition for a Node.
 * struct Node {
 *     int val;
 *     struct Node *next;
 *     struct Node *random;
 * };
 */

/// https://leetcode.com/problems/copy-list-with-random-pointer/?envType=study-plan-v2&envId=top-interview-150

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INITIAL_MEMO_SIZE 1000
struct Node {
    int val;
    struct Node* next;
    struct Node* random;
};

struct NodeMap {
    struct Node* old_node;
    struct Node* new_node;
};

struct Node*
Node_new()
{
    struct Node* new_node = (struct Node*)malloc(sizeof(struct Node));
    memset(new_node, 0, sizeof(struct Node));

    return new_node;
}

struct NodeMap* memo = NULL;
int memo_size        = 0;
int memo_capacity    = INITIAL_MEMO_SIZE;

struct Node* NodeMap_find(struct Node* node)
{
    for (int i = 0; i < memo_size; i++) {
        if (memo[i].old_node == node) {
            return memo[i].new_node;
        }
    }

    return NULL;
}

void NodeMap_free()
{
    free(memo);
    memo          = NULL;
    memo_size     = 0;
    memo_capacity = INITIAL_MEMO_SIZE;
}

struct Node* NodeMap_add(struct Node* old_node, struct Node* new_node)
{
    if (memo_size >= memo_capacity) {
        memo_capacity *= 2;
        memo = (struct NodeMap*)realloc(memo, sizeof(struct NodeMap) * memo_capacity);
    }

    memo[memo_size].old_node = old_node;
    memo[memo_size].new_node = new_node;
    memo_size++;

    return new_node;
}

struct Node* Node_deep_copy(struct Node* node)
{
    if (node == NULL) {
        return NULL;
    }

    if (memo == NULL) {
        memo = (struct NodeMap*)malloc(sizeof(struct NodeMap) * INITIAL_MEMO_SIZE);
        memset(memo, 0, sizeof(struct NodeMap) * INITIAL_MEMO_SIZE);
    }

    struct Node* found_node = NodeMap_find(node);
    if (found_node != NULL) {
        return found_node;
    }

    struct Node* new_node = Node_new();
    NodeMap_add(node, new_node);
    new_node->val    = node->val;
    new_node->next   = Node_deep_copy(node->next);
    new_node->random = Node_deep_copy(node->random);

    return new_node;
}

struct Node* copyRandomList(struct Node* head)
{
    if (head == NULL) {
        return NULL;
    }

    struct Node* result = Node_deep_copy(head);
    NodeMap_free();
    return result;
}

int main(int argc, char* argv[])
{
    return EXIT_SUCCESS;
}
