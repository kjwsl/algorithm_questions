/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ARR_SIZE(arr) (sizeof(arr) / sizeof(arr[0]))
#define ListNode_next(node) node = node->next

struct ListNode {
    int val;
    struct ListNode* next;
};

struct ListNode* ListNode_new(int val)
{
    struct ListNode* new_node = (struct ListNode*)malloc(sizeof(struct ListNode));
    memset(new_node, 0, sizeof(struct ListNode));

    new_node->val  = val;
    new_node->next = NULL;

    return new_node;
}

void ListNode_free(struct ListNode* node)
{
    struct ListNode* next = NULL;
    while (node->next != NULL) {
        next = node->next;
        free(node);
        node = next;
    }
    free(node);
}

struct ListNode* ListNode_push(struct ListNode* node, int val)
{
    struct ListNode* head     = node;
    struct ListNode* new_node = ListNode_new(val);

    if (node == NULL) {
        node       = new_node;
        node->next = NULL;
        return new_node;
    }

    while (node->next != NULL) {
        ListNode_next(node);
    }
    node->next = new_node;

    return head;
}

struct ListNode* ListNode_push_list(struct ListNode* node, int* vals, int size)
{
    for (int i = 0; i < size; i++) {
        node = ListNode_push(node, vals[i]);
    }

    return node;
}

struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2)
{
    struct ListNode* merged_list = NULL;
    // Until both of them are null
    while (list1 != NULL && list2 != NULL) {
        if (list1->val < list2->val) {
            merged_list = ListNode_push(merged_list, list1->val);
            ListNode_next(list1);
        } else {
            merged_list = ListNode_push(merged_list, list2->val);
            ListNode_next(list2);
        }
    }

    while (list1 != NULL) {
        merged_list = ListNode_push(merged_list, list1->val);
        ListNode_next(list1);
    }
    while (list2 != NULL) {
        merged_list = ListNode_push(merged_list, list2->val);
        ListNode_next(list2);
    }

    return merged_list;
}

void ListNode_print(struct ListNode* node)
{
    while (node->next != NULL) {
        printf("%d -> ", node->val);
        node = node->next;
    }
    printf("%d\n", node->val);
}

int main(int argc, char* argv[])
{
    struct ListNode* list1 = NULL;
    struct ListNode* list2 = NULL;

    int list1_vals[]       = { 1, 2, 4 };
    int list2_vals[]       = { 1, 3, 4 };

    list1                  = ListNode_push_list(list1, list1_vals, ARR_SIZE(list1_vals));
    list2                  = ListNode_push_list(list2, list2_vals, ARR_SIZE(list2_vals));

    ListNode_print(list1);
    ListNode_print(list2);

    struct ListNode* merged_list = mergeTwoLists(list1, list2);

    ListNode_print(merged_list);

    ListNode_free(list1);
    ListNode_free(list2);
    ListNode_free(merged_list);

    return 0;
}
